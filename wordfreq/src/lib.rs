mod numbers;

use std::io::BufRead;

use anyhow::{anyhow, Result};
use hashbrown::HashMap;

pub type Float = f32;

pub struct WordFreq {
    map: HashMap<String, Float>,
    minimum: Float,
    num_handler: numbers::NumberHandler,
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

    pub fn minimum(mut self, minimum: Float) -> Self {
        self.minimum = minimum;
        self
    }

    pub fn word_frequency<W>(&self, word: W) -> Float
    where
        W: AsRef<str>,
    {
        self.word_frequency_in(word).unwrap_or(0.).max(self.minimum)
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
        Float::from(10.).powf(zipf - 9.)
    }

    fn freq_to_zipf(freq: Float) -> Float {
        freq.log10() + 9.
    }

    fn round(x: Float, places: i32) -> Float {
        let multiplier = Float::from(10.).powi(places);
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
/// let word_weight_text = "las 10\nvegas 20\n";
/// let word_weights = wordfreq::word_weights_from_text(word_weight_text.as_bytes())?;
///
/// assert_eq!(
///     word_weights,
///     vec![("las".to_string(), 10.), ("vegas".to_string(), 20.)]
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
