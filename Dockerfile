FROM clux/muslrust:nightly as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/your-ip .
COPY LICENSE .
EXPOSE 8111
ENTRYPOINT [ "/your-ip" ]