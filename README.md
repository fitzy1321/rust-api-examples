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
## Build images
docker build -t actix-web:latest actix-web-api/.
docker build -t axum:latest axum-api/.
docker build -t rocket:latest rocket-api/.
```

```sh
## Run images with ports exposed
docker run -it --rm -p 8000:8000 rocket:latest
```

## Benchmarking

```sh
# Need wrk package
brew install wrk

# Run against apis
wrk -t4 -d30s --latecy http://127.0.0.1:8080
```

## New Idea

Build standard todo app crud api.

Standardize endpoints and I/O contracts for APIs.

Make Docker images for all APIs.

Deploy to k3p4 Raspberry Pi Cluster (k3s host).

Use Ansible to automate setup and deployments.

???

PROFIT! 🤣
