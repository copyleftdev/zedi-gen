
FROM rust:1.70-slim AS builder


RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


WORKDIR /app


COPY . .


RUN cargo build --release


FROM debian:bullseye-slim


RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/zedi-gen /usr/local/bin/zedi-gen


WORKDIR /data


ENTRYPOINT ["zedi-gen"]


CMD ["--help"]
