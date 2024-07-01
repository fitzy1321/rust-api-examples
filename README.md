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

```dockerfile
FROM rust:1.79-slim-bookworm as base

RUN apt update && apt upgrade -y

FROM base as builder

EXPOSE 8080

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

CMD ["bash"]
```

```.dockerignore
target
.cargo
**/*.sh
**/*.tar.gz
**/*.git*
```

```sh
docker build -t app-name .
docker run -it --rm app-name
```
