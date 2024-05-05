//! # Okta OIDC Demo
//!
//! Many thanks to [pka/actix-web-oidc-auth](https://github.com/pka/actix-web-oidc-auth), on which this app is on top of.
//!
//! THis app uses [actix_web], [askama] (for html templates) and [openidconnect] to demonstrate a simple resource serve
//! that does standard OIDC Authorization Code authentication.
//!
//! ## Layout
//!
//! ## Services
//!
//! The app exposes 2 services:
//!
//! - The app service, running on port 3000 (by default) that performs OIDC authentication with OKta.
//!
//! - The admin service, running on port 3006 (by default) that configures the OIDC settings, and enables session management.
//!
//! ## Web Pages
//! Web pages are comprised of 3 components:
//! - HTML template, written with [askama].  These are located in the [templates] folder.
//!
//! - A page struct to hold the template attributes. These are located in the [pages] folder.
//!
//! - An [actix_web] handler.  These are located in the [handlers] folder.
//!
//! The [static] folder contains other assets, such as css files, that are loaded by the browser.
//!
//! ## [AppConfig]
//! Manages configuration by pulling from env variables.  Take a look to see what needs to be defined
//! either in the env or in a `.env` file.
//!
//!
//!

use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware, App, HttpServer,
};
use futures::future;
use tracing::*;
use tracing_actix_web::TracingLogger;

use config::{AppConfig, AppState};

mod admin;
mod app;
mod auth;
mod common;
mod config;
mod error;
mod otel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to load env file");

    // Set environment for logging configuration
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // crate::AppConfig is responsible for loading values from the env.
    let state = AppState::init().await.expect("AppState::init failed");
    let app_state = state.clone();

    // Set up the OpenTelemetry subscriber
    otel::init_subscriber();

    // Clone an instance for
    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    let s1 = HttpServer::new(move || {
        App::new()
            .configure(app::configure(app_state.clone()))
            // Unfortunately, middleware can't be wrapped by the configure function.  So add it here.
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    // For demo purposes, set the session TTL to just 1 minute.  Change this in .env
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::minutes(app_state.config.session_ttl)),
                    )
                    .build(),
            )
            .wrap(middleware::NormalizePath::trim())
        // Mount `TracingLogger` as a middleware
        //.wrap(TracingLogger::default())
    })
    .bind((
        state.config.server_address.as_ref(),
        state.config.server_port,
    ))?
    .run();

    info!(
        " HTTP app server running at http://{}:{}",
        &state.config.server_address, &state.config.server_port
    );

    let s2 = HttpServer::new(move || {
        App::new()
            .configure(admin::configure())
            // Mount `TracingLogger` as a middleware
            .wrap(TracingLogger::default())
    })
    .bind((state.config.server_address.as_ref(), 3006))?
    .run();
    info!(
        " HTTP admin server running at http://{}:3006",
        &state.config.server_address
    );

    future::try_join(s1, s2).await?;

    // Ensure all spans have been shipped to the collector.
    // TODO:  This causes exiting th hang
    //opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
