# Coachee

Example app for tracking gym results.
The main goal of this project is however to try and learn rust programming language and give it some real-app feeling.

## Prerequisites
- [Rust language](https://www.rust-lang.org/learn/get-started)
- [Docker](https://docs.docker.com/engine/install/) with [docker-compose tool](https://docs.docker.com/compose/)
- [Diesel rust package with diesel_cli tool](https://diesel.rs/guides/getting-started)
    - setup the environment with `diesel setup` command
## Setup 
- `docker-compose up -d`
- `diesel migration run`

### Development
`cargo run`

### Build
`cargo build`

### Deploy as container
`docker-compose up -d coachee`