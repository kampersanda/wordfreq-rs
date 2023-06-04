use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use wordfreq::WordFreq;

fn build(file_base: &str) -> Result<(), Box<dyn Error>> {
    let build_dir = env::var_os("OUT_DIR").unwrap();
    let wf = if file_base == "example_en" {
        let word_weight_text = "las 10\nvegas 30\n";
        WordFreq::new(wordfreq::word_weights_from_text(
            word_weight_text.as_bytes(),
        )?)
    } else {
        let file_name = Path::new(file_base).with_extension("txt.zst");
        let input_file_path = Path::new(&build_dir).join(file_name);
        if !input_file_path.exists() {
            let tmp_path = input_file_path.with_extension("download");
            let download_url = format!("https://github.com/kampersanda/wordfreq-rs/releases/download/models-v1/{file_base}.txt.zst");
            let resp = ureq::get(&download_url).call()?;
            let mut dest = File::create(&tmp_path)?;
            std::io::copy(&mut resp.into_reader(), &mut dest)?;
            dest.flush()?;
            std::fs::rename(tmp_path, &input_file_path).expect("Failed to rename temporary file");
        }
        let reader = BufReader::new(zstd::Decoder::new(File::open(input_file_path)?)?);
        WordFreq::new(wordfreq::word_weights_from_text(reader)?)
    };
    let model = wf.serialize()?;
    let output_file_path = Path::new(&build_dir).join(file_base).with_extension("bin");
    let mut writer = BufWriter::new(File::create(output_file_path)?);
    writer.write_all(&model)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    build("example_en")?;
    #[cfg(feature = "large-ar")]
    build("large_ar")?;
    #[cfg(feature = "large-bn")]
    build("large_bn")?;
    #[cfg(feature = "large-ca")]
    build("large_ca")?;
    #[cfg(feature = "large-cs")]
    build("large_cs")?;
    #[cfg(feature = "large-de")]
    build("large_de")?;
    #[cfg(feature = "large-en")]
    build("large_en")?;
    #[cfg(feature = "large-es")]
    build("large_es")?;
    #[cfg(feature = "large-fi")]
    build("large_fi")?;
    #[cfg(feature = "large-fr")]
    build("large_fr")?;
    #[cfg(feature = "large-he")]
    build("large_he")?;
    #[cfg(feature = "large-it")]
    build("large_it")?;
    #[cfg(feature = "large-ja")]
    build("large_ja")?;
    #[cfg(feature = "large-mk")]
    build("large_mk")?;
    #[cfg(feature = "large-nb")]
    build("large_nb")?;
    #[cfg(feature = "large-nl")]
    build("large_nl")?;
    #[cfg(feature = "large-pl")]
    build("large_pl")?;
    #[cfg(feature = "large-pt")]
    build("large_pt")?;
    #[cfg(feature = "large-ru")]
    build("large_ru")?;
    #[cfg(feature = "large-sv")]
    build("large_sv")?;
    #[cfg(feature = "large-uk")]
    build("large_uk")?;
    #[cfg(feature = "large-zh")]
    build("large_zh")?;
    #[cfg(feature = "small-ar")]
    build("small_ar")?;
    #[cfg(feature = "small-bg")]
    build("small_bg")?;
    #[cfg(feature = "small-bn")]
    build("small_bn")?;
    #[cfg(feature = "small-ca")]
    build("small_ca")?;
    #[cfg(feature = "small-cs")]
    build("small_cs")?;
    #[cfg(feature = "small-da")]
    build("small_da")?;
    #[cfg(feature = "small-de")]
    build("small_de")?;
    #[cfg(feature = "small-el")]
    build("small_el")?;
    #[cfg(feature = "small-en")]
    build("small_en")?;
    #[cfg(feature = "small-es")]
    build("small_es")?;
    #[cfg(feature = "small-fa")]
    build("small_fa")?;
    #[cfg(feature = "small-fi")]
    build("small_fi")?;
    #[cfg(feature = "small-fil")]
    build("small_fil")?;
    #[cfg(feature = "small-fr")]
    build("small_fr")?;
    #[cfg(feature = "small-he")]
    build("small_he")?;
    #[cfg(feature = "small-hi")]
    build("small_hi")?;
    #[cfg(feature = "small-hu")]
    build("small_hu")?;
    #[cfg(feature = "small-id")]
    build("small_id")?;
    #[cfg(feature = "small-is")]
    build("small_is")?;
    #[cfg(feature = "small-it")]
    build("small_it")?;
    #[cfg(feature = "small-ja")]
    build("small_ja")?;
    #[cfg(feature = "small-ko")]
    build("small_ko")?;
    #[cfg(feature = "small-lt")]
    build("small_lt")?;
    #[cfg(feature = "small-lv")]
    build("small_lv")?;
    #[cfg(feature = "small-mk")]
    build("small_mk")?;
    #[cfg(feature = "small-ms")]
    build("small_ms")?;
    #[cfg(feature = "small-nb")]
    build("small_nb")?;
    #[cfg(feature = "small-nl")]
    build("small_nl")?;
    #[cfg(feature = "small-pl")]
    build("small_pl")?;
    #[cfg(feature = "small-pt")]
    build("small_pt")?;
    #[cfg(feature = "small-ro")]
    build("small_ro")?;
    #[cfg(feature = "small-ru")]
    build("small_ru")?;
    #[cfg(feature = "small-sh")]
    build("small_sh")?;
    #[cfg(feature = "small-sk")]
    build("small_sk")?;
    #[cfg(feature = "small-sl")]
    build("small_sl")?;
    #[cfg(feature = "small-sv")]
    build("small_sv")?;
    #[cfg(feature = "small-ta")]
    build("small_ta")?;
    #[cfg(feature = "small-tr")]
    build("small_tr")?;
    #[cfg(feature = "small-uk")]
    build("small_uk")?;
    #[cfg(feature = "small-ur")]
    build("small_ur")?;
    #[cfg(feature = "small-vi")]
    build("small_vi")?;
    #[cfg(feature = "small-zh")]
    build("small_zh")?;

    Ok(())
}
