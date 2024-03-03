FROM ubuntu:jammy
WORKDIR /apps/rookrant_serv
RUN apt-get update && apt-get install -y nano curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=$HOME/.cargo/bin:$PATH
CMD sh
