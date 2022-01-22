###########################################
## Builder (build binary)
###########################################
FROM lukemathwalker/cargo-chef:latest-rust-1.58.1-slim as chef
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already.
WORKDIR /app
FROM chef as planner
# Copy all files from our working environment to our Docker image 
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
# force sqlx to look at the saved metadata instead of trying to query a live database:
ENV SQLX_OFFLINE true
# Let's build our binary! 
RUN cargo build --release --bin rust2prod_api
###########################################
## Runtime (run the binary)
###########################################
# Runtime stage
FROM debian:bullseye-slim AS runtime
# Instead copy the compiled binary from the builder environment 
# to our runtime environment
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust2prod_api rust2prod_api
# We need the configuration file at runtime!
COPY configuration configuration
# Instruct binary in Docker image to use the production configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./rust2prod_api"]