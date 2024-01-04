# zero2prod

Web application for sending newsletter to subscribers.
Based on the book "Zero To Production In Rust" by Luca Palmieri.
All credits to owner's repository [zero-to-production](https://github.com/LukeMathWalker/zero-to-production)

# Setup

Run the following script to setup and run PostgreSQL docker image.

```shell
./scripts/init_db.sh
```

Run the following script to setup and run Redis docker image.

```shell
./scripts/init_redis.sh
```

Run the following to startup the application:

```shell
cargo run
```