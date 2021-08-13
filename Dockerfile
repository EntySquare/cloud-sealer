# FROM rust:1.50-prebuilt as builder
FROM rust:latest AS builder1

USER root
WORKDIR /root
ADD . .

#RUN mkdir -p .cargo
#COPY Config.toml .cargo/
COPY Cargo.toml Cargo.lock ./
COPY ./.gitconfig /root/.gitconfig

RUN git clone https://github.com/EntySquare/entysnark.git ./
RUN git clone https://github.com/EntySquare/filecoin-proof-debug.git ./

RUN cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu

FROM golang:1.15 AS builder2
ENV GOPROXY "https://goproxy.cn"
ENV GO111MODULE on
USER root
WORKDIR /root

ADD ./cloud-element .
COPY go.mod go.sum ./
RUN go mod download

FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04

RUN apt-get update
COPY --from=builder1 /usr/local/cargo/bin/cloud-sealer /cloud-sealer
COPY --from=builder2 /usr/local/cargo/bin/cloud-element /cloud-element

EXPOSE 4222 7788 9999
#ENTRYPOINT ["/$APP"]