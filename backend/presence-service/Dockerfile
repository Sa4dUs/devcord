FROM rust:1.87 AS builder

WORKDIR /usr/src/app

COPY ./presence-service/Cargo.toml ./presence-service/Cargo.lock ./
COPY ./presence-service/src ./src

ARG BUILD_MODE=release

RUN echo "Building in $BUILD_MODE mode"

RUN if [ "$BUILD_MODE" = "release" ]; then \
      cargo build --release; \
    else \
      cargo build; \
      mkdir -p target/release; \
      cp target/debug/presence-service target/release/presence-service; \
    fi

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser

COPY --from=builder /usr/src/app/target/release/presence-service /usr/local/bin/app

USER appuser

EXPOSE 3000

CMD ["app"]
