FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin license-data-manager
WORKDIR /license-data-manager
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/license-data-manager*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /license-data-manager/target/debug/license-data-manager /app/

ENTRYPOINT ["/app/license-manager"]