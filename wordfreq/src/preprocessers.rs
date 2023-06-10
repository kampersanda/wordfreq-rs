// Copyright 2022 Robyn Speer
// Copyright 2023 Shunsuke Kanda
//
// The code is a port from
//  - https://github.com/rspeer/wordfreq/blob/v3.0.2/wordfreq/preprocess.py and
//  - https://github.com/rspeer/wordfreq/blob/v3.0.2/wordfreq/language_info.py,
// together with the comments, following the MIT-license.
//! Preprocessers in multiple languages.

use anyhow::{anyhow, Result};
use language_tags::LanguageTag;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::chinese::ChineseSimplifier;
use crate::language;
use crate::transliterate::Transliterater;
use crate::transliterate::Transliteration;

const LATIN_SMALL_LETTER_S_WITH_COMMA_BELOW: &str = "ș";
const LATIN_SMALL_LETTER_S_WITH_CEDILLA: &str = "ş";
const LATIN_SMALL_LETTER_T_WITH_COMMA_BELOW: &str = "ț";
const LATIN_SMALL_LETTER_T_WITH_CEDILLA: &str = "ţ";

#[derive(Clone)]
enum NormalForm {
    NFC,
    NFKC,
}

#[derive(Clone)]
enum DiacriticsUnder {
    Cedillas,
    Commas,
    None,
}

/// This function applies pre-processing steps that convert forms of words
/// considered equivalent into one standardized form.
///
/// As one straightforward step, it case-folds the text. For the purposes of
/// wordfreq and related tools, a capitalized word shouldn't have a different
/// frequency from its lowercase version.
///
/// The steps that are applied in order, only some of which apply to each
/// language, are:
///
/// - [NFC or NFKC normalization, as needed for the language](#unicode-normalization)
/// - [Transliteration of multi-script languages](#transliteration-of-multi-script-languages)
/// - [Abjad mark removal](#abjad-mark-removal)
/// - [Case folding](#case-folding)
/// - [Fixing of diacritics](#fixing-of-diacritics)
///
/// We'll describe these steps out of order, to start with the more obvious
/// steps.
///
/// # Case folding
///
/// The most common effect of this function is that it case-folds alphabetic
/// text to lowercase:
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("en").unwrap();
/// assert_eq!(standardizer.apply("Word"), "word");
/// ```
///
/// This is proper Unicode-aware case-folding, so it eliminates distinctions
/// in lowercase letters that would not appear in uppercase. This accounts for
/// the German ß and the Greek final sigma:
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("de").unwrap();
/// assert_eq!(standardizer.apply("groß"), "gross");
/// ```
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("el").unwrap();
/// assert_eq!(standardizer.apply("λέξις"), "λέξισ");
/// ```
///
/// In Turkish (and Azerbaijani), case-folding is different, because the
/// uppercase and lowercase I come in two variants, one with a dot and one
/// without. They are matched in a way that preserves the number of dots, which
/// the usual pair of "I" and "i" do not.
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("tr").unwrap();
/// assert_eq!(standardizer.apply("HAKKINDA İSTANBUL"), "hakkında istanbul");
/// ```
///
/// # Fixing of diacritics
///
/// While we're talking about Turkish: the Turkish alphabet contains letters
/// with cedillas attached to the bottom. In the case of "ş" and "ţ", these
/// letters are very similar to two Romanian letters, "ș" and "ț", which have
/// separate _commas_ below them.
///
/// (Did you know that a cedilla is not the same as a comma under a letter? I
/// didn't until I started dealing with text normalization. My keyboard layout
/// even inputs a letter with a cedilla when you hit Compose+comma.)
///
/// Because these letters look so similar, and because some fonts only include
/// one pair of letters and not the other, there are many cases where the
/// letters are confused with each other. Our preprocessing normalizes these
/// Turkish and Romanian letters to the letters each language prefers.
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("tr").unwrap();
/// assert_eq!(standardizer.apply("kișinin"), "kişinin");
/// ```
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("ro").unwrap();
/// assert_eq!(standardizer.apply("ACELAŞI"), "același");
/// ```
///
/// # Unicode normalization
///
/// Unicode text is NFC normalized in most languages, removing trivial
/// distinctions between strings that should be considered equivalent in all
/// cases:
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("de").unwrap();
/// let word = standardizer.apply("natu\u{0308}rlich");
/// assert!(word.contains("ü"));
/// ```
///
/// NFC normalization is sufficient (and NFKC normalization is a bit too strong)
/// for many languages that are written in cased, alphabetic scripts.
/// Languages in other scripts tend to need stronger normalization to properly
/// compare text. So we use NFC normalization when the language's script is
/// Latin, Greek, or Cyrillic, and we use NFKC normalization for all other
/// languages.
///
/// Here's an example in Japanese, where preprocessing changes the width (and
/// the case) of a Latin letter that's used as part of a word:
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("ja").unwrap();
/// assert_eq!(standardizer.apply("Ｕターン"), "uターン");
/// ```
///
/// In Korean, NFKC normalization is important because it aligns two different
/// ways of encoding text -- as individual letters that are grouped together
/// into square characters, or as the entire syllables that those characters
/// represent:
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("ko").unwrap();
/// let word = "\u{1102}\u{1161}\u{11c0}\u{1106}\u{1161}\u{11af}";
/// assert_eq!(word, "낱말");
/// assert_eq!(word.chars().count(), 6);
/// let word = standardizer.apply(word);
/// assert_eq!(word, "낱말");
/// assert_eq!(word.chars().count(), 2);
/// ```
///
/// # Abjad mark removal
///
/// There are many abjad languages, such as Arabic, Hebrew, Persian, and Urdu,
/// where words can be marked with vowel points but rarely are. In languages
/// that use abjad scripts, we remove all modifiers that are classified by
/// Unicode as "marks". We also remove an Arabic character called the tatweel,
/// which is used to visually lengthen a word.
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("ar").unwrap();
/// assert_eq!(standardizer.apply("كَلِمَة"), "كلمة");
/// ```
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("ar").unwrap();
/// assert_eq!(standardizer.apply("الحمــــــد"), "الحمد");
/// ```
///
/// # Transliteration of multi-script languages
///
/// Some languages are written in multiple scripts, and require special care.
/// These languages include Chinese, Serbian, and Azerbaijani.
///
/// In Serbian, there is a well-established mapping from Cyrillic letters to
/// Latin letters. We apply this mapping so that Serbian is always represented
/// in Latin letters.
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("sr").unwrap();
/// assert_eq!(standardizer.apply("схваташ"), "shvataš");
/// ```
///
/// The transliteration is more complete than it needs to be to cover just
/// Serbian, so that -- for example -- borrowings from Russian can be
/// transliterated, instead of coming out in a mixed script.
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("sr").unwrap();
/// assert_eq!(standardizer.apply("культуры"), "kul'tury");
/// ```
///
/// Azerbaijani (Azeri) has a similar transliteration step to Serbian,
/// and then the Latin-alphabet text is handled similarly to Turkish.
///
/// ```
/// use wordfreq::Standardizer;
/// let standardizer = Standardizer::new("az").unwrap();
/// assert_eq!(standardizer.apply("бағырты"), "bağırtı");
/// ```
///
/// We don't transliterate Traditional to Simplified Chinese in this step.
/// There are some steps where we unify them internally: see chinese.py
/// for more information.
#[derive(Clone)]
pub struct Standardizer {
    normal_form: NormalForm,
    mark_re: Option<Regex>,
    dotless_i: bool,
    diacritics_under: DiacriticsUnder,
    transliterater: Option<Transliterater>,
    chinese_simplifier: Option<ChineseSimplifier>,
}

impl Standardizer {
    /// Creates a new Standardizer for the given language.
    ///
    /// # Arguments
    ///
    /// - `language_tag`: Language tag, which should be one of left keys in [`language::LIKELY_SUBTAGS`].
    pub fn new(language_tag: &str) -> Result<Self> {
        let language_tag = language::maximize_subtag(language_tag).ok_or(anyhow!(
            "{language_tag} is an unexpected language tag. You must input a language tag defined in left keys of wordfreq::language::LIKELY_SUBTAGS."
        ))?;
        let parsed = LanguageTag::parse(language_tag).unwrap();
        let script = parsed.script().unwrap();
        let primary_language = parsed.primary_language();

        let normal_form = if ["Latn", "Grek", "Cyrl"].contains(&script) {
            NormalForm::NFC
        } else {
            NormalForm::NFKC
        };

        // \p{} construct in regex is used to match a Unicode character property.
        // Mn stands for "Nonspacing Mark". \u{0640} is the Arabic Tatweel character (ـ).
        let mark_re = if ["Arab", "Hebr"].contains(&script) {
            Some(Regex::new(r"[\p{Mn}\u{0640}]").unwrap())
        } else {
            None
        };

        let (dotless_i, diacritics_under) = if ["tr", "az", "kk"].contains(&primary_language) {
            (true, DiacriticsUnder::Cedillas)
        } else if ["ro"].contains(&primary_language) {
            (false, DiacriticsUnder::Commas)
        } else {
            (false, DiacriticsUnder::None)
        };

        let transliterater = if "sr" == primary_language {
            Some(Transliterater::new(Transliteration::SrLatn))
        } else if "az" == primary_language {
            Some(Transliterater::new(Transliteration::AzLatn))
        } else {
            None
        };

        let chinese_simplifier = if "zh" == primary_language && "Hant" == script {
            Some(ChineseSimplifier::new())
        } else {
            None
        };

        Ok(Self {
            normal_form,
            mark_re,
            dotless_i,
            diacritics_under,
            transliterater,
            chinese_simplifier,
        })
    }

    /// Standardizes the given text.
    pub fn apply(&self, text: &str) -> String {
        // NFC or NFKC normalization, as needed for the language
        let text = match self.normal_form {
            NormalForm::NFC => text.nfc().collect::<String>(),
            NormalForm::NFKC => text.nfkc().collect::<String>(),
        };

        // Transliteration of multi-script languages
        let text = if let Some(transliterater) = self.transliterater.as_ref() {
            transliterater.transliterate(&text)
        } else {
            text
        };

        // Removes decorations from words in abjad scripts:
        //
        // - Combining marks of class Mn, which tend to represent non-essential
        //   vowel markings.
        // - Tatweels, horizontal segments that are used to extend or justify an
        //   Arabic word.
        let text = if let Some(mark_re) = self.mark_re.as_ref() {
            mark_re.replace_all(&text, "").to_string()
        } else {
            text
        };

        // Case folding
        let text = if self.dotless_i {
            self.casefold_with_i_dots(&text)
        } else {
            caseless::default_case_fold_str(&text)
        };

        // Fixing of diacritics
        let text = match self.diacritics_under {
            DiacriticsUnder::Cedillas => self.commas_to_cedillas(&text),
            DiacriticsUnder::Commas => self.cedillas_to_commas(&text),
            DiacriticsUnder::None => text,
        };

        // Simplyfing Chinese characters
        // NOTE: This step is from lossy_tokenize() in https://github.com/rspeer/wordfreq/blob/v3.0.2/wordfreq/tokens.py.
        let text = if let Some(chinese_simplifier) = self.chinese_simplifier.as_ref() {
            chinese_simplifier.simplify(&text)
        } else {
            text
        };

        text
    }

    /// Converts capital I's and capital dotted İ's to lowercase in the way
    /// that's appropriate for Turkish and related languages, then case-fold
    /// the rest of the letters.
    fn casefold_with_i_dots(&self, text: &str) -> String {
        let text = text.nfc().collect::<String>();
        let text = text.replace("İ", "i").replace("I", "ı");
        caseless::default_case_fold_str(&text)
    }

    /// Converts s and t with commas (ș and ț) to cedillas (ş and ţ), which is
    /// preferred in Turkish.
    ///
    /// Only the lowercase versions are replaced, because this assumes the
    /// text has already been case-folded.
    fn commas_to_cedillas(&self, text: &str) -> String {
        text.replace(
            LATIN_SMALL_LETTER_S_WITH_COMMA_BELOW,
            LATIN_SMALL_LETTER_S_WITH_CEDILLA,
        )
        .replace(
            LATIN_SMALL_LETTER_T_WITH_COMMA_BELOW,
            LATIN_SMALL_LETTER_T_WITH_CEDILLA,
        )
    }

    /// Converts s and t with cedillas (ş and ţ) to commas (ș and ț), which is
    /// preferred in Romanian.
    ///
    /// Only the lowercase versions are replaced, because this assumes the
    /// ext has already been case-folded.
    fn cedillas_to_commas(&self, text: &str) -> String {
        text.replace(
            LATIN_SMALL_LETTER_S_WITH_CEDILLA,
            LATIN_SMALL_LETTER_S_WITH_COMMA_BELOW,
        )
        .replace(
            LATIN_SMALL_LETTER_T_WITH_CEDILLA,
            LATIN_SMALL_LETTER_T_WITH_COMMA_BELOW,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_langtag_parse() {
        for &(_, subtag) in language::LIKELY_SUBTAGS {
            let langtag = LanguageTag::parse(subtag).unwrap();
            assert!(langtag.script().is_some());
        }
    }
}
