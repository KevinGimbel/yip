FROM clux/muslrust:nightly as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/yip .
COPY LICENSE .
EXPOSE 8111
ENTRYPOINT [ "/yip" ]