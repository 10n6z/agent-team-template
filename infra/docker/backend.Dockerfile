# syntax=docker/dockerfile:1
# Build context: the backend/ service dir (see infra/compose.yaml).
FROM rust:1.100 AS build
WORKDIR /src
COPY . .
# Cache mounts keep the cargo registry and target dir between builds; the binary is copied out of the cache.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked && cp target/release/backend /backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home app
COPY --from=build /backend /usr/local/bin/backend
USER 10001
ENV PORT=8080
EXPOSE 8080
CMD ["backend"]
