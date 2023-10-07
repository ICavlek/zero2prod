FROM rust:1.72.1
# Let's switch our working directory to `app` (equivalent to `cd app`)
WORKDIR /app
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]