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

cargo_toml = '''[package]
name = "wordfreq-data"
version = "0.1.0"
edition = "2021"

[features]
default = ["large-en"]

{features_block}

[dependencies]
anyhow = "1.0"
wordfreq-core = {{ path = "../wordfreq-core" }}

[build-dependencies]
wordfreq-core = {{ path = "../wordfreq-core" }}
'''

with open('Cargo.toml', 'wt') as f:
    features_block = []
    for wordlist, lang in targets:
        features_block.append(f'{wordlist}-{lang} = []')
    f.write(cargo_toml.format(features_block='\n'.join(features_block)))

build_rs = '''use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{{BufReader, BufWriter, Write}};
use std::path::Path;

use wordfreq_core::WordFreq;

fn build(file_base: &str) -> Result<(), Box<dyn Error>> {{
    let build_dir = env::var_os("OUT_DIR").unwrap();

    let resources_dir_path = Path::new("resources");
    let input_file_path = resources_dir_path.join(file_base).with_extension("txt");

    let reader = BufReader::new(File::open(input_file_path)?);
    let wf = WordFreq::new(wordfreq_core::word_weights_from_text(reader)?);
    let model = wf.serialize()?;

    let output_file_path = Path::new(&build_dir).join(file_base).with_extension("bin");
    let mut writer = BufWriter::new(File::create(output_file_path)?);
    writer.write_all(&model)?;

    Ok(())
}}

fn main() -> Result<(), Box<dyn Error>> {{
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

{main_block}

    Ok(())
}}
'''

with open('build.rs', 'wt') as f:
    main_block = []
    for wordlist, lang in targets:
        main_block.append(f'\t#[cfg(feature = "{wordlist}-{lang}")]')
        main_block.append(f'\tbuild("{wordlist}_{lang}")?;')
    f.write(build_rs.format(main_block='\n'.join(main_block)))

lib_rs = '''use std::env;

use anyhow::{{anyhow, Result}};
use wordfreq_core::WordFreq;

{const_block}

pub fn load_wordfreq(lang: &str, wordlist: &str) -> Result<WordFreq> {{
{if_block}
\tErr(anyhow!("Unknown language or wordlist: {{lang}}-{{wordlist}}"))
}}
'''

with open('src/lib.rs', 'wt') as f:
    const_block = []
    for wordlist, lang in targets:
        const_block.append(f'#[cfg(feature = "{wordlist}-{lang}")]')
        const_block.append(f'const DATA_{wordlist.upper()}_{lang.upper()}: &\'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/{wordlist}_{lang}.bin"));')
    if_block = []
    for wordlist, lang in targets:
        if_block.append(f'\t#[cfg(feature = "{wordlist}-{lang}")]')
        if_block.append(f'\tif lang == "{lang}" && wordlist == "{wordlist}" {{')
        if_block.append(f'\t\treturn Ok(WordFreq::deserialize(DATA_{wordlist.upper()}_{lang.upper()})?);')
        if_block.append('\t}')
    f.write(lib_rs.format(const_block='\n'.join(const_block), if_block='\n'.join(if_block)))
