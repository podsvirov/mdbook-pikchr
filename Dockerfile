FROM alpine:latest as builder

COPY . /opt/src/mdbook-pikchr
WORKDIR /opt/src/mdbook-pikchr

RUN apk --no-cache add cargo

RUN cargo install mdbook@0.4.32
RUN cargo install --path . --locked

FROM alpine:latest

COPY --from=builder /root/.cargo/bin/mdbook /usr/local/bin/mdbook
COPY --from=builder /root/.cargo/bin/mdbook-pikchr /usr/local/bin/mdbook-pikchr

RUN apk --no-cache add libgcc
