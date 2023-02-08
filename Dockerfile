FROM rust

RUN DEBIAN_FRONTEND=noninteractive \
  apt update \
  && apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/local/bin/ambi_rs

COPY . .

RUN cargo install --path .

CMD bash -c "diesel setup && ambi-rs"

ENV DATABASE_URL=postgres://postgres:postgres@db/ambi_rs_dev
ARG ROCKET_DATABASES

EXPOSE 8000