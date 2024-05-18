FROM debian:stable-slim

RUN apt-get update
RUN apt-get install -y curl gcc pkg-config libssl-dev
# Installing Rust through Rustup because Debian's version is too old
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable

WORKDIR /build
COPY . .
RUN cargo build --release

WORKDIR /app
RUN cp /build/target/release/nexus ./
RUN chmod +x ./nexus

RUN rm -rf /build
RUN rustup self uninstall -y
RUN apt-get remove -y curl gcc pkg-config libssl-dev
RUN apt-get autoremove -y


CMD ["/app/nexus"]