FROM rust:1.67 as build

RUN USER=root cargo new --bin textbin-axum
WORKDIR /textbin-axum

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migration ./migration

# build
RUN cargo build --release

# build migration
RUN cd migration
RUN cargo build --release

# run
FROM archlinux:base-devel

# install deps
RUN pacman -Syyu --noconfirm
RUN pacman -S openssl-1.1 --noconfirm

EXPOSE 8000

ENV DATABASE_URL \
    GRC_SECRET \
    RUST_LOG="DEBUG"

# copy build artifact
COPY --from=build /textbin-axum/target/release/textbin-axum .
COPY --from=build /textbin-axum/migration/target/release/migration .

CMD ["./textbin-axum"]