from glob import glob
import re

filenames = glob('resources/*.txt')
filenames.sort()

matcher = re.compile(r'^resources\/(?P<wordlist>.+?)_(?P<lang>.+?)\.txt$')

targets = []
for filename in filenames:
    m = matcher.match(filename)
    wordlist = m.group('wordlist')
    lang = m.group('lang')
    targets.append((wordlist, lang))

with open('features.txt', 'wt') as f:
    for wordlist, lang in targets:
        f.write(f'{wordlist}-{lang} = {[]}\n')

with open('build-rs.txt', 'wt') as f:
    for wordlist, lang in targets:
        f.write(f'\t#[cfg(feature = "{wordlist}-{lang}")]\n')
        f.write(f'\tbuild("{wordlist}_{lang}")?;\n\n')

with open('lib-rs.txt', 'wt') as f:
    f.write('use std::env;\n\n')
    f.write('use anyhow::{anyhow, Result};\n')
    f.write('use wordfreq_core::WordFreq;\n\n')
    for wordlist, lang in targets:
        f.write(f'#[cfg(feature = "{wordlist}-{lang}")]\n')
        f.write(f'const DATA_{wordlist.upper()}_{lang.upper()}: &\'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/{wordlist}_{lang}.bin"));\n')
    f.write('\n')
    f.write('pub fn load_wordfreq(lang: &str, wordlist: &str) -> Result<WordFreq> {\n')
    for wordlist, lang in targets:
        f.write(f'\t#[cfg(feature = "{wordlist}-{lang}")]\n')
        f.write(f'\tif lang == "{lang}" && wordlist == "{wordlist}" {{\n')
        f.write(f'\t\treturn Ok(WordFreq::deserialize(DATA_{wordlist.upper()}_{lang.upper()})?);\n')
        f.write('\t}\n')
    f.write(f'\tErr(anyhow!("Unknown language or wordlist: {{lang}}-{{wordlist}}"))\n')
    f.write('}\n')
