use redis::Commands;
use serde_json::json;
use serenity::{
    all::Presence,
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, Configuration, StandardFramework,
    },
    model::channel::Message,
    prelude::*,
};
use std::env;
#[macro_use]
extern crate rocket;

// This is done purely for compatibility with original lanyard endpoint
fn convert_json(input_json: &str) -> String {
    // Deserialize the input JSON
    let data: Presence = serde_json::from_str(input_json).unwrap();
    // Check if active_on_discord_mobile / active_on_discord_desktop
    let mobile = &data.clone().client_status.unwrap().mobile.ne(&None);
    let desktop = &data.clone().client_status.unwrap().desktop.ne(&None);
    let web = &data.clone().client_status.unwrap().web.ne(&None);
    // in activites, check if there's one with spotify
    let listening_to_spotify = data
        .activities
        .iter()
        .any(|activity| activity.name == Some("Spotify".to_string()).unwrap());
    let mut spotify_json: Option<serde_json::Value> = None;
    if listening_to_spotify {
        let spotify = Some(
            data.activities
                .iter()
                .find(|activity| activity.name == Some("Spotify".to_string()).unwrap())
                .unwrap()
                .clone(),
        );
        spotify_json = Option::from(json!({
            "timestamps":{
                "start":spotify.clone().unwrap().timestamps.unwrap().start,
                "end":spotify.clone().unwrap().timestamps.unwrap().end
            },
            "album": spotify.clone().unwrap().assets.unwrap().large_text.unwrap(),
            "track_id": spotify.clone().unwrap().sync_id.unwrap(),
            // Turn "large_image":"spotify:ab67616d0000b273b442345328808094f4c9728c" into https://i.scdn.co/image/ab67616d0000b273b442345328808094f4c9728c
            "album_art_url": spotify.clone().unwrap().assets.unwrap().large_image.unwrap().replace("spotify:","https://i.scdn.co/image/"),
            "artist": spotify.clone().unwrap().state,
            "song": spotify.clone().unwrap().details,
        }));
    }

    let out = json!({
        "success": true,
        "data": {
            "kv": {},
            "active_on_discord_mobile": mobile,
            "active_on_discord_desktop": desktop,
            "active_on_discord_web": web,
            "listening_to_spotify": listening_to_spotify,
            "discord_status": data.status,
            "activities": data.activities,
            "discord_user": {
                "id": data.user.id,
                "global_name": data.user.name,
                "bot": data.user.bot,
                "discriminator": data.user.discriminator,
                "avatar": data.user.avatar,
                "public_flags": data.user.public_flags,
            },
            "spotify": spotify_json.unwrap_or(json!(null))
        }
    });

    out.to_string()
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: serenity::all::Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn presence_update(&self, _: Context, p: Presence) {
        let json = convert_json(&serde_json::to_string(&p).unwrap());

        let mut redis = redis::Client::open(
            env::var("REDIS_LINK").expect("Expected a redis link in the environment"),
        )
        .unwrap()
        .get_connection()
        .unwrap();
        let _: () = redis.set(p.user.id.to_string(), json).unwrap();
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong!").await?;
    Ok(())
}

#[get("/users/<id>")]
fn presence(id: u64) -> String {
    let mut redis =
        redis::Client::open(env::var("REDIS_LINK").expect("Expected a redis link in the environment"))
            .unwrap()
            .get_connection()
            .unwrap();
    let json = redis.get(id.to_string()).unwrap();
    json
}

#[tokio::main]
async fn main() {
    // Start the Discord bot
    let discord_task = tokio::spawn(async {
        let framework = StandardFramework::new().group(&GENERAL_GROUP);
        framework.configure(Configuration::new().prefix("."));

        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let intents = GatewayIntents::non_privileged() | GatewayIntents::all();
        let mut client = Client::builder(token, intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Err creating client");

        if let Err(why) = client.start().await {
            println!("Err while running Discord bot: {:?}", why);
        }
    });

    // Start the Rocket server
    let rocket_task = tokio::spawn(async {
        rocket::build()
            .mount("/v1", routes![presence])
            .launch()
            .await
            .unwrap();
    });

    // Wait for both tasks to "complete"
    if let Err(e) = tokio::try_join!(discord_task, rocket_task) {
        println!("Error: {:?}", e);
    }
}
