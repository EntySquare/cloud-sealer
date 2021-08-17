# FROM rust:1.50-prebuilt as builder
#FROM rustdocker/rustfmt_clippy:nightly AS builder1
FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04 AS builder1

USER root
WORKDIR /root
RUN mkdir cloud-sealer
COPY ./params/config .cargo/config
COPY . /root/cloud-sealer
#RUN apt-get update && apt-get install -y git

RUN git clone -b 2080ti https://hub.fastgit.org/EntySquare/entysnark.git
#WORKDIR /root/entysnark
#RUN /root/.cargo/bin/cargo build

WORKDIR /root
RUN git clone https://hub.fastgit.org/EntySquare/filecoin-proof-debug.git
#WORKDIR /root/filecoin-proof-debug
#RUN /root/.cargo/bin/cargo build

WORKDIR /root/cloud-sealer
RUN apt update
RUN /root/.cargo/bin/rustup install nightly
RUN /root/.cargo/bin/rustc --version
RUN /root/.cargo/bin/rustup default nightly
#RUN /root/.cargo/bin/rustup update

RUN apt install openssl libssl-dev mesa-opencl-icd ocl-icd-opencl-dev gcc bzr jq pkg-config clang build-essential hwloc libhwloc-dev -y && apt upgrade -y
RUN /root/.cargo/bin/cargo build --release --no-default-features --features multicore-sdr --features pairing,gpu

FROM registry.cn-shanghai.aliyuncs.com/filtab/filecoin-ubuntu:nvidia-opencl-devel-ubuntu18.04 AS builder2
ENV GOPROXY "https://goproxy.cn"
ENV GO111MODULE on
USER root
WORKDIR /root
RUN git clone https://hub.fastgit.org/EntySquare/cloud-sealer-sidecar.git
WORKDIR /root/cloud-sealer-sidecar
RUN go mod tidy
RUN go build -o cloud-sealer-sidecar

FROM nvidia/opencl:runtime-ubuntu18.04
WORKDIR /root
RUN apt-get update && apt-get install -y hwloc
COPY --from=builder1 /root/cloud-sealer/target/release/cloud-sealer .
#COPY --from=cloud-sealer-sidecar /root/cloud-sealer-sidecar/cloud-sealer-sidecar .
COPY --from=builder2 /root/cloud-sealer-sidecar/cloud-sealer-sidecar .

#ENTRYPOINT ["/$APP"]