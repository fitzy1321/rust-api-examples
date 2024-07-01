# Rust API Examples

Testing out various Rust API Frameworks

- actix-web
- axum
- rocket

requirements for apis

- json request/response schemas
- orm / sql / NoSql / DB ?
- OpenAPI Spec Generator + OpenAPI UI
- logging (preferrably struct logs)

## Todo

- concurrency / async implementations ? (tokio vs other setups?)
- llvm-cov for coverage
- docker ?
  - aws linux base image?
  - alpine image?
    - what is the smallest docker base image?

## Run Locally

Need cargo and/or docker

```sh
docker build -t app-name .
docker run -it --rm app-name
```

## Benchmarking

```sh
# Need wrk package
brew install wrk

# Run against apis
wrk -t4 -d30s --latecy http://127.0.0.1:8080
```
