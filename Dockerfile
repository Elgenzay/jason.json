FROM rust:1.75-alpine
WORKDIR /usr/src/jason_json
ENV ROCKET_PROFILE=production
RUN apk update && apk add --no-cache build-base

COPY Cargo.toml Rocket.toml Cargo.lock .env ./
COPY src ./src
COPY templates ./templates
COPY static ./static

RUN cargo install --path .
EXPOSE 80
CMD ["jason_json"]