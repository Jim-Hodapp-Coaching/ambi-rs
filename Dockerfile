# FROM rust:latest as build
# COPY . /
# WORKDIR /usr/local/bin/ambi_rs

# RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \
#     cargo build --release

# FROM ubuntu:22.04 as runtime
# COPY --from=build /target/release/ambi-rs .
# COPY Rocket.toml .
# COPY static/* ./static/
# #ENV ROCKET_DATABASES={db={url="postgres://postgres:postgres@localhost:5432/ambi_rs_dev"}}
# RUN echo 'APT::Install-Suggests "0";' >> /etc/apt/apt.conf.d/00-docker
# RUN echo 'APT::Install-Recommends "0";' >> /etc/apt/apt.conf.d/00-docker
# RUN DEBIAN_FRONTEND=noninteractive \
#   apt-get update \
#   && apt-get install -y libpq-dev \
#   && apt-get install -y git \
#   && apt-get install -y cargo \
#   && rm -rf /var/lib/apt/lists/*
# # Needed to ensure Cargo can fetch its crates index via GIT
# ENV CARGO_NET_GIT_FETCH_WITH_CLI true
# #ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
# RUN cargo install diesel_cli --no-default-features --features postgres
# CMD ["bash -c 'diesel setup && ./ambi-rs'"]

# EXPOSE 8000

FROM rust

RUN DEBIAN_FRONTEND=noninteractive \
  apt update \
  && apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/local/bin/ambi_rs

COPY . .
#COPY Rocket.toml .
#COPY static/* ./static/

RUN cargo install --path .

CMD bash -c "diesel setup && ambi-rs"

EXPOSE 8000