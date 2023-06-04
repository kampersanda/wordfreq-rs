use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use wordfreq::WordFreq;

#[allow(dead_code)]
fn build(file_base: &str) -> Result<(), Box<dyn Error>> {
    let build_dir = env::var_os("OUT_DIR").unwrap();

    let resources_dir_path = Path::new("resources");
    let input_file_path = resources_dir_path.join(file_base).with_extension("txt.gz");

    let reader = BufReader::new(GzDecoder::new(File::open(input_file_path)?));
    let wf = WordFreq::new(wordfreq::word_weights_from_text(reader)?);
    let model = wf.serialize()?;

    let output_file_path = Path::new(&build_dir).join(file_base).with_extension("bin");
    let mut writer = BufWriter::new(File::create(output_file_path)?);
    writer.write_all(&model)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    #[cfg(feature = "large-en")]
    build("large_en")?;
    #[cfg(feature = "large-fr")]
    build("large_fr")?;
    #[cfg(feature = "small-en")]
    build("small_en")?;
    #[cfg(feature = "small-ja")]
    build("small_ja")?;

    Ok(())
}
