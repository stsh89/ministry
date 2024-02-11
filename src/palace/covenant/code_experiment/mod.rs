mod book;

pub use book::simulate_book_to_author_relationship;

pub type CodeExperiment = fn() -> ();
