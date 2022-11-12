FROM rust:1.62.0

WORKDIR /usr/src/gh-visibility
COPY . .

RUN cargo install --path .

ENTRYPOINT ["gh-visibility"]
