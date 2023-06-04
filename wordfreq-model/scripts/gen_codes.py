from glob import glob
import re

filenames = glob('resources/*.txt.gz')
filenames.sort()

matcher = re.compile(r'^resources\/(?P<wordlist>.+?)_(?P<lang>.+?)\.txt\.gz$')

targets = []
for filename in filenames:
    m = matcher.match(filename)
    wordlist = m.group('wordlist')
    lang = m.group('lang')
    targets.append((wordlist, lang))

#########################
# Cargo.toml
#########################

wordfreq_version = '0.1'

with open('templates/cargo_toml.txt', 'rt') as f:
    cargo_toml = f.read()

with open('Cargo.toml', 'wt') as f:
    features_block = []
    for wordlist, lang in targets:
        features_block.append(f'{wordlist}-{lang} = []')
    f.write(
        cargo_toml.format(
            features_block='\n'.join(features_block),
            wordfreq_version=wordfreq_version,
        )
    )

#########################
# build.rs
#########################

with open('templates/build_rs.txt', 'rt') as f:
    build_rs = f.read()

with open('build.rs', 'wt') as f:
    main_block = []
    for wordlist, lang in targets:
        main_block.append(f'    #[cfg(feature = "{wordlist}-{lang}")]')
        main_block.append(f'    build("{wordlist}_{lang}")?;')
    f.write(build_rs.format(main_block='\n'.join(main_block)))

#########################
# src/lib.rs
#########################

with open('templates/lib_rs.txt', 'rt') as f:
    lib_rs = f.read()

with open('src/lib.rs', 'wt') as f:
    const_block = []
    for wordlist, lang in targets:
        const_block.append(f'#[cfg(feature = "{wordlist}-{lang}")]')
        const_block.append(
            f'const DATA_{wordlist.upper()}_{lang.upper()}: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/{wordlist}_{lang}.bin"));'
        )
    model_kind_block = []
    for wordlist, lang in targets:
        model_kind_block.append(f'    #[cfg(feature = "{wordlist}-{lang}")]')
        model_kind_block.append(f'    {wordlist.capitalize()}{lang.capitalize()},')
    match_block = []
    for wordlist, lang in targets:
        match_block.append(f'        #[cfg(feature = "{wordlist}-{lang}")]')
        match_block.append(
            f'        ModelKind::{wordlist.capitalize()}{lang.capitalize()} => Ok(WordFreq::deserialize(DATA_{wordlist.upper()}_{lang.upper()})?),'
        )
    f.write(
        lib_rs.format(
            model_kind_block='\n'.join(model_kind_block),
            const_block='\n'.join(const_block),
            match_block='\n'.join(match_block),
        )
    )
