FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    librocksdb-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/waterfalls

COPY waterfalls/Cargo.toml waterfalls/Cargo.lock ./
COPY waterfalls/src ./src
COPY waterfalls/benches ./benches

RUN cargo build --locked --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    librocksdb-dev \
    ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/waterfalls/target/release/waterfalls /usr/local/bin/waterfalls

CMD ["waterfalls"]
