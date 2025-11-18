FROM rust:alpine AS builder

RUN apk --no-cache add musl-dev

RUN cargo install mdbook@0.5.0 --locked

COPY . /opt/src/mdbook-pikchr
WORKDIR /opt/src/mdbook-pikchr

RUN cargo install --path . --locked

FROM alpine:latest

RUN apk --no-cache add libgcc

COPY --from=builder /usr/local/cargo/bin/mdbook /usr/local/bin/mdbook
COPY --from=builder /usr/local/cargo/bin/mdbook-pikchr /usr/local/bin/mdbook-pikchr
