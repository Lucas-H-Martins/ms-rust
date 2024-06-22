use std::{env, fmt::Error};
use tracing::Level;
use tracing_subscriber;

pub fn environment_setup() -> Result<(), Error> {
    dotenvy::from_filename(
        match env::var("RUST_ENV")
            .unwrap_or_else(|_| "local".to_string())
            .as_str()
        {
            "local" => ".env.local",
            "dev" => ".env.dev",
            "stg" => ".env.stg",
            "prd" => ".env.prd",
            _ => ".env.local",
        },
    )
    .ok();

    tracing_subscriber::fmt()
        .with_max_level(
            match env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "trace".to_string())
                .as_str()
            {
                "info" => Level::INFO,
                "warn" => Level::WARN,
                "debug" => Level::DEBUG,
                "trace" => Level::TRACE,
                _ => Level::INFO,
            },
        )
        .init();

    Ok(())
}
