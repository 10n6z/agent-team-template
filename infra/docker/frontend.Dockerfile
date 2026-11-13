# syntax=docker/dockerfile:1
# Build context: the frontend/ service dir (see infra/compose.yaml).
FROM rust:1.100 AS build
WORKDIR /src
COPY . .
# Cache mounts keep the cargo registry and target dir between builds; the binary is copied out of the cache.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked && cp target/release/frontend /frontend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home app
COPY --from=build /frontend /usr/local/bin/frontend
USER 10001
ENV PORT=3000
EXPOSE 3000
CMD ["frontend"]
