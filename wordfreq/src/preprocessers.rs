//!

use anyhow::Result;
use language_tags::LanguageTag;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

enum NormalForm {
    NFC,
    NFKC,
}

enum DiacriticsUnder {
    Cedillas,
    Commas,
    None,
}

enum Transliteration {
    SrLatn,
    AzLatn,
    None,
}

/// https://docs.rs/language-tags/
/// https://github.com/rspeer/wordfreq/blob/master/wordfreq/language_info.py
pub struct Preprocesser {
    // BCP 47 script code
    script: String,
    normal_form: NormalForm,
    remove_marks: bool,
    mark_re: Regex,
    dotless_i: bool,
    diacritics_under: DiacriticsUnder,
    transliteration: Transliteration,
}

impl Preprocesser {
    /// https://github.com/rspeer/langcodes/blob/49beea8e20ae26c2dead7bd77f41cfba0e0ab533/langcodes/__init__.py#L182
    pub fn new(langcode: &str) -> Result<Self> {
        let langtag = LanguageTag::parse(langcode)?;
        let script = langtag.script().unwrap();
        let primary_language = langtag.primary_language();

        let normal_form = if ["Latn", "Grek", "Cyrl"].contains(&script) {
            NormalForm::NFC
        } else {
            NormalForm::NFKC
        };
        let remove_marks = ["Arab", "Hebr"].contains(&script);
        let mark_re = Regex::new(r"[\p{Mn}\N{ARABIC TATWEEL}]").unwrap();

        let (dotless_i, diacritics_under) = if ["tr", "az", "kk"].contains(&primary_language) {
            (true, DiacriticsUnder::Cedillas)
        } else if ["ro"].contains(&primary_language) {
            (false, DiacriticsUnder::Commas)
        } else {
            (false, DiacriticsUnder::None)
        };

        let transliteration = if ["sr"].contains(&primary_language) {
            Transliteration::SrLatn
        } else if ["az"].contains(&primary_language) {
            Transliteration::AzLatn
        } else {
            Transliteration::None
        };

        Ok(Self {
            script: script.to_string(),
            normal_form,
            remove_marks,
            mark_re,
            dotless_i,
            diacritics_under,
            transliteration,
        })
    }

    ///
    pub fn normalize(&self, text: &str) -> String {
        let text = self.unicode_normalize(text);

        // Abjad mark removal
        let text = if self.remove_marks {
            self.remove_marks(&text)
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

    ///
    pub fn unicode_normalize(&self, text: &str) -> String {
        match self.normal_form {
            NormalForm::NFC => text.nfc().collect::<String>(),
            NormalForm::NFKC => text.nfkc().collect::<String>(),
        }
    }

    ///
    pub fn remove_marks(&self, text: &str) -> String {
        self.mark_re.replace_all(text, "").to_string()
    }

    /// Converts capital I's and capital dotted İ's to lowercase in the way
    /// that's appropriate for Turkish and related languages, then case-fold
    /// the rest of the letters.
    pub fn casefold_with_i_dots(&self, text: &str) -> String {
        let text = text.nfc().collect::<String>();
        let text = text.replace("İ", "i").replace("I", "ı");
        caseless::default_case_fold_str(&text)
    }

    ///
    pub fn commas_to_cedillas(&self, text: &str) -> String {
        text.replace(
            r"\N{LATIN SMALL LETTER S WITH COMMA BELOW}",
            r"\N{LATIN SMALL LETTER S WITH CEDILLA}",
        )
        .replace(
            r"\N{LATIN SMALL LETTER T WITH COMMA BELOW}",
            r"\N{LATIN SMALL LETTER T WITH CEDILLA}",
        )
    }

    ///
    pub fn cedillas_to_commas(&self, text: &str) -> String {
        text.replace(
            r"\N{LATIN SMALL LETTER S WITH CEDILLA}",
            r"\N{LATIN SMALL LETTER S WITH COMMA BELOW}",
        )
        .replace(
            r"\N{LATIN SMALL LETTER T WITH CEDILLA}",
            r"\N{LATIN SMALL LETTER T WITH COMMA BELOW}",
        )
    }
}
