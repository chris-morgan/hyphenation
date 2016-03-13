// Forsaken docs justly quibble the vexed programmer's waning zeal
//! Text hyphenation in a variety of languages.
//!
//!
//! ## Usage
//!
//! A typical import comprises the `Hyphenation` trait, the `Standard`
//! hyphenator, and the `Language` enum. This exposes the crate's core
//! functionality, and the set of available languages.
//!
//!  ```rust
//! extern crate hyphenation;
//
//! use hyphenation::{Hyphenation, Standard, Language};
//! ```
//!
//! To begin with, we must initiate the `Corpus` for our working language.
//!
//! ```rust
//! let english_us = hyphenation::load(Language::English_US).unwrap();
//! ```
//!
//! Our English `Corpus` can now be used by *hyphenators*: iterators which
//! segment text in accordance with hyphenation practices, as described
//! by the corpus.
//!
//! The simplest (and, presently, only) hyphenator is `Standard`:
//!
//! ```rust
//! let h: Standard = "hyphenation".hyphenate(&english_us);
//! ```
//!
//! The `Standard` hyphenator does not allocate new strings, returning
//! slices instead.
//!
//! ```rust
//! let v: Vec<&str> = h.collect();
//! assert_eq!(v, vec!["hy", "phen", "ation"]);
//! ```
//!
//! While hyphenation is performed on a per-word basis, convenience calls
//! for `Hyphenation` to work with full text by default.
//!
//! ```rust
//! let h2: Standard = "Word hyphenation by computer.".hyphenate(&english_us);
//! let v2: Vec<&str> = h2.collect();
//! assert_eq!(v2, vec!["Word hy", "phen", "ation by com", "puter."]);
//! ```
//!
//! Moreover, hyphenators expose some simple methods to render hyphenated
//! text: `punctuate()` and `punctuate_with(string)`, which mark hyphenation
//! opportunities respectively with soft hyphens (Unicode `U+00AD SOFT HYPHEN`)
//! and any given `string`.
//!
//! ```rust
//! let h3 = "anfractuous".hyphenate(&english_us);
//! let s3: String = h2.clone().punctuate().collect();
//! assert_eq!(s3, "an\u{ad}frac\u{ad}tu\u{ad}ous".to_owned());
//!
//! let s4: String = h2.punctuate_with("-").collect()
//! assert_eq!(s4, "an-frac-tu-ous".to_owned());
//! ```
//!
//! If we would rather manipulate our text in other ways, we may employ
//! `opportunities()`, which returns the byte indices of hyphenation opportunities
//! within the string. (Internally, `opportunities()` is the fundamental method
//! required by `Hyphenation`; other functionality is implemented on top of it.)
//!
//! ```rust
//! let indices = "hyphenation".opportunities(&english_us);
//! assert_eq!(indices, vec![…]);
//! ```


extern crate fnv;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate unicode_segmentation;

mod klpair;
mod utilia;
pub mod exception;
pub mod hyphenator;
pub mod language;
pub mod load;
pub mod pattern;

pub use hyphenator::{Hyphenation, Standard};
pub use language::{Language, Corpus};
pub use load::{set_pattern_folder, language as load};