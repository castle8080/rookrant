FROM rust:1.76.0 as builder
WORKDIR /apps/rookrant_serv
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.* ./

# Compile a dummy source file to cache compiled deps
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl

COPY src src
COPY scripts scripts
RUN chmod a+x scripts/*.sh

# Update timestamp to force recompile.
RUN touch src/main.rs

RUN ./scripts/build-app.sh

FROM alpine:3.19.1
WORKDIR /apps/rookrant_serv
COPY --from=builder /apps/rookrant_serv/bin bin
COPY www www
COPY etc etc
COPY Cargo.* ./
COPY src src
COPY scripts scripts
EXPOSE 3000
CMD ["scripts/run-app.sh", "--tls=false", "--listen=0.0.0.0:3000"]