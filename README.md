# rust-restful-api

## Deploy

Build app
```
docker run --rm \
    -v cargo-cache:/root/.cargo \
    -v $$PWD:/volume \
    -w /volume \
    -it clux/muslrust:nightly \
    cargo build --release
```
move to current directory

`mv target/x86_64-unknown-linux-musl/release/rust_restful_api .`

and run `docker-compose up -d`