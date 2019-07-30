FROM rust:latest

RUN rustup update nightly;
RUN rustup default nightly;

RUN apt-get update
RUN apt-get install libssl-dev pkg-config -y

WORKDIR /usr/src/lunchtimer-hk

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN chmod +x /usr/src/lunchtimer-hk/target/release/lunchtime

RUN touch /usr/log.txt

CMD "tail -f > /usr/log.txt"

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

#FROM alpine:latest
#
#RUN addgroup -g 1000 lunchtimer-hk
#
#RUN adduser -D -s /bin/sh -u 1000 -G lunchtimer-hk lunchtimer-hk
#
#WORKDIR /home/lunchtimer-hk/bin/
#
#COPY --from=cargo-build /usr/src/lunchtimer-hk/target/x86_64-unknown-linux-musl/release/lunchtimer-hk .
#
#RUN chown lunchtimer-hk:lunchtimer-hk lunchtimer-hk
#
#USER lunchtimer-hk
#
#CMD ["./lunchtimer-hk"]