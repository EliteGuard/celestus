# Builder Stage
FROM rust:1.71 AS base
ENV SQLX_OFFLINE=true

# Create a new Rust project
RUN USER=root cargo new --bin celestus
WORKDIR /celestus

# Copy and build dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy the source code and build the application
COPY ./src ./src



FROM base AS build-prod
RUN cargo build --release --locked
RUN rm -rf src/*.rs



# Production Stage
FROM debian:buster-slim AS prod

RUN apt-get update \
    && apt-get install -y extra-runtime-dependencies ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC
ENV APP_USER=celestus

RUN groupadd ${APP_USER} \
    && useradd -g ${APP_USER} ${APP_USER} \
    && mkdir -p /usr/src/app

COPY --from=build-prod /celestus/target/release/celestus /usr/src/app/celestus

RUN chown -R ${APP_USER}:${APP_USER} /usr/src/app

USER $APP_USER
WORKDIR /usr/src/app

ENTRYPOINT ["./celestus"]



#Dev stage
FROM base AS build-dev
RUN cargo install cargo-watch

FROM build-dev AS dev
RUN RUST_LOG=info cargo-watch -x run -i ./*.json