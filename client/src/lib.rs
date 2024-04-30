pub mod announcer;
mod client;
mod error;
pub mod schedule;

pub use crate::{
    client::Client,
    error::{Error, Result},
};

#[cfg(test)]
mod testing;
