FROM rust:1.87 AS builder

WORKDIR /usr/src/app

COPY ./user-service/Cargo.toml ./user-service/Cargo.lock ./
COPY ./user-service/src ./src
COPY ./user-service/migrations ./migrations
COPY topic_structs ../topic_structs

ARG BUILD_MODE=release

RUN echo "Building in $BUILD_MODE mode"

RUN if [ "$BUILD_MODE" = "release" ]; then \
      cargo build --release; \
    else \
      cargo build; \
      mkdir -p target/release; \
      cp target/debug/user-service target/release/user-service; \
    fi

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser

COPY --from=builder /usr/src/app/target/release/user-service /usr/local/bin/app
COPY --from=builder /usr/src/app/migrations ./user-service/migrations

USER appuser

EXPOSE 3000

CMD ["app"]
