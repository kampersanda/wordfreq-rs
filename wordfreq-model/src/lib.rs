//! # wordfreq-model
//!
//! This crate provides a loader for pre-compiled wordfreq models,
//! allowing you to easily create [`WordFreq`] instances for various languages.
//!
//! ## Instructions
//!
//! The provided models are the same as those distributed in the original Python package.
//! See the [original documentation](https://github.com/rspeer/wordfreq/tree/v3.0.2#sources-and-supported-languages)
//! for the supported languages and their sources.
//!
//! You need to specify models you want to use with `features`.
//! The feature names are in the form of `large-xx` or `small-xx`, where `xx` is the language code.
//! For example, if you want to use the large-English and small-Japanese models,
//! specify `large-en` and `small-ja` as follows:
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies.wordfreq-model]
//! version = "0.1"
//! features = ["large-en", "small-ja"]
//! ```
//!
//! There is no default feature.
//! **Be sure to specify at least one feature.**
//!
//! ## Examples
//!
//! [`load_wordfreq`] can create a [`WordFreq`] instance from a [`ModelKind`] enum value.
//! [`ModelKind`] will have the specified feature names in CamelCase.
//!
//! ```ignore
//! use wordfreq_model::load_wordfreq;
//! use wordfreq_model::ModelKind;
//!
//! let wf = load_wordfreq(ModelKind::LargeEn).unwrap();
//! println!("{:?}", wf.word_frequency("cafe"));
//! // => 1.2481286e-5
//! println!("{:?}", wf.zipf_frequency("cafe"));
//! // => 4.1
//! ```
//!
//! ## Notes
//!
//! This crate embeds models directly into the source code.
//! Specify as many models as you need to avoid bloating your binary.
use std::env;

use anyhow::Result;
use wordfreq::WordFreq;

/// Supported model kinds.
///
/// Since only specified feature names are available,
/// nothing will be displayed in docs.rs.
/// Normally, those you specify will be available such as `LargeEn` or `SmallJa`.
///
/// If models you want to use are not available,
/// specify the features following the Instructions.
pub enum ModelKind {
    #[cfg(feature = "large-en")]
    LargeEn,
    #[cfg(feature = "large-fr")]
    LargeFr,
    #[cfg(feature = "small-en")]
    SmallEn,
    #[cfg(feature = "small-ja")]
    SmallJa,
}

#[cfg(feature = "large-en")]
const DATA_LARGE_EN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_en.bin"));
#[cfg(feature = "large-fr")]
const DATA_LARGE_FR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_fr.bin"));
#[cfg(feature = "small-en")]
const DATA_SMALL_EN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_en.bin"));
#[cfg(feature = "small-ja")]
const DATA_SMALL_JA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ja.bin"));

/// Loads a pre-compiled [`WordFreq`] model.
pub fn load_wordfreq(kind: ModelKind) -> Result<WordFreq> {
    match kind {
        #[cfg(feature = "large-en")]
        ModelKind::LargeEn => Ok(WordFreq::deserialize(DATA_LARGE_EN)?),
        #[cfg(feature = "large-fr")]
        ModelKind::LargeFr => Ok(WordFreq::deserialize(DATA_LARGE_FR)?),
        #[cfg(feature = "small-en")]
        ModelKind::SmallEn => Ok(WordFreq::deserialize(DATA_SMALL_EN)?),
        #[cfg(feature = "small-ja")]
        ModelKind::SmallJa => Ok(WordFreq::deserialize(DATA_SMALL_JA)?),
    }
}
