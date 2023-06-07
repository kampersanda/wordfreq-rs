//! # wordfreq
//!
//! This crate is a yet another Rust port of [Python's wordfreq](https://github.com/rspeer/wordfreq),
//! allowing you to look up the frequencies of words in many languages.
//!
//! Note that **this crate provides only the algorithms and does not contain the models**.
//! Use [wordfreq-model](https://docs.rs/wordfreq-model/) to easily load distributed models.
//! We recommend to see the [documentation](https://docs.rs/wordfreq-model/) for quick start.
//!
//! ## Development status
//!
//! This aims to reproduce the behavior of the original Python implementation,
//! but it is not yet perfect (and we do not know if we will provide everything).
//!
//! Features currently provided and not provided are listed below.
//!
//! ### Provided
//!
//! - [Original wordfreq models](https://github.com/rspeer/wordfreq/tree/v3.0.2#sources-and-supported-languages)
//! - [Estimation for numbers](https://github.com/rspeer/wordfreq/tree/v3.0.2#numbers)
//!
//! ### Not provided
//!
//! - [Additional functions](https://github.com/rspeer/wordfreq/tree/v3.0.2#other-functions)
//! - [Tokenization and normalization](https://github.com/rspeer/wordfreq/tree/v3.0.2#tokenization)
//! - [Multi-script languages](https://github.com/rspeer/wordfreq/tree/v3.0.2#multi-script-languages)
//!
//! ## Precision errors
//!
//! Even if the algorithms are the same, the results may differ slightly from the original implementation
//! due to floating point precision.
//!
//! ## How to create instances from model files without wordfreq-model
//!
//! If you do not desire automatic model downloads using [wordfreq-model](https://docs.rs/wordfreq-model/),
//! you can create instances directly from the actual model files placed [here](https://github.com/kampersanda/wordfreq-rs/releases/tag/models-v1).
//! The model files describe words and their frequencies in the text format:
//!
//! ```text
//! <word1> <freq1>
//! <word2> <freq2>
//! <word3> <freq3>
//! ...
//! ```
//!
//! You can create instances as follows:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use approx::assert_relative_eq;
//! use wordfreq::WordFreq;
//!
//! let word_weight_text = "las 10\nvegas 30\n";
//! let word_weights = wordfreq::word_weights_from_text(word_weight_text.as_bytes())?;
//!
//! let wf = WordFreq::new(word_weights);
//! assert_relative_eq!(wf.word_frequency("las"), 0.25);
//! assert_relative_eq!(wf.word_frequency("vegas"), 0.75);
//! # Ok(())
//! # }
//! ```
#![deny(missing_docs)]

mod numbers;
pub mod preprocessers;

use std::io::BufRead;

use anyhow::{anyhow, Result};
use hashbrown::HashMap;

/// Common type of floating numbers.
pub type Float = f32;

// To avoid annoying clippy errors.
const FLOAT_10: Float = 10.;

/// Implementation of wordfreq.
pub struct WordFreq {
    map: HashMap<String, Float>,
    minimum: Float,
    num_handler: numbers::NumberHandler,
}

impl WordFreq {
    /// Creates an instance from frequencies.
    ///
    /// # Arguments
    ///
    /// - `word_weights`: Pairs of words and their frequencies (or probabilities) from a corpus.
    ///
    /// # Notes
    ///
    /// If the input contains duplicate words, the last occurrence is used.
    pub fn new<I, W>(word_weights: I) -> Self
    where
        I: IntoIterator<Item = (W, Float)>,
        W: AsRef<str>,
    {
        let mut map: HashMap<_, _> = word_weights
            .into_iter()
            .map(|(word, weight)| (word.as_ref().to_string(), weight))
            .collect();
        let sum_weight = map.values().fold(0., |acc, w| acc + w);
        map.values_mut().for_each(|w| *w /= sum_weight);
        Self {
            map,
            minimum: 0.,
            num_handler: numbers::NumberHandler::new(),
        }
    }

    /// Sets the lower bound of returned frequencies (default is 0.0).
    ///
    /// An error is returned if the input is negative.
    pub fn minimum(mut self, minimum: Float) -> Result<Self> {
        if minimum < 0. {
            return Err(anyhow!("minimum must be non-negative"));
        }
        self.minimum = minimum;
        Ok(self)
    }

    /// Returns the word's frequency, normalized between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use wordfreq::WordFreq;
    ///
    /// let word_weights = [("las", 10.), ("vegas", 30.)];
    /// let wf = WordFreq::new(word_weights);
    ///
    /// assert_relative_eq!(wf.word_frequency("las"), 0.25);
    /// assert_relative_eq!(wf.word_frequency("vegas"), 0.75);
    /// assert_relative_eq!(wf.word_frequency("Las"), 0.00);
    /// ```
    pub fn word_frequency<W>(&self, word: W) -> Float
    where
        W: AsRef<str>,
    {
        self.word_frequency_in(word).unwrap_or(0.).max(self.minimum)
    }

    /// Returns the Zipf frequency of a word as a human-friendly logarithmic scale.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use wordfreq::WordFreq;
    ///
    /// let word_weights = [("las", 10.), ("vegas", 30.)];
    /// let wf = WordFreq::new(word_weights);
    ///
    /// assert_relative_eq!(wf.zipf_frequency("las"), 8.4);
    /// assert_relative_eq!(wf.zipf_frequency("vegas"), 8.88);
    /// assert_relative_eq!(wf.zipf_frequency("Las"), 0.00);
    /// ```
    pub fn zipf_frequency<W>(&self, word: W) -> Float
    where
        W: AsRef<str>,
    {
        let freq_min = Self::zipf_to_freq(self.minimum);
        let freq = self.word_frequency_in(word).unwrap_or(0.).max(freq_min);
        let zipf = Self::freq_to_zipf(freq);
        Self::round(zipf, 2)
    }

    fn word_frequency_in<W>(&self, word: W) -> Option<Float>
    where
        W: AsRef<str>,
    {
        let word = word.as_ref();
        let smashed = self.num_handler.smash_numbers(word);
        let mut freq = self.map.get(&smashed).cloned()?;

        if smashed != word {
            // If there is a digit sequence in the token, the digits are
            // internally replaced by 0s to aggregate their probabilities
            // together. We then assign a specific frequency to the digit
            // sequence using the `digit_freq` distribution.
            freq *= self.num_handler.digit_freq(word);
        }

        // All our frequency data is only precise to within 1% anyway, so round
        // it to 3 significant digits
        // let leading_zeroes = (-freq.log10()).floor() as i32;
        // Some(Self::round(freq, leading_zeroes + 3))

        // NOTE(kampersanda): Rounding would not always be necessary.
        Some(freq)
    }

    fn zipf_to_freq(zipf: Float) -> Float {
        FLOAT_10.powf(zipf - 9.)
    }

    fn freq_to_zipf(freq: Float) -> Float {
        freq.log10() + 9.
    }

    fn round(x: Float, places: i32) -> Float {
        let multiplier = FLOAT_10.powi(places);
        (x * multiplier).round() / multiplier
    }

    /// Exports the model data.
    ///
    /// Note that the format is distinct from the one used in the oritinal Python package.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![];
        for (k, v) in &self.map {
            bincode::serialize_into(&mut bytes, k.as_bytes())?;
            bincode::serialize_into(&mut bytes, v)?;
        }
        Ok(bytes)
    }

    /// Deserializes the model, which is exported by [`WordFreq::serialize()`].
    pub fn deserialize(mut bytes: &[u8]) -> Result<Self> {
        let mut map = HashMap::new();
        while !bytes.is_empty() {
            let k: String = bincode::deserialize_from(&mut bytes)?;
            let v: Float = bincode::deserialize_from(&mut bytes)?;
            map.insert(k, v);
        }
        Ok(Self {
            map,
            minimum: 0.,
            num_handler: numbers::NumberHandler::new(),
        })
    }
}

/// Parses pairs of a word and its weight from a text file,
/// where each line has a word and its weight sparated by the ASCII whitespace.
///
/// ```text
/// <word> <weight>
/// ```
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let word_weight_text = "las 10\nvegas 30\n";
/// let word_weights = wordfreq::word_weights_from_text(word_weight_text.as_bytes())?;
///
/// assert_eq!(
///     word_weights,
///     vec![("las".to_string(), 10.), ("vegas".to_string(), 30.)]
/// );
/// # Ok(())
/// # }
/// ```
pub fn word_weights_from_text<R: BufRead>(rdr: R) -> Result<Vec<(String, Float)>> {
    let mut word_weights = vec![];
    for (i, line) in rdr.lines().enumerate() {
        let line = line?;
        let cols: Vec<_> = line.split_ascii_whitespace().collect();
        if cols.len() != 2 {
            return Err(anyhow!(
                "Line {i}: a line should be <word><SPACE><weight>, but got {line}."
            ));
        }
        word_weights.push((cols[0].to_string(), cols[1].parse()?));
    }
    Ok(word_weights)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io() {
        let word_weights = [("las", 10.), ("vegas", 30.)];
        let wf = WordFreq::new(word_weights);

        let model = wf.serialize().unwrap();
        let other = WordFreq::deserialize(&model[..]).unwrap();

        assert_eq!(wf.map, other.map);
    }
}
