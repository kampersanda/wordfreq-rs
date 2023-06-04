use regex::{Captures, Regex};

// Frequencies of leading digits, according to Benford's law, sort of.
// Benford's law doesn't describe numbers with leading zeroes, because "007"
// and "7" are the same number, but for us they should have different frequencies.
// I added an estimate for the frequency of numbers with leading zeroes.
const DIGIT_FREQS: [f32; 10] = [
    0.009, 0.300, 0.175, 0.124, 0.096, 0.078, 0.066, 0.057, 0.050, 0.045,
];

// Suppose you have a token NNNN, a 4-digit number representing a year. We're making
// a probability distribution of P(token=NNNN) | P(token is 4 digits).
//
// We do this with a piecewise exponential function whose peak is a plateau covering
// the years 2019 to 2039.

// Determined by experimentation: makes the probabilities of all years add up to 90%.
// The other 10% goes to NOT_YEAR_PROB. tests/test_numbers.py confirms that this
// probability distribution adds up to 1.
const YEAR_LOG_PEAK: f32 = -1.9185;
const NOT_YEAR_PROB: f32 = 0.1;
const REFERENCE_YEAR: f32 = 2019.;
const PLATEAU_WIDTH: f32 = 20.;

pub struct NumberHandler {
    digit_re: Regex,
    multi_digit_re: Regex,
    pure_digit_re: Regex,
}

impl NumberHandler {
    pub fn new() -> Self {
        Self {
            digit_re: Regex::new(r"\d").unwrap(),
            multi_digit_re: Regex::new(r"\d[\d.,]+").unwrap(),
            pure_digit_re: Regex::new(r"\d+").unwrap(),
        }
    }

    /// Replace sequences of multiple digits with zeroes, so we don't need to
    /// distinguish the frequencies of thousands of numbers.
    pub fn smash_numbers(&self, text: &str) -> String {
        self.multi_digit_re
            .replace_all(text, |captures: &Captures| self.sub_zeroes(captures))
            .to_string()
    }

    /// Given a regex match, return what it matched with digits replaced by
    /// zeroes.
    fn sub_zeroes(&self, captures: &Captures) -> String {
        let group0 = captures.get(0).unwrap().as_str();
        self.digit_re.replace_all(group0, "0").to_string()
    }

    /// Get the relative frequency of a string of digits, using our estimates.
    pub fn digit_freq(&self, text: &str) -> f32 {
        let mut freq = 1.;
        for m in self.multi_digit_re.find_iter(text) {
            for sm in self.pure_digit_re.find_iter(m.as_str()) {
                if sm.as_str().len() == 4 {
                    freq *= self.year_freq(sm.as_str());
                } else {
                    freq *= self.benford_freq(sm.as_str());
                }
            }
        }
        freq
    }

    /// Estimate the frequency of a digit sequence according to Benford's law.
    fn benford_freq(&self, text: &str) -> f32 {
        let chars = text.chars().collect::<Vec<char>>();
        let first_digit = chars[0].to_digit(10).unwrap() as usize;
        DIGIT_FREQS[first_digit] / 10f32.powi(chars.len() as i32 - 1)
    }

    /// Estimate the relative frequency of a particular 4-digit sequence representing
    /// a year.
    ///
    /// For example, suppose text == "1985". We're estimating the probability that a
    /// randomly-selected token from a large corpus will be "1985" and refer to the
    /// year, _given_ that it is 4 digits. Tokens that are not 4 digits are not involved
    /// in the probability distribution.
    fn year_freq(&self, text: &str) -> f32 {
        let year = text.parse::<f32>().unwrap();

        let year_log_freq = if year <= REFERENCE_YEAR {
            // Fitting a line to the curve seen at
            // https://twitter.com/r_speer/status/1493715982887571456.
            YEAR_LOG_PEAK - 0.0083 * (REFERENCE_YEAR - year)
        } else if REFERENCE_YEAR < year && year <= REFERENCE_YEAR + PLATEAU_WIDTH {
            // It's no longer 2019, which is when the Google Books data was last collected.
            // It's 2022 as I write this, and possibly even later as you're using it. Years
            // keep happening.
            //
            // So, we'll just keep the expected frequency of the "present" year constant for
            // 20 years.
            YEAR_LOG_PEAK
        } else {
            // Fall off quickly to catch up with the actual frequency of future years
            // (it's low). This curve is made up to fit with the made-up "present" data above.
            YEAR_LOG_PEAK - 0.2 * (year - (REFERENCE_YEAR + PLATEAU_WIDTH))
        };

        let year_prob = 10f32.powf(year_log_freq);

        // If this token _doesn't_ represent a year, then use the Benford frequency
        // distribution.
        let not_year_prob = NOT_YEAR_PROB * self.benford_freq(text);
        return year_prob + not_year_prob;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smash_numbers() {
        let nh = NumberHandler::new();
        let x = nh.smash_numbers("I have 1234.56 yen");
        dbg!(x);
    }
}
