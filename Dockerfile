# FROM rust:1.50-prebuilt as builder
FROM rustdocker/rustfmt_clippy:nightly AS builder1
#FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04 AS builder1

USER root
WORKDIR /root
RUN mkdir cloud-sealer
COPY ./params/config .cargo/config
COPY . /root/cloud-sealer
#RUN apt-get update && apt-get install -y git

RUN git clone -b 2080ti https://hub.fastgit.org/EntySquare/entysnark.git
WORKDIR /root/entysnark
RUN cargo build

WORKDIR /root
RUN git clone https://hub.fastgit.org/EntySquare/filecoin-proof-debug.git
WORKDIR /root/filecoin-proof-debug
RUN cargo build

WORKDIR /root/cloud-sealer
RUN apt install mesa-opencl-icd ocl-icd-opencl-dev gcc git bzr jq pkg-config curl clang build-essential hwloc libhwloc-dev wget -y && apt upgrade -y
RUN cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu

FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04 AS builder2
ENV GOPROXY "https://goproxy.cn"
ENV GO111MODULE on
USER root
WORKDIR /root
RUN git clone https://hub.fastgit.org/EntySquare/cloud-sealer-sidecar.git
WORKDIR /root/cloud-sealer-sidecar
RUN go mod download
RUN apt-get update && apt-get install -y libhwloc-dev && go build

FROM nvidia/opencl:runtime-ubuntu18.04
WORKDIR /root
RUN apt-get update && apt-get install -y hwloc
COPY --from=builder1 /root/cloud-sealer/cloud-sealer .
#COPY --from=cloud-sealer-sidecar /root/cloud-sealer-sidecar/cloud-sealer-sidecar .
COPY --from=builder2 /root/cloud-sealer-sidecar/cloud-sealer-sidecar .

EXPOSE 7788
#ENTRYPOINT ["/$APP"]