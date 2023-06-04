FROM lukemathwalker/cargo-chef:0.1.61-rust-latest as chef

WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .

# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release

FROM debian:bullseye-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir ./conf

COPY --from=builder /app/target/release/web3-trader web3-trader
COPY conf/* ./conf
ENV APP_ENVIRONMENT production

EXPOSE 8080

ENTRYPOINT ["./web3-trader"]