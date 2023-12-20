# Builder Stage
FROM rust:1.71 AS base
ENV SQLX_OFFLINE=true

# Create a new Rust project
RUN USER=root cargo new --bin celestus
WORKDIR /celestus

# Copy and build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --locked
RUN rm src/*.rs

# Copy the source code and build the application
COPY . .

FROM base AS build-prod
RUN cargo build --release --locked



# Production Stage
FROM debian:buster-slim AS prod
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y extra-runtime-dependencies ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC
ENV APP_USER=celestus

RUN groupadd ${APP_USER} \
    && useradd -g ${APP_USER} ${APP_USER} \
    && mkdir -p ${APP}

COPY --from=build-prod /celestus/target/release/celestus ${APP}/celestus

RUN chown -R ${APP_USER}:${APP_USER} ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./celestus"]



#Dev stage
FROM base AS dev


RUN RUST_LOG=info cargo-watch -x run -i ./*.json