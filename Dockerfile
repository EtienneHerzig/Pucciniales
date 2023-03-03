FROM rust:latest

WORKDIR /app

COPY . .

RUN rustup default nightly

RUN cargo build --release --out-dir out -Z unstable-options

EXPOSE 443

CMD ["./out/pucciniales"]