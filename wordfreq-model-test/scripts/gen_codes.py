targets = [
    ('large', 'ar'),
    ('large', 'bn'),
    ('large', 'ca'),
    ('large', 'cs'),
    ('large', 'de'),
    ('large', 'en'),
    ('large', 'es'),
    ('large', 'fi'),
    ('large', 'fr'),
    ('large', 'he'),
    ('large', 'it'),
    ('large', 'ja'),
    ('large', 'mk'),
    ('large', 'nb'),
    ('large', 'nl'),
    ('large', 'pl'),
    ('large', 'pt'),
    ('large', 'ru'),
    ('large', 'sv'),
    ('large', 'uk'),
    ('large', 'zh'),
    ('small', 'ar'),
    ('small', 'bg'),
    ('small', 'bn'),
    ('small', 'ca'),
    ('small', 'cs'),
    ('small', 'da'),
    ('small', 'de'),
    ('small', 'el'),
    ('small', 'en'),
    ('small', 'es'),
    ('small', 'fa'),
    ('small', 'fi'),
    ('small', 'fil'),
    ('small', 'fr'),
    ('small', 'he'),
    ('small', 'hi'),
    ('small', 'hu'),
    ('small', 'id'),
    ('small', 'is'),
    ('small', 'it'),
    ('small', 'ja'),
    ('small', 'ko'),
    ('small', 'lt'),
    ('small', 'lv'),
    ('small', 'mk'),
    ('small', 'ms'),
    ('small', 'nb'),
    ('small', 'nl'),
    ('small', 'pl'),
    ('small', 'pt'),
    ('small', 'ro'),
    ('small', 'ru'),
    ('small', 'sh'),
    ('small', 'sk'),
    ('small', 'sl'),
    ('small', 'sv'),
    ('small', 'ta'),
    ('small', 'tr'),
    ('small', 'uk'),
    ('small', 'ur'),
    ('small', 'vi'),
    ('small', 'zh'),
]

TAB = ' ' * 4

#########################
# Cargo.toml
#########################

with open('templates/cargo_toml.txt', 'rt') as f:
    cargo_toml = f.read()

with open('Cargo.toml', 'wt') as f:
    features_block = []
    for wordlist, lang in targets:
        features_block.append(f'{TAB}"{wordlist}-{lang}",')
    f.write(cargo_toml.format(features_block='\n'.join(features_block)))

#########################
# src/lib.rs
#########################

with open('templates/lib_rs.txt', 'rt') as f:
    lib_rs = f.read()

with open('src/lib.rs', 'wt') as f:
    test_block = []
    for wordlist, lang in targets:
        test_block.append('#[test]')
        test_block.append(f'fn test_{wordlist}_{lang}() {{')
        test_block.append(
            f'{TAB}assert!(load_wordfreq(ModelKind::{wordlist.capitalize()}{lang.capitalize()}).is_ok());'
        )
        test_block.append('}')
    f.write(lib_rs.format(test_block='\n'.join(test_block)))
