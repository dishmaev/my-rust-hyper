pub mod entities;
pub mod commands;
pub mod events;
pub mod traits;
pub mod access;
pub mod routes;
pub mod errors;
pub mod settings;
pub mod handlers;

#[cfg(not(test))]
pub mod collections;

pub mod connectors;
pub mod router;
pub mod publishers;
pub mod executors;

#[cfg(test)]
mod tests;
