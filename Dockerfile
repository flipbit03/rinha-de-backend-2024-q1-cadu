FROM rust:1.76.0 as build-env
WORKDIR /build
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu --release

FROM scratch
COPY --from=build-env /build/target/x86_64-unknown-linux-gnu/release/rinha-de-backend-2024-q1-cadu /
CMD ["./rinha-de-backend-2024-q1-cadu"]