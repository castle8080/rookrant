# Stage-1 Setup a container which builds the app.
# 2 stages are used because the dev tools aren't needed for deployment.
FROM ubuntu:jammy as builder
WORKDIR /apps/rookrant_serv

# Setup packages needed to build rust apps.
RUN apt-get update && apt-get install -y nano curl build-essential libssl-dev pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

# Compile a dummy source file to cache compiled deps
COPY Cargo.* ./
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release

COPY src src
COPY scripts scripts
RUN chmod a+x scripts/*.sh

# Update timestamp to force recompile.
RUN touch src/main.rs

RUN ./scripts/build-app.sh

# Stage-2 Create the image with required binaries.
FROM ubuntu:jammy
WORKDIR /apps/rookrant_serv
COPY --from=builder /apps/rookrant_serv/bin/ bin/
COPY www www
COPY etc etc
COPY Cargo.* ./
COPY src src
COPY scripts scripts
EXPOSE 3000
CMD ["scripts/run-app.sh", "--tls=false", "--listen=0.0.0.0:3000"]