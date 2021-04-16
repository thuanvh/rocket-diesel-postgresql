FROM rustlang/rust:nightly as builder

RUN mkdir /build
WORKDIR /build

COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
COPY ./.env ./.env
RUN ls -lR
# RUN rm ./target/debug/deps/license-data-manager*
RUN cargo build
RUN ls /build/target/debug/

FROM buildpack-deps:stretch

COPY --from=builder /build/target/debug/license_data_manager /app/

CMD ["/app/license_data_manager"]
