ARG RUST_VERSION=1.75
ARG IMAGE=rust:${RUST_VERSION}

FROM ${IMAGE} as build
WORKDIR /src
COPY . ./

ARG FEATURES

RUN cargo build --release --features "${FEATURES}" \
    && mv target/release/automod ./automod

FROM debian:bookworm-slim

RUN apt update && apt install -y ca-certificates libssl-dev

COPY --from=build /src/automod /usr/local/bin
ENTRYPOINT ["automod"]
