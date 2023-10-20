FROM rust:1.67 as build

RUN USER=root cargo new --bin textbin-axum

WORKDIR /textbin-axum
COPY . .

# build migration
RUN cd migration
RUN cargo build --release

# build
RUN cargo build --release

# setup rocky image
FROM rockylinux:9-minimal

# install deps
RUN microdnf -y install compat-openssl11

EXPOSE 8000

ENV DATABASE_URL \
    GRC_SECRET \
    RUST_LOG="DEBUG"

# copy build artifact
COPY --from=build /textbin-axum/target/release/textbin-axum .
COPY --from=build /textbin-axum/migration/target/release/migration .

# run
CMD ["./textbin-axum"]