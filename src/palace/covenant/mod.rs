//! The place of ministerial agreements.
//!
//! More generally, this module will be called "entity".

mod code_experiment;
mod idea;

pub use code_experiment::{simulate_book_to_author_relationship, CodeExperiment};
pub use idea::Idea;
