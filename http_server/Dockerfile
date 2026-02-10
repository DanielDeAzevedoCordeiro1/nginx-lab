FROM clux/muslrust:stable AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM scratch
WORKDIR /instance
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/http_server /usr/local/bin/http_server
EXPOSE 3000
CMD ["/usr/local/bin/http_server"]