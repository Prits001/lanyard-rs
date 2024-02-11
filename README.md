
## This is a Rust rewrite of [Lanyard](https://github.com/Phineas/lanyard)
## <img src="https://github.com/Prits001/lanyard-rs/blob/3f5add219fe2d6be49bcb6f05f8d7d6a677df849/Icon.png" width="100"/> Expose your Discord presence and activities to a RESTful API ~~and WebSocket~~ in less than 10 seconds

Lanyard-rust is a service that makes it easy to export your live Discord presence to an API endpoint (`lanyard.devprits.ru/v1/users/:your_id`) and ~~to a WebSocket (see below)~~(to be implemented later) for you to use wherever you want.
You can use Lanyard-rust's API without deploying anything yourself - but if you want to self host it, you have the option to, though it'll require a tiny bit of configuration.

# Setup
Join [this Discord server](https://discord.gg/9yQSzXUpsB) and your presence will start showing up when you `GET lanyard.devprits.ru/v1/users/:your_id`. It's that easy.

## API Ouput Example
Here's an example json from my account:
```json
{
  "data": {
    "active_on_discord_desktop": true,
    "active_on_discord_mobile": false,
    "active_on_discord_web": false,
    "activities": [
      {
        "application_id": null,
        "assets": {
          "large_image": "spotify:ab67616d0000b2736f72d74746a93f1ce7014bc0",
          "large_text": "A MILLION WAYS TO MURDER (DELUXE)",
          "small_image": null,
          "small_text": null
        },
        "buttons": [],
        "created_at": 1707611255195,
        "details": "MURDER IN MY MIND",
        "emoji": null,
        "flags": 48,
        "instance": null,
        "name": "Spotify",
        "party": {
          "id": "spotify:478869660851372062",
          "size": null
        },
        "secrets": null,
        "session_id": "f085ce8a9e118e1bd34faabe299872a1",
        "state": "Kordhell",
        "sync_id": "46NH2gyAyKCG8Km5IB8FTh",
        "timestamps": {
          "end": 1707611397584,
          "start": 1707611252584
        },
        "type": 2,
        "url": null
      },
      {
        "application_id": "810516608442695700",
        "assets": {
          "large_image": null,
          "large_text": null,
          "small_image": "mp:external/Joitre7BBxO-F2IaS7R300AaAcixAvPu3WD1YchRgdc/https/raw.githubusercontent.com/LeonardSSH/vscord/main/assets/icons/vscode.png",
          "small_text": null
        },
        "buttons": [],
        "created_at": 1707610583961,
        "details": null,
        "emoji": null,
        "flags": 1,
        "instance": null,
        "name": "Visual Studio Code",
        "party": null,
        "secrets": null,
        "session_id": "f085ce8a9e118e1bd34faabe299872a1",
        "state": null,
        "sync_id": null,
        "timestamps": {
          "end": null,
          "start": 287454421000
        },
        "type": 0,
        "url": null
      }
    ],
    "discord_status": "online",
    "discord_user": {
      "avatar": "3b32267c473e552d2c44e36e6e98d5bb",
      "bot": false,
      "discriminator": null,
      "global_name": ".prits",
      "id": "478869660851372062",
      "public_flags": 4194368
    },
    "kv": {},
    "listening_to_spotify": true,
    "spotify": {
      "album": "A MILLION WAYS TO MURDER (DELUXE)",
      "album_art_url": "https://i.scdn.co/image/ab67616d0000b2736f72d74746a93f1ce7014bc0",
      "artist": "Kordhell",
      "song": "MURDER IN MY MIND",
      "timestamps": {
        "end": 1707611397584,
        "start": 1707611252584
      },
      "track_id": "46NH2gyAyKCG8Km5IB8FTh"
    }
  },
  "success": true
}
```
__There are minor differences to original Lanyard's output that will be patched soon.__

# Self-host with Docker
Build the Docker image by cloning this repo and running:
```bash
git clone https://github.com/Prits001/lanyard-rs
cd lanyard-rs
docker build -t pri1s/lanyard-rust .
```
OR Download from hub
```
docker pull pri1s/lanyard-rust:latest
```
Redis server required! Use this to deploy a container of it + create network with it.
```bash
docker network create lanyard
docker run -d --name lanyard-redis --network lanyard -v docker_mount_location_on_host:/data redis-alpine
```
And run the freshly baked image
```
docker run --network lanyard -d -e REDIS_LINK=redis://lanyard-redis:6379/ -e DISCORD_TOKEN=TOKEN_HERE -p 5050:5050 pri1s/lanyard-rust
```
Make sure you have a Discord bot with **__PRESENCE INTENT and SERVER MEMBERS INTENT__** turned **on**
