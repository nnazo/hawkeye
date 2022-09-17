# BUILD CACHING
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
WORKDIR /usr/src/migration
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher ${CARGO_HOME} ${CARGO_HOME}
RUN cargo build --release

FROM rust:1.63.0-slim-bullseye as runtime
WORKDIR /usr/src
COPY --from=builder /usr/src/migration/target/release/migration /usr/local/bin/migration
ENV PATH ${CARGO_HOME}/bin:/usr/local/bin:${PATH}

CMD ["migration", "up"]