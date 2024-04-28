//! # Okta OIDC Demo
//!
//! Many thanks to [pka/actix-web-oidc-auth](https://github.com/pka/actix-web-oidc-auth), on which this app is on top of.
//! 
//! THis app uses [actix_web], [askama] (for html templates) and [openidconnect] to demonstrate a simple resource serve
//! that does standard OIDC Authorization Code authentication.
//!
//! ## App Layout
//! 
//! ## Web Pages
//! Web pages are comprised of 3 components:
//! - HTML template, written with [askama].  These are located in the [templates] folder.
//! - A page struct to hold the template attributes. These are located in the [pages] folder.
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
use actix_files::Files;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use dotenv;
use log::info;

use auth::{auth, OidcClient};
use config::AppConfig;
use handlers::{default_handler, index, login, logout};

mod auth;
mod config;
mod handlers;
mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to load env file");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // crate::AppConfig is responsible for loading values from the env.  
    let app_config = AppConfig::init();

    let oidc = OidcClient::setup(&app_config).await;

    let secret_key = Key::generate();

    info!(
        "starting HTTP server at http://{}:{}",
        &&app_config.server_address, &app_config.server_port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(oidc.clone()))
            .service(web::resource("/login").route(web::get().to(login)))
            .service(web::resource("/auth").route(web::get().to(auth)))
            .service(web::resource("/logout").route(web::get().to(logout)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(Files::new("/static", "static").show_files_listing())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(app_config.session_timeout.clone()),
                    )
                    .build(),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .default_service(web::to(default_handler))
    })
    .bind((app_config.server_address.as_ref(), app_config.server_port))?
    .run()
    .await
}
