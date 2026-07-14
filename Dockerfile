ARG RUST_VERSION=1.97

FROM docker.io/library/rust:${RUST_VERSION}-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

# Running in a clean instance
FROM gcr.io/distroless/cc-debian13 as final
WORKDIR /app
COPY --from=build /app/target/release/filehost .
EXPOSE 42967
CMD ["./filehost"]
