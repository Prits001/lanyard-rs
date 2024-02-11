FROM ubuntu:latest


RUN apt-get update && apt-get upgrade -y
RUN apt-get install libssl-dev
RUN apt-get install -y -q build-essential curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY Cargo.toml Cargo.lock Rocket.toml ./
RUN mkdir src
COPY src ./src

RUN cargo build --release

EXPOSE 5050

# Command to run the application
CMD ["/app/target/release/lanyard-rust"]

