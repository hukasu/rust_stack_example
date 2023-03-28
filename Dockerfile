FROM rust:alpine as setup

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV OPENSSL_LIB_DIR=/usr/lib/
ENV OPENSSL_INCLUDE_DIR=/usr/include
ENV OPENSSL_STATIC=yes

WORKDIR /usr/financial/src

RUN apk update && apk add musl-dev openssl-dev openssl-libs-static ca-certificates-bundle

RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

COPY --from=setup /etc/ssl1.1/certs /etc/ssl/certs
COPY --from=setup /usr/financial/src/schema.sql /var/app/schema.sql
COPY --from=setup /usr/financial/src/target/x86_64-unknown-linux-musl/release/rust_stack_example /usr/local/bin/financial_data

EXPOSE 8000

ENTRYPOINT [ "financial_data" ]