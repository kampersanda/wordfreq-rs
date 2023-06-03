use std::env;

use anyhow::{anyhow, Result};
use wordfreq_core::WordFreq;

#[cfg(feature = "large-ar")]
const DATA_LARGE_AR: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ar.bin"));
#[cfg(feature = "large-bn")]
const DATA_LARGE_BN: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_bn.bin"));
#[cfg(feature = "large-ca")]
const DATA_LARGE_CA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ca.bin"));
#[cfg(feature = "large-cs")]
const DATA_LARGE_CS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_cs.bin"));
#[cfg(feature = "large-de")]
const DATA_LARGE_DE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_de.bin"));
#[cfg(feature = "large-en")]
const DATA_LARGE_EN: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_en.bin"));
#[cfg(feature = "large-es")]
const DATA_LARGE_ES: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_es.bin"));
#[cfg(feature = "large-fi")]
const DATA_LARGE_FI: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_fi.bin"));
#[cfg(feature = "large-fr")]
const DATA_LARGE_FR: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_fr.bin"));
#[cfg(feature = "large-he")]
const DATA_LARGE_HE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_he.bin"));
#[cfg(feature = "large-it")]
const DATA_LARGE_IT: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_it.bin"));
#[cfg(feature = "large-ja")]
const DATA_LARGE_JA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ja.bin"));
#[cfg(feature = "large-mk")]
const DATA_LARGE_MK: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_mk.bin"));
#[cfg(feature = "large-nb")]
const DATA_LARGE_NB: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_nb.bin"));
#[cfg(feature = "large-nl")]
const DATA_LARGE_NL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_nl.bin"));
#[cfg(feature = "large-pl")]
const DATA_LARGE_PL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_pl.bin"));
#[cfg(feature = "large-pt")]
const DATA_LARGE_PT: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_pt.bin"));
#[cfg(feature = "large-ru")]
const DATA_LARGE_RU: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ru.bin"));
#[cfg(feature = "large-sv")]
const DATA_LARGE_SV: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_sv.bin"));
#[cfg(feature = "large-uk")]
const DATA_LARGE_UK: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_uk.bin"));
#[cfg(feature = "large-zh")]
const DATA_LARGE_ZH: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_zh.bin"));
#[cfg(feature = "small-ar")]
const DATA_SMALL_AR: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ar.bin"));
#[cfg(feature = "small-bg")]
const DATA_SMALL_BG: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_bg.bin"));
#[cfg(feature = "small-bn")]
const DATA_SMALL_BN: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_bn.bin"));
#[cfg(feature = "small-ca")]
const DATA_SMALL_CA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ca.bin"));
#[cfg(feature = "small-cs")]
const DATA_SMALL_CS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_cs.bin"));
#[cfg(feature = "small-da")]
const DATA_SMALL_DA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_da.bin"));
#[cfg(feature = "small-de")]
const DATA_SMALL_DE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_de.bin"));
#[cfg(feature = "small-el")]
const DATA_SMALL_EL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_el.bin"));
#[cfg(feature = "small-en")]
const DATA_SMALL_EN: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_en.bin"));
#[cfg(feature = "small-es")]
const DATA_SMALL_ES: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_es.bin"));
#[cfg(feature = "small-fa")]
const DATA_SMALL_FA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fa.bin"));
#[cfg(feature = "small-fi")]
const DATA_SMALL_FI: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fi.bin"));
#[cfg(feature = "small-fil")]
const DATA_SMALL_FIL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fil.bin"));
#[cfg(feature = "small-fr")]
const DATA_SMALL_FR: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fr.bin"));
#[cfg(feature = "small-he")]
const DATA_SMALL_HE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_he.bin"));
#[cfg(feature = "small-hi")]
const DATA_SMALL_HI: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_hi.bin"));
#[cfg(feature = "small-hu")]
const DATA_SMALL_HU: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_hu.bin"));
#[cfg(feature = "small-id")]
const DATA_SMALL_ID: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_id.bin"));
#[cfg(feature = "small-is")]
const DATA_SMALL_IS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_is.bin"));
#[cfg(feature = "small-it")]
const DATA_SMALL_IT: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_it.bin"));
#[cfg(feature = "small-ja")]
const DATA_SMALL_JA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ja.bin"));
#[cfg(feature = "small-ko")]
const DATA_SMALL_KO: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ko.bin"));
#[cfg(feature = "small-lt")]
const DATA_SMALL_LT: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_lt.bin"));
#[cfg(feature = "small-lv")]
const DATA_SMALL_LV: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_lv.bin"));
#[cfg(feature = "small-mk")]
const DATA_SMALL_MK: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_mk.bin"));
#[cfg(feature = "small-ms")]
const DATA_SMALL_MS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ms.bin"));
#[cfg(feature = "small-nb")]
const DATA_SMALL_NB: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_nb.bin"));
#[cfg(feature = "small-nl")]
const DATA_SMALL_NL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_nl.bin"));
#[cfg(feature = "small-pl")]
const DATA_SMALL_PL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_pl.bin"));
#[cfg(feature = "small-pt")]
const DATA_SMALL_PT: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_pt.bin"));
#[cfg(feature = "small-ro")]
const DATA_SMALL_RO: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ro.bin"));
#[cfg(feature = "small-ru")]
const DATA_SMALL_RU: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ru.bin"));
#[cfg(feature = "small-sh")]
const DATA_SMALL_SH: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sh.bin"));
#[cfg(feature = "small-sk")]
const DATA_SMALL_SK: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sk.bin"));
#[cfg(feature = "small-sl")]
const DATA_SMALL_SL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sl.bin"));
#[cfg(feature = "small-sv")]
const DATA_SMALL_SV: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sv.bin"));
#[cfg(feature = "small-ta")]
const DATA_SMALL_TA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ta.bin"));
#[cfg(feature = "small-tr")]
const DATA_SMALL_TR: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_tr.bin"));
#[cfg(feature = "small-uk")]
const DATA_SMALL_UK: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_uk.bin"));
#[cfg(feature = "small-ur")]
const DATA_SMALL_UR: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ur.bin"));
#[cfg(feature = "small-vi")]
const DATA_SMALL_VI: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_vi.bin"));
#[cfg(feature = "small-zh")]
const DATA_SMALL_ZH: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_zh.bin"));

pub fn load_wordfreq(lang: &str, wordlist: &str) -> Result<WordFreq> {
	#[cfg(feature = "large-ar")]
	if lang == "ar" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_AR)?);
	}
	#[cfg(feature = "large-bn")]
	if lang == "bn" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_BN)?);
	}
	#[cfg(feature = "large-ca")]
	if lang == "ca" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_CA)?);
	}
	#[cfg(feature = "large-cs")]
	if lang == "cs" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_CS)?);
	}
	#[cfg(feature = "large-de")]
	if lang == "de" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_DE)?);
	}
	#[cfg(feature = "large-en")]
	if lang == "en" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_EN)?);
	}
	#[cfg(feature = "large-es")]
	if lang == "es" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_ES)?);
	}
	#[cfg(feature = "large-fi")]
	if lang == "fi" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_FI)?);
	}
	#[cfg(feature = "large-fr")]
	if lang == "fr" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_FR)?);
	}
	#[cfg(feature = "large-he")]
	if lang == "he" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_HE)?);
	}
	#[cfg(feature = "large-it")]
	if lang == "it" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_IT)?);
	}
	#[cfg(feature = "large-ja")]
	if lang == "ja" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_JA)?);
	}
	#[cfg(feature = "large-mk")]
	if lang == "mk" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_MK)?);
	}
	#[cfg(feature = "large-nb")]
	if lang == "nb" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_NB)?);
	}
	#[cfg(feature = "large-nl")]
	if lang == "nl" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_NL)?);
	}
	#[cfg(feature = "large-pl")]
	if lang == "pl" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_PL)?);
	}
	#[cfg(feature = "large-pt")]
	if lang == "pt" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_PT)?);
	}
	#[cfg(feature = "large-ru")]
	if lang == "ru" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_RU)?);
	}
	#[cfg(feature = "large-sv")]
	if lang == "sv" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_SV)?);
	}
	#[cfg(feature = "large-uk")]
	if lang == "uk" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_UK)?);
	}
	#[cfg(feature = "large-zh")]
	if lang == "zh" && wordlist == "large" {
		return Ok(WordFreq::deserialize(DATA_LARGE_ZH)?);
	}
	#[cfg(feature = "small-ar")]
	if lang == "ar" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_AR)?);
	}
	#[cfg(feature = "small-bg")]
	if lang == "bg" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_BG)?);
	}
	#[cfg(feature = "small-bn")]
	if lang == "bn" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_BN)?);
	}
	#[cfg(feature = "small-ca")]
	if lang == "ca" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_CA)?);
	}
	#[cfg(feature = "small-cs")]
	if lang == "cs" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_CS)?);
	}
	#[cfg(feature = "small-da")]
	if lang == "da" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_DA)?);
	}
	#[cfg(feature = "small-de")]
	if lang == "de" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_DE)?);
	}
	#[cfg(feature = "small-el")]
	if lang == "el" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_EL)?);
	}
	#[cfg(feature = "small-en")]
	if lang == "en" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_EN)?);
	}
	#[cfg(feature = "small-es")]
	if lang == "es" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_ES)?);
	}
	#[cfg(feature = "small-fa")]
	if lang == "fa" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_FA)?);
	}
	#[cfg(feature = "small-fi")]
	if lang == "fi" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_FI)?);
	}
	#[cfg(feature = "small-fil")]
	if lang == "fil" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_FIL)?);
	}
	#[cfg(feature = "small-fr")]
	if lang == "fr" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_FR)?);
	}
	#[cfg(feature = "small-he")]
	if lang == "he" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_HE)?);
	}
	#[cfg(feature = "small-hi")]
	if lang == "hi" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_HI)?);
	}
	#[cfg(feature = "small-hu")]
	if lang == "hu" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_HU)?);
	}
	#[cfg(feature = "small-id")]
	if lang == "id" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_ID)?);
	}
	#[cfg(feature = "small-is")]
	if lang == "is" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_IS)?);
	}
	#[cfg(feature = "small-it")]
	if lang == "it" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_IT)?);
	}
	#[cfg(feature = "small-ja")]
	if lang == "ja" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_JA)?);
	}
	#[cfg(feature = "small-ko")]
	if lang == "ko" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_KO)?);
	}
	#[cfg(feature = "small-lt")]
	if lang == "lt" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_LT)?);
	}
	#[cfg(feature = "small-lv")]
	if lang == "lv" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_LV)?);
	}
	#[cfg(feature = "small-mk")]
	if lang == "mk" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_MK)?);
	}
	#[cfg(feature = "small-ms")]
	if lang == "ms" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_MS)?);
	}
	#[cfg(feature = "small-nb")]
	if lang == "nb" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_NB)?);
	}
	#[cfg(feature = "small-nl")]
	if lang == "nl" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_NL)?);
	}
	#[cfg(feature = "small-pl")]
	if lang == "pl" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_PL)?);
	}
	#[cfg(feature = "small-pt")]
	if lang == "pt" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_PT)?);
	}
	#[cfg(feature = "small-ro")]
	if lang == "ro" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_RO)?);
	}
	#[cfg(feature = "small-ru")]
	if lang == "ru" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_RU)?);
	}
	#[cfg(feature = "small-sh")]
	if lang == "sh" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_SH)?);
	}
	#[cfg(feature = "small-sk")]
	if lang == "sk" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_SK)?);
	}
	#[cfg(feature = "small-sl")]
	if lang == "sl" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_SL)?);
	}
	#[cfg(feature = "small-sv")]
	if lang == "sv" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_SV)?);
	}
	#[cfg(feature = "small-ta")]
	if lang == "ta" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_TA)?);
	}
	#[cfg(feature = "small-tr")]
	if lang == "tr" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_TR)?);
	}
	#[cfg(feature = "small-uk")]
	if lang == "uk" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_UK)?);
	}
	#[cfg(feature = "small-ur")]
	if lang == "ur" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_UR)?);
	}
	#[cfg(feature = "small-vi")]
	if lang == "vi" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_VI)?);
	}
	#[cfg(feature = "small-zh")]
	if lang == "zh" && wordlist == "small" {
		return Ok(WordFreq::deserialize(DATA_SMALL_ZH)?);
	}
	Err(anyhow!("Unknown language or wordlist: {lang}-{wordlist}"))
}
