//! The core lib handles the morphological analysis of text.
//! As well as the matching of morphemes to dictionary entries.
//!
//! This library is designed to be used as a dependency for other
//! libraries and applications within the `komitan` project.
//!
//! It can also be compiled to a standalone binary with a CLI interface.
//!
//! How it works:
//! 1. Text is tokenized into individual words.
//! 2. Each word is then analyzed for morphemes.
//! 3. Each morpheme is then matched to a dictionary entry.
//! 4. The dictionary entry contains the meaning, reading, etc.
//! 5. The meaning, reading, etc. are then returned to the caller.
//!
//! The following diagram shows the flow of data through the library:
//! input text -> Tokenizer -> Morphemes -> Dictionary -> Meaning, Reading, etc.
//!
//! The implmentation of the library is split into the following modules:
//! - `analysis`: Contains the core analysis logic.
//! - `dictionary`: Contains the dictionary logic.
//! - `tokenizer`: Contains the tokenizer logic.
//! - `utils`: Contains utility functions.
//!
//! The dictionary format is based on the yomichan dictionary format.
//! See: https://foosoft.net/projects/yomichan/
//!
//! In the future, an import module will be added to convert from
//! other dictionary formats to the yomichan format.
//!

// All public modules are defined here.
pub mod analysis {
    pub mod dictionary;
    pub mod token;
}

pub mod dictionary {
    pub mod yomichan;
}
