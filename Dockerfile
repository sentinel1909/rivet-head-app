# builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 as chef

WORKDIR /app

RUN apt update && apt install lld clang -y

FROM chef as planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build -p rivet-head-app --release

# Runtime stage

FROM debian:bullseye-slim AS Runtime

WORKDIR /app

RUN apt-get update -y && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rivet-head-app rivet-head-app

COPY assets assets

ENTRYPOINT ["./rivet-head-app"]