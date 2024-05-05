# Okta OIDC Resource Server

This is a Rust based app.  It leverage these services (via Docker):
- [Valkey](https://valkey.io/) (open source Redis)  for in-memory (not persisted) session management,
- [Jaeger](https://www.jaegertracing.io/) (OpenTelemetry) for logging/tracing.  

## Setup
There are a few variables that you should set.  Up to you how you manage your environment. 
I suggest using an `.env` file.  All the vars have defaults (shown) with the exception of 
`OIDC_CLIENT_ID` and `OIDC_CLIENT_SECRET`.  So if the defaults work for you, that's all you need to set

In the repo root (same folder as `Cargo.toml`), create a `.env` file and add the following:
    ````bash
    SERVER_ADDR=127.0.0.1
    SERVER_PORT=3000
    SERVER_SESSION_TTL=1
    OIDC_CLIENT_ID=<YOUR Okta client_id>
    OIDC_CLIENT_SECRET=<YOUR OKTA client_secret>
    OTEL_URL=http://localhost:4317
    ````

The OIDC Resource server also has some configuration options.  Apologies for the redundancy.  I'm 
reading the entire file with [serde](https://docs.rs/serde) and [toml](https://docs.rs/toml). 

Again, these are all defaulted

In the repo root (same folder as `Cargo.toml`), create `oidc.toml` and add the following:
  ````toml
  server_address = "127.0.0.1"
  server_port = 3000
  domain = "dskyberg"
  app = "default"
  session_timeouit = 1
  scopes = ["email", "profile"]
  amrs = ["mfa", "pwd", "user", "hwk"]
  verify_aud = true
  ````

## Run the app
- Build and run the app with `cargo run`
- Run `docker-compose up -d` to launch the Valkey and Jaeger containers.
- Browse to `localhost:3006` to manage the Resource Server (OIDC settings).
- Browse to `localhost:3000` to excercize OIDC based authentication.
- Browse to `localhost:16686` to see the OTEL traces.


For more info, read the Rust doc with `cargo doc --no-deps --open`