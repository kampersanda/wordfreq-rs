//!

use anyhow::Result;
use language_tags::LanguageTag;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::transliterate::Transliterater;
use crate::transliterate::Transliteration;

const LATIN_SMALL_LETTER_S_WITH_COMMA_BELOW: &str = "ș";
const LATIN_SMALL_LETTER_S_WITH_CEDILLA: &str = "ş";
const LATIN_SMALL_LETTER_T_WITH_COMMA_BELOW: &str = "ț";
const LATIN_SMALL_LETTER_T_WITH_CEDILLA: &str = "ţ";

enum NormalForm {
    NFC,
    NFKC,
}

enum DiacriticsUnder {
    Cedillas,
    Commas,
    None,
}

/// https://docs.rs/language-tags/
/// https://github.com/rspeer/wordfreq/blob/master/wordfreq/language_info.py
pub struct Preprocesser {
    normal_form: NormalForm,
    mark_re: Option<Regex>,
    dotless_i: bool,
    diacritics_under: DiacriticsUnder,
    transliterater: Option<Transliterater>,
}

impl Preprocesser {
    /// https://github.com/rspeer/langcodes/blob/49beea8e20ae26c2dead7bd77f41cfba0e0ab533/langcodes/__init__.py#L182
    ///
    /// 'tokenizer' and 'lookup_transliteration' are not implemented.
    pub fn new(langcode: &str) -> Result<Self> {
        let langtag = LanguageTag::parse(langcode)?;
        let script = langtag.script().unwrap();
        let primary_language = langtag.primary_language();

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

        let transliterater = if ["sr"].contains(&primary_language) {
            Some(Transliterater::new(Transliteration::SrLatn))
        } else if ["az"].contains(&primary_language) {
            Some(Transliterater::new(Transliteration::AzLatn))
        } else {
            None
        };

        Ok(Self {
            normal_form,
            mark_re,
            dotless_i,
            diacritics_under,
            transliterater,
        })
    }

    ///
    pub fn normalize(&self, text: &str) -> String {
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
        text
    }

    /// Converts capital I's and capital dotted İ's to lowercase in the way
    /// that's appropriate for Turkish and related languages, then case-fold
    /// the rest of the letters.
    pub fn casefold_with_i_dots(&self, text: &str) -> String {
        let text = text.nfc().collect::<String>();
        let text = text.replace("İ", "i").replace("I", "ı");
        caseless::default_case_fold_str(&text)
    }

    /// Converts s and t with commas (ș and ț) to cedillas (ş and ţ), which is
    /// preferred in Turkish.
    ///
    /// Only the lowercase versions are replaced, because this assumes the
    /// text has already been case-folded.
    pub fn commas_to_cedillas(&self, text: &str) -> String {
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
    pub fn cedillas_to_commas(&self, text: &str) -> String {
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
