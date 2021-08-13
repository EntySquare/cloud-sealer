FROM rust:latest as builder1
# Use prebuilt builder image
# FROM rust:1.50-prebuilt as builder
ARG APP=cloud-sealer

# New cargo project and copy Rust dependencies (and store as a separate Docker layer)
# NOTE: must call `cargo vendor` first and add `vendor` folder to git
RUN USER=root
WORKDIR /root
ADD . .

RUN mkdir -p .cargo
COPY config.toml .cargo/
COPY Cargo.toml Cargo.lock ./

RUN git clone https://github.com/EntySquare/entysnark.git ./
RUN git clone https://github.com/EntySquare/filecoin-proof-debug.git ./

RUN cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu

FROM ubuntu:18.04
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/$APP /$APP

EXPOSE 7788 4222 9999
ENTRYPOINT ["/$APP"]