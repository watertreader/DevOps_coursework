# Rust as the base image
FROM rust:bookworm as build

# Create a new empty shell project
RUN USER=root cargo new --bin weather
WORKDIR /weather

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src
#COPY .env  .env

# Build for release.
RUN rm ./target/release/deps/weather*
RUN cargo build --release


# The final base image
#FROM debian:buster-slim
FROM debian:bookworm-slim

# Copy from the previous build
COPY --from=build /weather/target/release/weather /usr/src/weather
# COPY --from=build /holodeck/target/release/holodeck/target/x86_64-unknown-linux-musl/release/holodeck .
#COPY --from=build /weather/.env /usr/src/.env
# get SSL
RUN apt-get update && apt install -y openssl

EXPOSE 8080 4040 3000 80 443 22

# Run the binary
CMD ["/usr/src/weather"]