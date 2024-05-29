# try-sha1 contract
This is a simple contract for tests which calls sha1-calculate.
This is an API which is no longer supported.

## build
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="devcontract_cache_try_sha1",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.15.0 ./
```
