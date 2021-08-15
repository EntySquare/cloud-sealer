# FROM rust:1.50-prebuilt as builder
FROM rust:latest AS builder1
#FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04 AS builder1

USER root
WORKDIR /root
RUN mkdir cloud-sealer
COPY . /root/cloud-sealer
RUN apt-get update && apt-get install -y git

RUN git clone -b 2080ti http://github.com/EntySquare/entysnark.git
WORKDIR /root/entysnark
RUN cargo check

WORKDIR /root
RUN git clone http://github.com/EntySquare/filecoin-proof-debug.git
WORKDIR /root/filecoin-proof-debug
RUN cargo check

WORKDIR /root/cloud-sealer
#RUN cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu
RUN cargo clean
RUN cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu

FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04 AS builder2
ENV GOPROXY "https://goproxy.cn"
ENV GO111MODULE on
USER root
WORKDIR /root
RUN git clone http://github.com/EntySquare/cloud-element.git
WORKDIR /root/cloud-element
RUN go mod download
RUN apt-get update && apt-get install -y libhwloc-dev && go build

FROM nvidia/opencl:runtime-ubuntu18.04
WORKDIR /root
RUN apt-get update && apt-get install -y hwloc
COPY --from=builder1 /root/cloud-sealer/cloud-sealer .
#COPY --from=cloud-element /root/cloud-element/cloud-element .
COPY --from=builder2 /root/cloud-element/cloud-element .

EXPOSE 4222 7788 9999
#ENTRYPOINT ["/$APP"]