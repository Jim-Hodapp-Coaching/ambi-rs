# Ambi

Ambi is an Rust-based web service that presents a basic HTML5 + JavaScript frontend to display real time ambient room conditions
like temperature, humidity, pressure, air quality, dust concentration, etc. It uses the Rocket web framework + Server Sent Events (SSE)
to push updates to the client with no page refresh needed.

## Running

To run:
```
ROCKET_DATABASES='{db={url="postgres://username:passwd@localhost:5432/ambi_rs_dev"}}' cargo run
```
