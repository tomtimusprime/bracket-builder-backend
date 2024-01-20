# ----- Builder Image -----
FROM rust:slim as builder

WORKDIR /app

# Install deps
RUN cargo init --bin .
COPY Cargo.* ./
RUN cargo install --path . && rm src/*.rs

# Build the binary
COPY ./src ./src
RUN cargo build --release

# ----- Final Image -----
FROM bitnami/minideb:latest

WORKDIR /app

RUN apt-get update && apt-get install -y dumb-init \
  && rm -rf /var/lib/apt/lists/*

USER nobody

ENV RUST_LOG info
ENV PORT 7878

COPY --from=builder --chown=nobody:nogroup /app/target/release/* ./

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["./bracket-builder-backend"]
