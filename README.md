# Okta OIDC Resource Server

This is a Rust based app.  
- Clone the repo
- In the repo root, create a `.env` file and add the following:
    ````toml
    SERVER_ADDR=127.0.0.1
    SERVER_PORT=3000
    SERVER_SESSION_TIMEOUT=1
    OKTA_DOMAIN=<YOUR Okta DOMAIN>
    OKTA_APP=<YOUR Okta APPLICATION>
    OIDC_CLIENT_ID=<YOUR Okta client_id>
    OIDC_CLIENT_SECRET=<YOUR OKTA client_secret>
    OIDC_SCOPES="email profile"
    OIDC_USERNAME="name"
    ````
- Build and run the app with `cargo run`
- Browse to `localhost:3000`.

This app is designed to be very minimal.  It does not require any other services, such as a cache or data layer.

For more info, read the Rust doc with `cargo doc --no-deps --open`