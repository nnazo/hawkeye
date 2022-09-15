FROM rust:1.63.0 as base
WORKDIR /
RUN cargo install sea-orm-cli@0.9.2

# Build caching
FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build project
FROM rust:1.63.0 as builder
WORKDIR /usr/src/hawkeye
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher ${CARGO_HOME} ${CARGO_HOME}
RUN cargo build --release

FROM rust:1.63.0-slim-bullseye as runtime
WORKDIR /app

RUN apt update
RUN apt install -y chromium chromium-driver

COPY --from=base ${CARGO_HOME} ${CARGO_HOME}
COPY --from=builder /usr/src/hawkeye/target/release/hawkeye /usr/local/bin/hawkeye
COPY entrypoint.sh .
ENV PATH ${CARGO_HOME}/bin:/usr/local/bin:/app/target/release:${PATH}

ENTRYPOINT ["/app/entrypoint.sh"]