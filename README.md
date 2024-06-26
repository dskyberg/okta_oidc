# Okta OIDC Resource Server

Okta acts as an Authentication Server for a Rust based app that acts as a Resource Server.  Meaning,
the browser is NOT the client, and does not need to be trusted.  The Resource Server is the client.

To setup in Okta, create a new Web application integration using OIDC.

This is a Rust based app.  It leverage these services (via Docker):
- [Valkey](https://valkey.io/) (open source Redis)  for in-memory (not persisted) session management,
- [Jaeger](https://www.jaegertracing.io/) (OpenTelemetry) for logging/tracing.  

## Client Credentials

I use 1Password, and the 1Password CLI to manage API tokens. See [export_op_creds](./export_op_creds) to see how I load them into the env.  But you do you.  However you typically manage credentals.

```bash
OIDC_CLIENT_ID='<your id>' OIDC_CLIENT_SECRET='<your secret>' cargo run 
```
There's also built-in support for [keyring](https://docs.rs/keyring) if you want to use that.

```bash
cargo run --features keyring

```

## Setup
There are a few variables that you should set.  Up to you how you manage your environment. 
I suggest using an `.env` file.  All the vars have defaults (shown).  So if the defaults work for you, that's all you need to set.

Note, the following shows how to set up client creds also.  But by default, this won't work.  You will need to
run the app with the `--no-default-features` flag.

In the repo root (same folder as `Cargo.toml`), create a `.env` file and add the following:

  ```sh
  SERVER_ADDR=127.0.0.1
  SERVER_PORT=3000
  SERVER_SESSION_TTL=1
  OIDC_CLIENT_ID=<YOUR Okta client_id>
  OIDC_CLIENT_SECRET=<YOUR OKTA client_secret>
  OTEL_URL=http://localhost:4317
  ```

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