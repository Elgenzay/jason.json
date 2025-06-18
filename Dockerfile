FROM rust:latest
WORKDIR /usr/src/jason_json
ENV ROCKET_PROFILE=production
RUN apt-get update && apt-get install -y build-essential

COPY Cargo.toml Rocket.toml Cargo.lock .env ./
COPY src ./src
COPY templates ./templates
COPY static ./static

RUN cargo install --path .
EXPOSE 80
CMD ["jason_json"]
