pub mod entities;
pub mod access;
pub mod routes;
pub mod errors;
pub mod settings;
pub mod handlers;

#[cfg(not(test))]
pub mod collections;

pub mod connectors;

#[cfg(test)]
mod tests;
