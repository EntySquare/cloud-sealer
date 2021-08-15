# FROM rust:1.50-prebuilt as builder
#FROM rust:latest AS builder1
FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04 AS builder1

USER root
WORKDIR /root
RUN mkdir cloud-sealer
COPY . /root/cloud-sealer
RUN apt-get update && apt-get install -y git
#RUN mkdir -p .cargo
#COPY Config.toml .cargo/
#COPY Cargo.toml Cargo.lock ./

RUN git clone -b 2080ti https://github.com/EntySquare/entysnark.git
WORKDIR /root/entysnark
RUN cargo check

WORKDIR /root
RUN git clone https://github.com/EntySquare/filecoin-proof-debug.git
WORKDIR /root/filecoin-proof-debug
RUN cargo check

WORKDIR /root/cloud-sealer
#RUN cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu
RUN cargo clean
RUN cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu

#FROM golang:1.15 AS builder2
#ENV GOPROXY "https://goproxy.cn"
#ENV GO111MODULE on
#USER root
#WORKDIR /root
#
#ADD ./cloud-element .
#COPY go.mod go.sum ./
#RUN go mod download

#FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04
FROM nvidia/opencl:runtime-ubuntu18.04
RUN apt-get update && apt-get install -y hwloc
COPY --from=builder1 /usr/local/cargo/bin/cloud-sealer /cloud-sealer
#COPY --from=builder2 /usr/local/cargo/bin/cloud-element /cloud-element
COPY --from=cloud-element /usr/local/cargo/bin/cloud-element /cloud-element

EXPOSE 4222 7788 9999
#ENTRYPOINT ["/$APP"]