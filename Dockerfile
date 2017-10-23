FROM liuchong/rustup:nightly-musl
ADD . /src
WORKDIR /src/
RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM alpine:latest
COPY --from=0 /src/target/x86_64-unknown-linux-musl/release/golinks-rs /bin/golinks-rs
RUN mkdir /linkdb
EXPOSE 8976
CMD ["/bin/golinks-rs","/linkdb"]
