#![deny(missing_docs)]

//! This crates deals with the content of the OpenTherm protocol

#[macro_use]
extern crate bitflags;


mod application;
mod conversation;
mod error;
mod message;

pub use error::Error;
pub use message::*;
pub use conversation::{Conversation, NullableComplexType};
pub use application::*;
