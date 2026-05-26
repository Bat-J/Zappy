FROM ubuntu:26.04

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

RUN apt update && apt install -y python3-pip clang cmake make

RUN apt clean && rm -rf /var/lib/apt/lists/*

USER actions

WORKDIR /home/actions/pipeline
