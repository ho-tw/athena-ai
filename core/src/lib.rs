//! Core types and traits for the AI agent framework.
//!
//! This crate provides fundamental types used throughout the framework:
//! - [`Message`] and [`Role`] for representing conversation turns
//! - [`AgentError`] for error handling across all components
//! - [`Result`] type alias for convenient error propagation
//!
//! # Example
//!
//! ```
//! use agent_core::{Message, Role};
//!
//! let msg = Message::user("Hello, world!");
//! assert_eq!(msg.role, Role::User);
//! ```

mod error;
mod message;

pub use error::{AgentError, Result};
pub use message::{Message, Role};
