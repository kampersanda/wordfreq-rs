use hashbrown::HashMap;

const SR_LATN_TABLE: &[(char, &str)] = &[
    ('А', "A"),
    ('а', "a"),
    ('Б', "B"),
    ('б', "b"),
    ('В', "V"),
    ('в', "v"),
    ('Г', "G"),
    ('г', "g"),
    ('Д', "D"),
    ('д', "d"),
    ('Ђ', "Đ"),
    ('ђ', "đ"),
    ('Е', "E"),
    ('е', "e"),
    ('Ж', "Ž"),
    ('ж', "ž"),
    ('З', "Z"),
    ('з', "z"),
    ('И', "I"),
    ('и', "i"),
    ('Ј', "J"),
    ('ј', "j"),
    ('К', "K"),
    ('к', "k"),
    ('Л', "L"),
    ('л', "l"),
    ('Љ', "Lj"),
    ('љ', "lj"),
    ('М', "M"),
    ('м', "m"),
    ('Н', "N"),
    ('н', "n"),
    ('Њ', "Nj"),
    ('њ', "nj"),
    ('О', "O"),
    ('о', "o"),
    ('П', "P"),
    ('п', "p"),
    ('Р', "R"),
    ('р', "r"),
    ('С', "S"),
    ('с', "s"),
    ('Т', "T"),
    ('т', "t"),
    ('Ћ', "Ć"),
    ('ћ', "ć"),
    ('У', "U"),
    ('у', "u"),
    ('Ф', "F"),
    ('ф', "f"),
    ('Х', "H"),
    ('х', "h"),
    ('Ц', "C"),
    ('ц', "c"),
    ('Ч', "Č"),
    ('ч', "č"),
    ('Џ', "Dž"),
    ('џ', "dž"),
    ('Ш', "Š"),
    ('ш', "š"),
    //
    // Handle Cyrillic letters from other languages. We hope these cases don't
    // come up often when we're trying to transliterate Serbian, but if these
    // letters show up in loan-words or code-switching text, we can at least
    // transliterate them approximately instead of leaving them as Cyrillic
    // letters surrounded by Latin.
    //
    // Russian letters
    ('Ё', "Jo"),
    ('ё', "jo"),
    ('Й', "J"),
    ('й', "j"),
    ('Щ', "Šč"),
    ('щ', "šč"),
    ('Ъ', ""),
    ('ъ', ""),
    ('Ы', "Y"),
    ('ы', "y"),
    ('Ь', "'"),
    ('ь', "'"),
    ('Э', "E"),
    ('э', "e"),
    ('Ю', "Ju"),
    ('ю', "ju"),
    ('Я', "Ja"),
    ('я', "ja"),
    // Belarusian letter
    ('Ў', "Ŭ"),
    ('ў', "ŭ"),
    // Ukrainian letters
    ('Є', "Je"),
    ('є', "je"),
    ('І', "I"),
    ('і', "i"),
    ('Ї', "Ï"),
    ('ї', "ï"),
    ('Ґ', "G"),
    ('ґ', "g"),
    // Macedonian letters
    ('Ѕ', "Dz"),
    ('ѕ', "dz"),
    ('Ѓ', "Ǵ"),
    ('ѓ', "ǵ"),
    ('Ќ', "Ḱ"),
    ('ќ', "ḱ"),
];

const AZ_LATN_TABLE: &[(char, &str)] = &[
    ('А', "A"),
    ('а', "a"),
    ('Б', "B"),
    ('б', "b"),
    ('В', "V"),
    ('в', "v"),
    ('Г', "Q"),
    ('г', "q"),
    ('Д', "D"),
    ('д', "d"),
    ('Ђ', "Đ"),
    ('ђ', "đ"),
    ('Е', "E"),
    ('е', "e"),
    ('Ж', "J"),
    ('ж', "j"),
    ('З', "Z"),
    ('з', "z"),
    ('И', "İ"),
    ('и', "i"),
    ('Ј', "Y"),
    ('ј', "y"),
    ('К', "K"),
    ('к', "k"),
    ('Л', "L"),
    ('л', "l"),
    ('Љ', "Lj"),
    ('љ', "lj"),
    ('М', "M"),
    ('м', "m"),
    ('Н', "N"),
    ('н', "n"),
    ('Њ', "Nj"),
    ('њ', "nj"),
    ('О', "O"),
    ('о', "o"),
    ('П', "P"),
    ('п', "p"),
    ('Р', "R"),
    ('р', "r"),
    ('С', "S"),
    ('с', "s"),
    ('Т', "T"),
    ('т', "t"),
    ('Ћ', "Ć"),
    ('ћ', "ć"),
    ('У', "U"),
    ('у', "u"),
    ('Ф', "F"),
    ('ф', "f"),
    ('Х', "X"),
    ('х', "x"),
    ('Ц', "C"),
    ('ц', "c"),
    ('Ч', "Ç"),
    ('ч', "ç"),
    ('Џ', "Dž"),
    ('џ', "dž"),
    ('Ш', "Ş"),
    ('ш', "ş"),
    ('Ё', "Jo"),
    ('ё', "jo"),
    ('Й', "J"),
    ('й', "j"),
    ('Щ', "Šč"),
    ('щ', "šč"),
    ('Ъ', ""),
    ('ъ', ""),
    ('Ы', "I"),
    ('ы', "ı"),
    ('Ь', "'"),
    ('ь', "'"),
    ('Э', "E"),
    ('э', "e"),
    ('Ю', "Ju"),
    ('ю', "ju"),
    ('Я', "Ja"),
    ('я', "ja"),
    ('Ў', "Ŭ"),
    ('ў', "ŭ"),
    ('Є', "Je"),
    ('є', "je"),
    ('І', "I"),
    ('і', "i"),
    ('Ї', "Ï"),
    ('ї', "ï"),
    ('Ґ', "G"),
    ('ґ', "g"),
    ('Ѕ', "Dz"),
    ('ѕ', "dz"),
    ('Ѓ', "Ǵ"),
    ('ѓ', "ǵ"),
    ('Ќ', "Ḱ"),
    ('ќ', "ḱ"),
    // Distinct Azerbaijani letters
    ('Ҹ', "C"),
    ('ҹ', "c"),
    ('Ә', "Ə"),
    ('ә', "ə"),
    ('Ғ', "Ğ"),
    ('ғ', "ğ"),
    ('Һ', "H"),
    ('һ', "h"),
    ('Ө', "Ö"),
    ('ө', "ö"),
    ('Ҝ', "G"),
    ('ҝ', "g"),
    ('Ү', "Ü"),
    ('ү', "ü"),
];

pub enum Transliteration {
    SrLatn,
    AzLatn,
}

pub struct Transliterater {
    table: HashMap<char, &'static str>,
}

impl Transliterater {
    pub fn new(tr: Transliteration) -> Self {
        match tr {
            Transliteration::SrLatn => {
                let table = SR_LATN_TABLE.iter().cloned().collect();
                Self { table }
            }
            Transliteration::AzLatn => {
                let table = AZ_LATN_TABLE.iter().cloned().collect();
                Self { table }
            }
        }
    }

    /// Applies the transliteration rules to the given text.
    pub fn transliterate(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        for c in text.chars() {
            if let Some(replacement) = self.table.get(&c) {
                result.push_str(replacement);
            } else {
                result.push(c);
            }
        }
        result
    }
}
