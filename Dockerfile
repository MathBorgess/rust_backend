FROM rustlang/rust:nightly-alpine as builder
WORKDIR usr/app

COPY . .
RUN cargo build --release
# remove the dummy build.
RUN cargo clean -p api

# second stage.
FROM debian:buster-slim
WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/api .

CMD ["./api"]