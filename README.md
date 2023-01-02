# Ambi

Ambi is a Rust-based web service that presents a basic HTML5 + JavaScript frontend and an HTMX frontend to display real time ambient room conditions like temperature, humidity, pressure, air quality, dust concentration, etc. It uses the Rocket web framework + Server Sent Events (SSE) to push updates to the client with no page refresh needed.

## Setup

1. First [install the DB ORM Diesel CLI](https://diesel.rs/guides/getting-started), then proceed to setting up the Postgresql DB


Setting up the Postgresql DB:
```sh
diesel setup
```

Now run the DB migrations to complete prepping the DB:
```sh
diesel migration run
```

## Running

To run:
```sh
ROCKET_DATABASES='{db={url="postgres://username:passwd@localhost:5432/ambi_rs_dev"}}' cargo run
```
