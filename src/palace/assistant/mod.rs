//! Place for various implementations.
//!
//! Generally, this module will be called 'client'.

mod laboratory_manager;
mod psychologist;
mod secretary;

pub use laboratory_manager::LaboratoryManager;
pub use psychologist::Psychologist;
pub use secretary::Secretary;
