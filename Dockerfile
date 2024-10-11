FROM rust:1.81 as builder
WORKDIR /usr/src/collapse-expand-fasta
COPY . .
RUN cargo install --path .

FROM debian:bookworm
RUN apt-get update && apt-get install -y procps && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/collapse-expand-fasta /usr/local/bin/collapse-expand-fasta

