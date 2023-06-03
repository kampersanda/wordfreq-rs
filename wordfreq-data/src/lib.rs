use std::env;

use anyhow::Result;
use wordfreq::WordFreq;

const DATA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_en.bin"));

pub fn load_wordfreq() -> Result<WordFreq> {
    let model = WordFreq::deserialize(DATA)?;
    Ok(model)
}
