use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use std::env;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write};
    use std::path::Path;

    use wordfreq::WordFreq;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Directory path for build package
    let build_dir = env::var_os("OUT_DIR").unwrap();
    println!("build_dir: {:?}", build_dir);

    // Resources directory
    let resources_dir_path = Path::new("resources");

    // Dictionary file name
    let txt_file_name = "small_en.txt";

    // Source dictionary file path
    let source_txt_file_path = resources_dir_path.join(txt_file_name);

    let reader = BufReader::new(File::open(source_txt_file_path)?);
    let wf = WordFreq::new(wordfreq::word_weights_from_text(reader)?);

    let model = wf.serialize()?;

    let output_path = Path::new(&build_dir).join("small_en.bin");
    let mut writer = BufWriter::new(File::create(output_path)?);
    writer.write_all(&model)?;

    Ok(())
}
