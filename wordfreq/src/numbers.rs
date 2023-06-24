// Copyright 2022 Robyn Speer
// Copyright 2023 Shunsuke Kanda
//
// The code is a port from https://github.com/rspeer/wordfreq/blob/v3.0.2/wordfreq/numbers.py
// together with the comments, following the MIT-license.
use hashbrown::HashMap;
use regex::{Captures, Regex};

use crate::Float;

// Frequencies of leading digits, according to Benford's law, sort of.
// Benford's law doesn't describe numbers with leading zeroes, because "007"
// and "7" are the same number, but for us they should have different frequencies.
// I added an estimate for the frequency of numbers with leading zeroes.
const DIGIT_FREQS: [Float; 10] = [
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
const YEAR_LOG_PEAK: Float = -1.9185;
const NOT_YEAR_PROB: Float = 0.1;
const REFERENCE_YEAR: Float = 2019.;
const PLATEAU_WIDTH: Float = 20.;

// To avoid annoying clippy errors.
const FLOAT_10: Float = 10.;
const FLOAT_0_2: Float = 0.2;
const FLOAT_0_0083: Float = 0.0083;

// Mapping from characters defined in \d (=\p{Nd}) to digits.
const DIGIT_MAPPING: &[(char, u32)] = &[
    ('0', 0),
    ('1', 1),
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('٠', 0),
    ('١', 1),
    ('٢', 2),
    ('٣', 3),
    ('٤', 4),
    ('٥', 5),
    ('٦', 6),
    ('٧', 7),
    ('٨', 8),
    ('٩', 9),
    ('۰', 0),
    ('۱', 1),
    ('۲', 2),
    ('۳', 3),
    ('۴', 4),
    ('۵', 5),
    ('۶', 6),
    ('۷', 7),
    ('۸', 8),
    ('۹', 9),
    ('߀', 0),
    ('߁', 1),
    ('߂', 2),
    ('߃', 3),
    ('߄', 4),
    ('߅', 5),
    ('߆', 6),
    ('߇', 7),
    ('߈', 8),
    ('߉', 9),
    ('०', 0),
    ('१', 1),
    ('२', 2),
    ('३', 3),
    ('४', 4),
    ('५', 5),
    ('६', 6),
    ('७', 7),
    ('८', 8),
    ('९', 9),
    ('০', 0),
    ('১', 1),
    ('২', 2),
    ('৩', 3),
    ('৪', 4),
    ('৫', 5),
    ('৬', 6),
    ('৭', 7),
    ('৮', 8),
    ('৯', 9),
    ('੦', 0),
    ('੧', 1),
    ('੨', 2),
    ('੩', 3),
    ('੪', 4),
    ('੫', 5),
    ('੬', 6),
    ('੭', 7),
    ('੮', 8),
    ('੯', 9),
    ('૦', 0),
    ('૧', 1),
    ('૨', 2),
    ('૩', 3),
    ('૪', 4),
    ('૫', 5),
    ('૬', 6),
    ('૭', 7),
    ('૮', 8),
    ('૯', 9),
    ('୦', 0),
    ('୧', 1),
    ('୨', 2),
    ('୩', 3),
    ('୪', 4),
    ('୫', 5),
    ('୬', 6),
    ('୭', 7),
    ('୮', 8),
    ('୯', 9),
    ('௦', 0),
    ('௧', 1),
    ('௨', 2),
    ('௩', 3),
    ('௪', 4),
    ('௫', 5),
    ('௬', 6),
    ('௭', 7),
    ('௮', 8),
    ('௯', 9),
    ('౦', 0),
    ('౧', 1),
    ('౨', 2),
    ('౩', 3),
    ('౪', 4),
    ('౫', 5),
    ('౬', 6),
    ('౭', 7),
    ('౮', 8),
    ('౯', 9),
    ('೦', 0),
    ('೧', 1),
    ('೨', 2),
    ('೩', 3),
    ('೪', 4),
    ('೫', 5),
    ('೬', 6),
    ('೭', 7),
    ('೮', 8),
    ('೯', 9),
    ('൦', 0),
    ('൧', 1),
    ('൨', 2),
    ('൩', 3),
    ('൪', 4),
    ('൫', 5),
    ('൬', 6),
    ('൭', 7),
    ('൮', 8),
    ('൯', 9),
    ('෦', 0),
    ('෧', 1),
    ('෨', 2),
    ('෩', 3),
    ('෪', 4),
    ('෫', 5),
    ('෬', 6),
    ('෭', 7),
    ('෮', 8),
    ('෯', 9),
    ('๐', 0),
    ('๑', 1),
    ('๒', 2),
    ('๓', 3),
    ('๔', 4),
    ('๕', 5),
    ('๖', 6),
    ('๗', 7),
    ('๘', 8),
    ('๙', 9),
    ('໐', 0),
    ('໑', 1),
    ('໒', 2),
    ('໓', 3),
    ('໔', 4),
    ('໕', 5),
    ('໖', 6),
    ('໗', 7),
    ('໘', 8),
    ('໙', 9),
    ('༠', 0),
    ('༡', 1),
    ('༢', 2),
    ('༣', 3),
    ('༤', 4),
    ('༥', 5),
    ('༦', 6),
    ('༧', 7),
    ('༨', 8),
    ('༩', 9),
    ('၀', 0),
    ('၁', 1),
    ('၂', 2),
    ('၃', 3),
    ('၄', 4),
    ('၅', 5),
    ('၆', 6),
    ('၇', 7),
    ('၈', 8),
    ('၉', 9),
    ('႐', 0),
    ('႑', 1),
    ('႒', 2),
    ('႓', 3),
    ('႔', 4),
    ('႕', 5),
    ('႖', 6),
    ('႗', 7),
    ('႘', 8),
    ('႙', 9),
    ('០', 0),
    ('១', 1),
    ('២', 2),
    ('៣', 3),
    ('៤', 4),
    ('៥', 5),
    ('៦', 6),
    ('៧', 7),
    ('៨', 8),
    ('៩', 9),
    ('᠐', 0),
    ('᠑', 1),
    ('᠒', 2),
    ('᠓', 3),
    ('᠔', 4),
    ('᠕', 5),
    ('᠖', 6),
    ('᠗', 7),
    ('᠘', 8),
    ('᠙', 9),
    ('᥆', 0),
    ('᥇', 1),
    ('᥈', 2),
    ('᥉', 3),
    ('᥊', 4),
    ('᥋', 5),
    ('᥌', 6),
    ('᥍', 7),
    ('᥎', 8),
    ('᥏', 9),
    ('᧐', 0),
    ('᧑', 1),
    ('᧒', 2),
    ('᧓', 3),
    ('᧔', 4),
    ('᧕', 5),
    ('᧖', 6),
    ('᧗', 7),
    ('᧘', 8),
    ('᧙', 9),
    ('᪀', 0),
    ('᪁', 1),
    ('᪂', 2),
    ('᪃', 3),
    ('᪄', 4),
    ('᪅', 5),
    ('᪆', 6),
    ('᪇', 7),
    ('᪈', 8),
    ('᪉', 9),
    ('᪐', 0),
    ('᪑', 1),
    ('᪒', 2),
    ('᪓', 3),
    ('᪔', 4),
    ('᪕', 5),
    ('᪖', 6),
    ('᪗', 7),
    ('᪘', 8),
    ('᪙', 9),
    ('᭐', 0),
    ('᭑', 1),
    ('᭒', 2),
    ('᭓', 3),
    ('᭔', 4),
    ('᭕', 5),
    ('᭖', 6),
    ('᭗', 7),
    ('᭘', 8),
    ('᭙', 9),
    ('᮰', 0),
    ('᮱', 1),
    ('᮲', 2),
    ('᮳', 3),
    ('᮴', 4),
    ('᮵', 5),
    ('᮶', 6),
    ('᮷', 7),
    ('᮸', 8),
    ('᮹', 9),
    ('᱀', 0),
    ('᱁', 1),
    ('᱂', 2),
    ('᱃', 3),
    ('᱄', 4),
    ('᱅', 5),
    ('᱆', 6),
    ('᱇', 7),
    ('᱈', 8),
    ('᱉', 9),
    ('᱐', 0),
    ('᱑', 1),
    ('᱒', 2),
    ('᱓', 3),
    ('᱔', 4),
    ('᱕', 5),
    ('᱖', 6),
    ('᱗', 7),
    ('᱘', 8),
    ('᱙', 9),
    ('꘠', 0),
    ('꘡', 1),
    ('꘢', 2),
    ('꘣', 3),
    ('꘤', 4),
    ('꘥', 5),
    ('꘦', 6),
    ('꘧', 7),
    ('꘨', 8),
    ('꘩', 9),
    ('꣐', 0),
    ('꣑', 1),
    ('꣒', 2),
    ('꣓', 3),
    ('꣔', 4),
    ('꣕', 5),
    ('꣖', 6),
    ('꣗', 7),
    ('꣘', 8),
    ('꣙', 9),
    ('꤀', 0),
    ('꤁', 1),
    ('꤂', 2),
    ('꤃', 3),
    ('꤄', 4),
    ('꤅', 5),
    ('꤆', 6),
    ('꤇', 7),
    ('꤈', 8),
    ('꤉', 9),
    ('꧐', 0),
    ('꧑', 1),
    ('꧒', 2),
    ('꧓', 3),
    ('꧔', 4),
    ('꧕', 5),
    ('꧖', 6),
    ('꧗', 7),
    ('꧘', 8),
    ('꧙', 9),
    ('꧰', 0),
    ('꧱', 1),
    ('꧲', 2),
    ('꧳', 3),
    ('꧴', 4),
    ('꧵', 5),
    ('꧶', 6),
    ('꧷', 7),
    ('꧸', 8),
    ('꧹', 9),
    ('꩐', 0),
    ('꩑', 1),
    ('꩒', 2),
    ('꩓', 3),
    ('꩔', 4),
    ('꩕', 5),
    ('꩖', 6),
    ('꩗', 7),
    ('꩘', 8),
    ('꩙', 9),
    ('꯰', 0),
    ('꯱', 1),
    ('꯲', 2),
    ('꯳', 3),
    ('꯴', 4),
    ('꯵', 5),
    ('꯶', 6),
    ('꯷', 7),
    ('꯸', 8),
    ('꯹', 9),
    ('０', 0),
    ('１', 1),
    ('２', 2),
    ('３', 3),
    ('４', 4),
    ('５', 5),
    ('６', 6),
    ('７', 7),
    ('８', 8),
    ('９', 9),
    ('𐒠', 0),
    ('𐒡', 1),
    ('𐒢', 2),
    ('𐒣', 3),
    ('𐒤', 4),
    ('𐒥', 5),
    ('𐒦', 6),
    ('𐒧', 7),
    ('𐒨', 8),
    ('𐒩', 9),
    ('𐴰', 0),
    ('𐴱', 1),
    ('𐴲', 2),
    ('𐴳', 3),
    ('𐴴', 4),
    ('𐴵', 5),
    ('𐴶', 6),
    ('𐴷', 7),
    ('𐴸', 8),
    ('𐴹', 9),
    ('𑁦', 0),
    ('𑁧', 1),
    ('𑁨', 2),
    ('𑁩', 3),
    ('𑁪', 4),
    ('𑁫', 5),
    ('𑁬', 6),
    ('𑁭', 7),
    ('𑁮', 8),
    ('𑁯', 9),
    ('𑃰', 0),
    ('𑃱', 1),
    ('𑃲', 2),
    ('𑃳', 3),
    ('𑃴', 4),
    ('𑃵', 5),
    ('𑃶', 6),
    ('𑃷', 7),
    ('𑃸', 8),
    ('𑃹', 9),
    ('𑄶', 0),
    ('𑄷', 1),
    ('𑄸', 2),
    ('𑄹', 3),
    ('𑄺', 4),
    ('𑄻', 5),
    ('𑄼', 6),
    ('𑄽', 7),
    ('𑄾', 8),
    ('𑄿', 9),
    ('𑇐', 0),
    ('𑇑', 1),
    ('𑇒', 2),
    ('𑇓', 3),
    ('𑇔', 4),
    ('𑇕', 5),
    ('𑇖', 6),
    ('𑇗', 7),
    ('𑇘', 8),
    ('𑇙', 9),
    ('𑋰', 0),
    ('𑋱', 1),
    ('𑋲', 2),
    ('𑋳', 3),
    ('𑋴', 4),
    ('𑋵', 5),
    ('𑋶', 6),
    ('𑋷', 7),
    ('𑋸', 8),
    ('𑋹', 9),
    ('𑑐', 0),
    ('𑑑', 1),
    ('𑑒', 2),
    ('𑑓', 3),
    ('𑑔', 4),
    ('𑑕', 5),
    ('𑑖', 6),
    ('𑑗', 7),
    ('𑑘', 8),
    ('𑑙', 9),
    ('𑓐', 0),
    ('𑓑', 1),
    ('𑓒', 2),
    ('𑓓', 3),
    ('𑓔', 4),
    ('𑓕', 5),
    ('𑓖', 6),
    ('𑓗', 7),
    ('𑓘', 8),
    ('𑓙', 9),
    ('𑙐', 0),
    ('𑙑', 1),
    ('𑙒', 2),
    ('𑙓', 3),
    ('𑙔', 4),
    ('𑙕', 5),
    ('𑙖', 6),
    ('𑙗', 7),
    ('𑙘', 8),
    ('𑙙', 9),
    ('𑛀', 0),
    ('𑛁', 1),
    ('𑛂', 2),
    ('𑛃', 3),
    ('𑛄', 4),
    ('𑛅', 5),
    ('𑛆', 6),
    ('𑛇', 7),
    ('𑛈', 8),
    ('𑛉', 9),
    ('𑜰', 0),
    ('𑜱', 1),
    ('𑜲', 2),
    ('𑜳', 3),
    ('𑜴', 4),
    ('𑜵', 5),
    ('𑜶', 6),
    ('𑜷', 7),
    ('𑜸', 8),
    ('𑜹', 9),
    ('𑣠', 0),
    ('𑣡', 1),
    ('𑣢', 2),
    ('𑣣', 3),
    ('𑣤', 4),
    ('𑣥', 5),
    ('𑣦', 6),
    ('𑣧', 7),
    ('𑣨', 8),
    ('𑣩', 9),
    ('𑥐', 0),
    ('𑥑', 1),
    ('𑥒', 2),
    ('𑥓', 3),
    ('𑥔', 4),
    ('𑥕', 5),
    ('𑥖', 6),
    ('𑥗', 7),
    ('𑥘', 8),
    ('𑥙', 9),
    ('𑱐', 0),
    ('𑱑', 1),
    ('𑱒', 2),
    ('𑱓', 3),
    ('𑱔', 4),
    ('𑱕', 5),
    ('𑱖', 6),
    ('𑱗', 7),
    ('𑱘', 8),
    ('𑱙', 9),
    ('𑵐', 0),
    ('𑵑', 1),
    ('𑵒', 2),
    ('𑵓', 3),
    ('𑵔', 4),
    ('𑵕', 5),
    ('𑵖', 6),
    ('𑵗', 7),
    ('𑵘', 8),
    ('𑵙', 9),
    ('𑶠', 0),
    ('𑶡', 1),
    ('𑶢', 2),
    ('𑶣', 3),
    ('𑶤', 4),
    ('𑶥', 5),
    ('𑶦', 6),
    ('𑶧', 7),
    ('𑶨', 8),
    ('𑶩', 9),
    ('𖩠', 0),
    ('𖩡', 1),
    ('𖩢', 2),
    ('𖩣', 3),
    ('𖩤', 4),
    ('𖩥', 5),
    ('𖩦', 6),
    ('𖩧', 7),
    ('𖩨', 8),
    ('𖩩', 9),
    ('𖭐', 0),
    ('𖭑', 1),
    ('𖭒', 2),
    ('𖭓', 3),
    ('𖭔', 4),
    ('𖭕', 5),
    ('𖭖', 6),
    ('𖭗', 7),
    ('𖭘', 8),
    ('𖭙', 9),
    ('𝟎', 0),
    ('𝟏', 1),
    ('𝟐', 2),
    ('𝟑', 3),
    ('𝟒', 4),
    ('𝟓', 5),
    ('𝟔', 6),
    ('𝟕', 7),
    ('𝟖', 8),
    ('𝟗', 9),
    ('𝟘', 0),
    ('𝟙', 1),
    ('𝟚', 2),
    ('𝟛', 3),
    ('𝟜', 4),
    ('𝟝', 5),
    ('𝟞', 6),
    ('𝟟', 7),
    ('𝟠', 8),
    ('𝟡', 9),
    ('𝟢', 0),
    ('𝟣', 1),
    ('𝟤', 2),
    ('𝟥', 3),
    ('𝟦', 4),
    ('𝟧', 5),
    ('𝟨', 6),
    ('𝟩', 7),
    ('𝟪', 8),
    ('𝟫', 9),
    ('𝟬', 0),
    ('𝟭', 1),
    ('𝟮', 2),
    ('𝟯', 3),
    ('𝟰', 4),
    ('𝟱', 5),
    ('𝟲', 6),
    ('𝟳', 7),
    ('𝟴', 8),
    ('𝟵', 9),
    ('𝟶', 0),
    ('𝟷', 1),
    ('𝟸', 2),
    ('𝟹', 3),
    ('𝟺', 4),
    ('𝟻', 5),
    ('𝟼', 6),
    ('𝟽', 7),
    ('𝟾', 8),
    ('𝟿', 9),
    ('𞅀', 0),
    ('𞅁', 1),
    ('𞅂', 2),
    ('𞅃', 3),
    ('𞅄', 4),
    ('𞅅', 5),
    ('𞅆', 6),
    ('𞅇', 7),
    ('𞅈', 8),
    ('𞅉', 9),
    ('𞋰', 0),
    ('𞋱', 1),
    ('𞋲', 2),
    ('𞋳', 3),
    ('𞋴', 4),
    ('𞋵', 5),
    ('𞋶', 6),
    ('𞋷', 7),
    ('𞋸', 8),
    ('𞋹', 9),
    ('𞥐', 0),
    ('𞥑', 1),
    ('𞥒', 2),
    ('𞥓', 3),
    ('𞥔', 4),
    ('𞥕', 5),
    ('𞥖', 6),
    ('𞥗', 7),
    ('𞥘', 8),
    ('𞥙', 9),
    ('🯰', 0),
    ('🯱', 1),
    ('🯲', 2),
    ('🯳', 3),
    ('🯴', 4),
    ('🯵', 5),
    ('🯶', 6),
    ('🯷', 7),
    ('🯸', 8),
    ('🯹', 9),
];

#[derive(Clone)]
pub struct NumberHandler {
    digit_re: Regex,
    multi_digit_re: Regex,
    pure_digit_re: Regex,
    digit_map: HashMap<char, char>,
}

impl NumberHandler {
    pub fn new() -> Self {
        let digit_map = DIGIT_MAPPING
            .iter()
            .map(|&(c, i)| (c, char::from_digit(i, 10).unwrap()))
            .collect();
        Self {
            digit_re: Regex::new(r"\d").unwrap(),
            multi_digit_re: Regex::new(r"\d[\d.,]+").unwrap(),
            pure_digit_re: Regex::new(r"\d+").unwrap(),
            digit_map,
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
    pub fn digit_freq(&self, text: &str) -> Float {
        let mut freq = 1.;
        for m in self.multi_digit_re.find_iter(text) {
            for sm in self.pure_digit_re.find_iter(m.as_str()) {
                let sm = self.standardize_digits(sm.as_str());
                if sm.len() == 4 {
                    freq *= self.year_freq(sm.as_str());
                } else {
                    freq *= self.benford_freq(sm.as_str());
                }
            }
        }
        freq
    }

    fn standardize_digits(&self, text: &str) -> String {
        let mut result = String::new();
        for c in text.chars() {
            result.push(self.digit_map[&c]);
        }
        result
    }

    /// Estimate the frequency of a digit sequence according to Benford's law.
    fn benford_freq(&self, text: &str) -> Float {
        debug_assert_ne!(text.len(), 0);
        let first_digit = text.chars().next().unwrap().to_digit(10).unwrap() as usize;
        DIGIT_FREQS[first_digit] / FLOAT_10.powi(text.len() as i32 - 1)
    }

    /// Estimate the relative frequency of a particular 4-digit sequence representing
    /// a year.
    ///
    /// For example, suppose text == "1985". We're estimating the probability that a
    /// randomly-selected token from a large corpus will be "1985" and refer to the
    /// year, _given_ that it is 4 digits. Tokens that are not 4 digits are not involved
    /// in the probability distribution.
    fn year_freq(&self, text: &str) -> Float {
        debug_assert_eq!(text.len(), 4);
        let year = text.parse::<Float>().unwrap();

        let year_log_freq = if year <= REFERENCE_YEAR {
            // Fitting a line to the curve seen at
            // https://twitter.com/r_speer/status/1493715982887571456.
            FLOAT_0_0083.mul_add(-REFERENCE_YEAR + year, YEAR_LOG_PEAK)
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
            FLOAT_0_2.mul_add(-year + (REFERENCE_YEAR + PLATEAU_WIDTH), YEAR_LOG_PEAK)
        };

        let year_prob = FLOAT_10.powf(year_log_freq);

        // If this token _doesn't_ represent a year, then use the Benford frequency
        // distribution.
        let not_year_prob = NOT_YEAR_PROB * self.benford_freq(text);
        year_prob + not_year_prob
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_relative_eq;

    #[test]
    fn test_smash_numbers() {
        let handler: NumberHandler = NumberHandler::new();
        assert_eq!(handler.smash_numbers("33.4"), "00.0");
        assert_eq!(handler.smash_numbers("33-4"), "00-4");
        assert_eq!(handler.smash_numbers("三三.四"), "三三.四");
    }

    #[test]
    fn test_digit_freq() {
        let handler: NumberHandler = NumberHandler::new();
        assert_relative_eq!(handler.digit_freq("1991.08.07"), 5.7467896897867986e-09);
        assert_relative_eq!(handler.digit_freq("1991年08月07日"), 5.7467896897867986e-09);
        assert_relative_eq!(handler.digit_freq("平成三年八月七日"), 1.0);
        assert_relative_eq!(
            handler.digit_freq("１９９１.０８.０７"),
            5.7467896897867986e-09
        );
    }

    #[test]
    fn test_benford_freq() {
        let handler: NumberHandler = NumberHandler::new();
        assert_relative_eq!(handler.benford_freq("7"), 0.057);
        assert_relative_eq!(handler.benford_freq("07"), 0.0009);
        assert_relative_eq!(handler.benford_freq("007"), 8.999999999999999e-05);
    }

    #[test]
    fn test_year_freq() {
        let handler: NumberHandler = NumberHandler::new();
        assert_relative_eq!(handler.year_freq("1992"), 0.007231119202497894);
        assert_relative_eq!(handler.year_freq("2023"), 0.012081740881970011);
        assert_relative_eq!(handler.year_freq("0000"), 9.000000000002107e-07);
        assert_relative_eq!(handler.year_freq("9999"), 4.5e-06);
    }
}
