# Shortener

Shortener is a simple application that shortens URLs into something easier to share, and when requested with a short url as a `/path`, will redirect to the corresponding URL.

It is written in Rust, using the Rocket web framework and Aerospike as a persistance layer.

***Table of Contents***

* [Development Environment](#up-and-away-locally)
* [Configuration](#configuration)
* [Endpoints](#endpoints)

## Up and away (locally)

To create a **development** environment, a local Rust toolchain is necessary. [`rustup`](https://rustup.rs/) is recommended.

This repository provides a Docker Compose file that simplifies setting up an environment, but it's not the only way to set it up.

### Requirements (for when **not** using Docker Compose):

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
| 1 | <pre>~ podman-compose up aerospike -d</pre> Or <pre>~ docker compose up aerospike -d</pre> | Start Aerospike in daemon mode (using Docker Compose) |
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

## Endpoints

### `/health`

***`GET /health`***

An overview of the health of the system

---

### `/url`

***`POST /url`***

Creates a new url or replies with an existing one

***Arguments***
| Argument | Description |
|---|---
| <sub><sup>required</sup></sub><br>`"url"` | URL to generate a hash for

***Example***
```json
{
  "url": "https://tanjarinne.myportfolio.com/work"
}
```

| HTTP Response | Example | Description |
|---|---|---
| `200 OK` | <pre>{<br>  "url": "URL TO SHORTEN",<br>  "shorten_url": "HASH"<br>}</pre> | Hash for long UL has been created |
| `500 Internal Server Error` | <pre>{<br>  "message": "Server error: Namespace not found"<br>}</pre> | Error messages from Aerospike or Rocket

---

### `/<hash>`

***`GET /<hash>`***

Redirects to the long url stored in Aerospike for that hash or 404

***Arguments***
| Argument | Description |
|---|---
| <sub><sup>required</sup></sub><br>`"hash"` | The hash obtained when calling `POST /url` sent as the first segment of the URL

## Capacity testing

### Requirements
* K6

```shell
k6 run tests/capacity.js

          /\      |‾‾| /‾‾/   /‾‾/
     /\  /  \     |  |/  /   /  /
    /  \/    \    |     (   /   ‾‾\
   /          \   |  |\  \ |  (‾)  |
  / __________ \  |__| \__\ \_____/ .io

  execution: local
     script: test/capacity.js
     output: -

  scenarios: (100.00%) 1 scenario, 1000 max VUs, 33s max duration (incl. graceful stop):
           * default: Up to 1000 looping VUs for 3s over 3 stages (gracefulRampDown: 30s, gracefulStop: 30s)
running (33.0s), 0000/1000 VUs, 10088 complete and 697 interrupted iterations
default ✓ [======================================] 0345/1000 VUs  3s

     ✗ POST request is status 200
      ↳  93% — ✓ 10026 / ✗ 744
     ✗ URL redirect is as expected
      ↳  99% — ✓ 10026 / ✗ 62

     checks.........................: 96.13% ✓ 20052      ✗ 806
     data_received..................: 5.5 MB 168 kB/s
     data_sent......................: 2.5 MB 75 kB/s
     http_req_blocked...............: avg=2.14ms   min=0s       med=1µs     max=6.7s    p(90)=1µs     p(95)=2µs
     http_req_connecting............: avg=2.13ms   min=0s       med=0s      max=6.7s    p(90)=0s      p(95)=0s
     http_req_duration..............: avg=108.82ms min=0s       med=11.45ms max=30.52s  p(90)=43.89ms p(95)=53.9ms
       { expected_response:true }...: avg=20.26ms  min=378µs    med=12.52ms max=7.3s    p(90)=43.74ms p(95)=53.06ms
     http_req_failed................: 4.75%  ✓ 1002       ✗ 20052
     http_req_receiving.............: avg=27.76µs  min=0s       med=10µs    max=11.62ms p(90)=18µs    p(95)=29µs
     http_req_sending...............: avg=4.17µs   min=0s       med=3µs     max=2.39ms  p(90)=6µs     p(95)=10µs
     http_req_tls_handshaking.......: avg=0s       min=0s       med=0s      max=0s      p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=108.78ms min=0s       med=11.44ms max=30.52s  p(90)=43.8ms  p(95)=53.78ms
     http_reqs......................: 21054  637.47696/s
     iteration_duration.............: avg=192.08ms min=862.91µs med=30ms    max=30.79s  p(90)=84.39ms p(95)=98.02ms
     iterations.....................: 10088  305.446356/s
     vus............................: 703    min=18       max=949
     vus_max........................: 1000   min=1000     max=1000
```




# Author
Tanja Álvarez, 2023
