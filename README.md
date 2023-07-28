# Shortener

Shortener is a simple application that shortens URLs into something easier to share, and when requested with a short url as a `/path`, will redirect to the corresponding URL.

It is written in Rust, using the Rocket web framework and Aerospike as a persistance layer.

***Table of Contents***

* [Development Environment](#up-and-away-locally)
* [Configuration](#configuration)

## Up and away (locally)

To create a **development** environment, a local Rust toolchain is necessary. [`rustup`](https://rustup.rs/) is recommended.

This repository provides a [Compose](https://compose-spec.io/) file that simplifies setting up an environment, but it's not the only way to set it up.

### Requirements (for when **not** using a container):

- [Aerospike Community Edition](https://github.com/aerospike/aerospike-server/releases)
- The Rust language
- Podman or Docker
- `podman-compose` when using Podman

#### Optionally:
- [cargo-watch](https://crates.io/crates/cargo-watch)

### 1. the fastest way

The project will run with baseline configurations.

| Pros | Cons |
|---|---|
| One command is all you need | Need to manually restart/(re)build/(sometimes delete) the image every time a new code change is made |

| Step | Command & example output | Commentary |
|---|---|---|
| 1 | <pre>~ podman-compose up -d</pre> Or <pre>~ docker compose up -d</pre> | Start all the services in daemon mode

### 2. the most efficient way

Will be most comfortable to use it as a development environment.

| Pros | Cons |
|---|---|
| Quick and it automatically reloads on every save | Haven't found any really |

| Step | Command & example output | Commentary |
|---|---|---|
| 1 | <pre>~ podman-compose up aerospike -d</pre> Or <pre>~ docker compose up aerospike -d</pre> | Start Aerospike in daemon mode |
| 2 | <pre>~ cd app</pre> | Change directory to _the app_ |
| 3 | <pre>app~ cp Rocket-example.toml Rocket.toml<br>app~ vim Rocket.toml</pre> | [Configure the app](https://rocket.rs/v0.5-rc/guide/configuration/) |
| 4 | <pre>app~ cargo watch -q -c -w src/ -x run</pre>Or without `cargo watch`:<br><pre>app~ cargo run</pre> | Run Shortener |

## Configuration

***Application***

The application uses environment variables to configure.

| Variable | Default | Description
|---|---|---
| `AEROSPIKE_HOSTS` | `localhost:3000` | A comma-separated list of the Aerospike nodes to connect to

***Rocket***

| File | Description |
|---|---
| `app/Rocket-example.toml` | Example configuration for a local environment
| `app/Rocket-container.tml` | Configurations that the application runs with in a Container

***Aerospike***

| File | Description |
|---|---
| `aerospike/aerospike.conf` | Example configuration for a local environment


# Author
Tanja Álvarez, 2023
