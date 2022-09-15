FROM rust:1.63.0 as base
WORKDIR /
RUN cargo install sea-orm-cli@0.9.2

# BUILD CACHING
FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

# Build project
FROM rust:1.63.0 as builder
WORKDIR /usr/src/migration
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher ${CARGO_HOME} ${CARGO_HOME}
RUN cargo build

FROM rust:1.63.0-slim-bullseye as runtime
WORKDIR /usr/src
COPY --from=base ${CARGO_HOME} ${CARGO_HOME}
COPY --from=builder /usr/src/migration migration
ENV PATH ${CARGO_HOME}/bin:${PATH}

CMD ["sea-orm-cli", "migrate", "up"]