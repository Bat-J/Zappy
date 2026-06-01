FROM alpine:20260127

COPY usr /usr

RUN apk add curl tar

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

RUN apk add python3 py3-pip clang clang-extra-tools cmake make git nodejs

RUN apk cache clean && rm -rf /var/cache/apk/*

WORKDIR /home/actions
