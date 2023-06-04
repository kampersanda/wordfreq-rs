mod numbers;

use std::io::BufRead;

use anyhow::{anyhow, Result};
use hashbrown::HashMap;

///
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
///
/// assert_relative_eq!(wf.zipf_frequency("las"), 0.25);
/// assert_relative_eq!(wf.zipf_frequency("vegas"), 0.75);
/// assert_relative_eq!(wf.zipf_frequency("Las"), 0.00);
/// ```
pub struct WordFreq {
    map: HashMap<String, f32>,
    minimum: f32,
}

impl WordFreq {
    /// Creates the language model.
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
        I: IntoIterator<Item = (W, f32)>,
        W: AsRef<str>,
    {
        let mut map: HashMap<_, _> = word_weights
            .into_iter()
            .map(|(word, weight)| (word.as_ref().to_string(), weight))
            .collect();
        let sum_weight = map.values().fold(0., |acc, w| acc + w);
        map.values_mut().for_each(|w| *w /= sum_weight);
        Self { map, minimum: 0. }
    }

    pub fn minimum(mut self, minimum: f32) -> Self {
        self.minimum = minimum;
        self
    }

    pub fn word_frequency<W>(&self, word: W) -> f32
    where
        W: AsRef<str>,
    {
        self.get(word).max(self.minimum)
    }

    /// Returns the probability for an input word.
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
    pub fn zipf_frequency<W>(&self, word: W) -> f32
    where
        W: AsRef<str>,
    {
        let freq_min = Self::zipf_to_freq(self.minimum);
        let freq = self.get(word).max(freq_min);
        let zipf = Self::freq_to_zipf(freq);
        Self::round(zipf, 2)
    }

    fn get<W>(&self, word: W) -> f32
    where
        W: AsRef<str>,
    {
        self.map.get(word.as_ref()).cloned().unwrap_or(0.)
    }

    fn zipf_to_freq(zipf: f32) -> f32 {
        10f32.powf(zipf - 9.)
    }

    fn freq_to_zipf(freq: f32) -> f32 {
        freq.log10() + 9.
    }

    fn round(x: f32, places: i32) -> f32 {
        let multiplier = 10f32.powi(places);
        (x * multiplier).round() / multiplier
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![];
        for (k, v) in &self.map {
            bincode::serialize_into(&mut bytes, k.as_bytes())?;
            bincode::serialize_into(&mut bytes, v)?;
        }
        Ok(bytes)
    }

    pub fn deserialize(mut bytes: &[u8]) -> Result<Self> {
        let mut map = HashMap::new();
        while !bytes.is_empty() {
            let k: String = bincode::deserialize_from(&mut bytes)?;
            let v: f32 = bincode::deserialize_from(&mut bytes)?;
            map.insert(k, v);
        }
        Ok(Self { map, minimum: 0. })
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
/// use sif_embedding::util;
///
/// let word_weight_text = "las 10\nvegas 20\n";
/// let word_weights = util::word_weights_from_text(word_weight_text.as_bytes())?;
///
/// assert_eq!(
///     word_weights,
///     vec![("las".to_string(), 10.), ("vegas".to_string(), 20.)]
/// );
/// # Ok(())
/// # }
/// ```
pub fn word_weights_from_text<R: BufRead>(rdr: R) -> Result<Vec<(String, f32)>> {
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
