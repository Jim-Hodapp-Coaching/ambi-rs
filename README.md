# Ambi

Ambi is a Rust-based web service that presents a basic HTML5 + JavaScript frontend and an HTMX frontend to display real time ambient room conditions like temperature, humidity, pressure, air quality, dust concentration, etc. It uses the Rocket web framework + Server Sent Events (SSE) to push updates to the client with no page refresh needed.

<img width="1455" alt="Screenshot 2023-01-02 at 22 03 59" src="https://user-images.githubusercontent.com/3219120/210298784-cd1479e5-ef1b-4e81-b213-22dba83ec6bb.png">

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

### Set Up Git Hooks

The Ambi repository makes use of several Git hooks to ensure that code quality standards are met and consistent. To automatically configure these hooks for your local workspace, you can run the following:

./scripts/create-git-hooks
This will create symlinks to the Git hooks, preserving any hooks that you may have already configured.

## Running

To run:
```sh
ROCKET_DATABASES='{db={url="postgres://username:passwd@localhost:5432/ambi_rs_dev"}}' cargo run
```
