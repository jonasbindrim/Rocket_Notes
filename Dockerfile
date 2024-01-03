FROM rust:latest as builder
WORKDIR /usr/src/rocket_notes
COPY . /usr/src/rocket_notes/
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/rocket_notes /usr/local/app/rocket_notes/rocket_notes
COPY ./Rocket.toml /usr/local/app/rocket_notes/Rocket.toml
WORKDIR /usr/local/app/rocket_notes
CMD ["./rocket_notes"]