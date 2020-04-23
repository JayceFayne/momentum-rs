mod client;
mod reply;
mod routes;
#[cfg(test)]
mod tests;

pub use client::Client;
pub use reply::*;

pub type Fallible<T> = Result<T, anyhow::Error>;
