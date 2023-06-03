use std::env;

use anyhow::{anyhow, Result};
use wordfreq_core::WordFreq;

pub enum ModelKind {
    #[cfg(feature = "large-ar")]
    LARGE_AR,
    #[cfg(feature = "large-bn")]
    LARGE_BN,
    #[cfg(feature = "large-ca")]
    LARGE_CA,
    #[cfg(feature = "large-cs")]
    LARGE_CS,
    #[cfg(feature = "large-de")]
    LARGE_DE,
    #[cfg(feature = "large-en")]
    LARGE_EN,
    #[cfg(feature = "large-es")]
    LARGE_ES,
    #[cfg(feature = "large-fi")]
    LARGE_FI,
    #[cfg(feature = "large-fr")]
    LARGE_FR,
    #[cfg(feature = "large-he")]
    LARGE_HE,
    #[cfg(feature = "large-it")]
    LARGE_IT,
    #[cfg(feature = "large-ja")]
    LARGE_JA,
    #[cfg(feature = "large-mk")]
    LARGE_MK,
    #[cfg(feature = "large-nb")]
    LARGE_NB,
    #[cfg(feature = "large-nl")]
    LARGE_NL,
    #[cfg(feature = "large-pl")]
    LARGE_PL,
    #[cfg(feature = "large-pt")]
    LARGE_PT,
    #[cfg(feature = "large-ru")]
    LARGE_RU,
    #[cfg(feature = "large-sv")]
    LARGE_SV,
    #[cfg(feature = "large-uk")]
    LARGE_UK,
    #[cfg(feature = "large-zh")]
    LARGE_ZH,
    #[cfg(feature = "small-ar")]
    SMALL_AR,
    #[cfg(feature = "small-bg")]
    SMALL_BG,
    #[cfg(feature = "small-bn")]
    SMALL_BN,
    #[cfg(feature = "small-ca")]
    SMALL_CA,
    #[cfg(feature = "small-cs")]
    SMALL_CS,
    #[cfg(feature = "small-da")]
    SMALL_DA,
    #[cfg(feature = "small-de")]
    SMALL_DE,
    #[cfg(feature = "small-el")]
    SMALL_EL,
    #[cfg(feature = "small-en")]
    SMALL_EN,
    #[cfg(feature = "small-es")]
    SMALL_ES,
    #[cfg(feature = "small-fa")]
    SMALL_FA,
    #[cfg(feature = "small-fi")]
    SMALL_FI,
    #[cfg(feature = "small-fil")]
    SMALL_FIL,
    #[cfg(feature = "small-fr")]
    SMALL_FR,
    #[cfg(feature = "small-he")]
    SMALL_HE,
    #[cfg(feature = "small-hi")]
    SMALL_HI,
    #[cfg(feature = "small-hu")]
    SMALL_HU,
    #[cfg(feature = "small-id")]
    SMALL_ID,
    #[cfg(feature = "small-is")]
    SMALL_IS,
    #[cfg(feature = "small-it")]
    SMALL_IT,
    #[cfg(feature = "small-ja")]
    SMALL_JA,
    #[cfg(feature = "small-ko")]
    SMALL_KO,
    #[cfg(feature = "small-lt")]
    SMALL_LT,
    #[cfg(feature = "small-lv")]
    SMALL_LV,
    #[cfg(feature = "small-mk")]
    SMALL_MK,
    #[cfg(feature = "small-ms")]
    SMALL_MS,
    #[cfg(feature = "small-nb")]
    SMALL_NB,
    #[cfg(feature = "small-nl")]
    SMALL_NL,
    #[cfg(feature = "small-pl")]
    SMALL_PL,
    #[cfg(feature = "small-pt")]
    SMALL_PT,
    #[cfg(feature = "small-ro")]
    SMALL_RO,
    #[cfg(feature = "small-ru")]
    SMALL_RU,
    #[cfg(feature = "small-sh")]
    SMALL_SH,
    #[cfg(feature = "small-sk")]
    SMALL_SK,
    #[cfg(feature = "small-sl")]
    SMALL_SL,
    #[cfg(feature = "small-sv")]
    SMALL_SV,
    #[cfg(feature = "small-ta")]
    SMALL_TA,
    #[cfg(feature = "small-tr")]
    SMALL_TR,
    #[cfg(feature = "small-uk")]
    SMALL_UK,
    #[cfg(feature = "small-ur")]
    SMALL_UR,
    #[cfg(feature = "small-vi")]
    SMALL_VI,
    #[cfg(feature = "small-zh")]
    SMALL_ZH,
}

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

pub fn load_wordfreq(kind: ModelKind) -> Result<WordFreq> {
    match kind {
        #[cfg(feature = "large-ar")]
        ModelKind::LARGE_AR => Ok(WordFreq::deserialize(DATA_LARGE_AR)?),
        #[cfg(feature = "large-bn")]
        ModelKind::LARGE_BN => Ok(WordFreq::deserialize(DATA_LARGE_BN)?),
        #[cfg(feature = "large-ca")]
        ModelKind::LARGE_CA => Ok(WordFreq::deserialize(DATA_LARGE_CA)?),
        #[cfg(feature = "large-cs")]
        ModelKind::LARGE_CS => Ok(WordFreq::deserialize(DATA_LARGE_CS)?),
        #[cfg(feature = "large-de")]
        ModelKind::LARGE_DE => Ok(WordFreq::deserialize(DATA_LARGE_DE)?),
        #[cfg(feature = "large-en")]
        ModelKind::LARGE_EN => Ok(WordFreq::deserialize(DATA_LARGE_EN)?),
        #[cfg(feature = "large-es")]
        ModelKind::LARGE_ES => Ok(WordFreq::deserialize(DATA_LARGE_ES)?),
        #[cfg(feature = "large-fi")]
        ModelKind::LARGE_FI => Ok(WordFreq::deserialize(DATA_LARGE_FI)?),
        #[cfg(feature = "large-fr")]
        ModelKind::LARGE_FR => Ok(WordFreq::deserialize(DATA_LARGE_FR)?),
        #[cfg(feature = "large-he")]
        ModelKind::LARGE_HE => Ok(WordFreq::deserialize(DATA_LARGE_HE)?),
        #[cfg(feature = "large-it")]
        ModelKind::LARGE_IT => Ok(WordFreq::deserialize(DATA_LARGE_IT)?),
        #[cfg(feature = "large-ja")]
        ModelKind::LARGE_JA => Ok(WordFreq::deserialize(DATA_LARGE_JA)?),
        #[cfg(feature = "large-mk")]
        ModelKind::LARGE_MK => Ok(WordFreq::deserialize(DATA_LARGE_MK)?),
        #[cfg(feature = "large-nb")]
        ModelKind::LARGE_NB => Ok(WordFreq::deserialize(DATA_LARGE_NB)?),
        #[cfg(feature = "large-nl")]
        ModelKind::LARGE_NL => Ok(WordFreq::deserialize(DATA_LARGE_NL)?),
        #[cfg(feature = "large-pl")]
        ModelKind::LARGE_PL => Ok(WordFreq::deserialize(DATA_LARGE_PL)?),
        #[cfg(feature = "large-pt")]
        ModelKind::LARGE_PT => Ok(WordFreq::deserialize(DATA_LARGE_PT)?),
        #[cfg(feature = "large-ru")]
        ModelKind::LARGE_RU => Ok(WordFreq::deserialize(DATA_LARGE_RU)?),
        #[cfg(feature = "large-sv")]
        ModelKind::LARGE_SV => Ok(WordFreq::deserialize(DATA_LARGE_SV)?),
        #[cfg(feature = "large-uk")]
        ModelKind::LARGE_UK => Ok(WordFreq::deserialize(DATA_LARGE_UK)?),
        #[cfg(feature = "large-zh")]
        ModelKind::LARGE_ZH => Ok(WordFreq::deserialize(DATA_LARGE_ZH)?),
        #[cfg(feature = "small-ar")]
        ModelKind::SMALL_AR => Ok(WordFreq::deserialize(DATA_SMALL_AR)?),
        #[cfg(feature = "small-bg")]
        ModelKind::SMALL_BG => Ok(WordFreq::deserialize(DATA_SMALL_BG)?),
        #[cfg(feature = "small-bn")]
        ModelKind::SMALL_BN => Ok(WordFreq::deserialize(DATA_SMALL_BN)?),
        #[cfg(feature = "small-ca")]
        ModelKind::SMALL_CA => Ok(WordFreq::deserialize(DATA_SMALL_CA)?),
        #[cfg(feature = "small-cs")]
        ModelKind::SMALL_CS => Ok(WordFreq::deserialize(DATA_SMALL_CS)?),
        #[cfg(feature = "small-da")]
        ModelKind::SMALL_DA => Ok(WordFreq::deserialize(DATA_SMALL_DA)?),
        #[cfg(feature = "small-de")]
        ModelKind::SMALL_DE => Ok(WordFreq::deserialize(DATA_SMALL_DE)?),
        #[cfg(feature = "small-el")]
        ModelKind::SMALL_EL => Ok(WordFreq::deserialize(DATA_SMALL_EL)?),
        #[cfg(feature = "small-en")]
        ModelKind::SMALL_EN => Ok(WordFreq::deserialize(DATA_SMALL_EN)?),
        #[cfg(feature = "small-es")]
        ModelKind::SMALL_ES => Ok(WordFreq::deserialize(DATA_SMALL_ES)?),
        #[cfg(feature = "small-fa")]
        ModelKind::SMALL_FA => Ok(WordFreq::deserialize(DATA_SMALL_FA)?),
        #[cfg(feature = "small-fi")]
        ModelKind::SMALL_FI => Ok(WordFreq::deserialize(DATA_SMALL_FI)?),
        #[cfg(feature = "small-fil")]
        ModelKind::SMALL_FIL => Ok(WordFreq::deserialize(DATA_SMALL_FIL)?),
        #[cfg(feature = "small-fr")]
        ModelKind::SMALL_FR => Ok(WordFreq::deserialize(DATA_SMALL_FR)?),
        #[cfg(feature = "small-he")]
        ModelKind::SMALL_HE => Ok(WordFreq::deserialize(DATA_SMALL_HE)?),
        #[cfg(feature = "small-hi")]
        ModelKind::SMALL_HI => Ok(WordFreq::deserialize(DATA_SMALL_HI)?),
        #[cfg(feature = "small-hu")]
        ModelKind::SMALL_HU => Ok(WordFreq::deserialize(DATA_SMALL_HU)?),
        #[cfg(feature = "small-id")]
        ModelKind::SMALL_ID => Ok(WordFreq::deserialize(DATA_SMALL_ID)?),
        #[cfg(feature = "small-is")]
        ModelKind::SMALL_IS => Ok(WordFreq::deserialize(DATA_SMALL_IS)?),
        #[cfg(feature = "small-it")]
        ModelKind::SMALL_IT => Ok(WordFreq::deserialize(DATA_SMALL_IT)?),
        #[cfg(feature = "small-ja")]
        ModelKind::SMALL_JA => Ok(WordFreq::deserialize(DATA_SMALL_JA)?),
        #[cfg(feature = "small-ko")]
        ModelKind::SMALL_KO => Ok(WordFreq::deserialize(DATA_SMALL_KO)?),
        #[cfg(feature = "small-lt")]
        ModelKind::SMALL_LT => Ok(WordFreq::deserialize(DATA_SMALL_LT)?),
        #[cfg(feature = "small-lv")]
        ModelKind::SMALL_LV => Ok(WordFreq::deserialize(DATA_SMALL_LV)?),
        #[cfg(feature = "small-mk")]
        ModelKind::SMALL_MK => Ok(WordFreq::deserialize(DATA_SMALL_MK)?),
        #[cfg(feature = "small-ms")]
        ModelKind::SMALL_MS => Ok(WordFreq::deserialize(DATA_SMALL_MS)?),
        #[cfg(feature = "small-nb")]
        ModelKind::SMALL_NB => Ok(WordFreq::deserialize(DATA_SMALL_NB)?),
        #[cfg(feature = "small-nl")]
        ModelKind::SMALL_NL => Ok(WordFreq::deserialize(DATA_SMALL_NL)?),
        #[cfg(feature = "small-pl")]
        ModelKind::SMALL_PL => Ok(WordFreq::deserialize(DATA_SMALL_PL)?),
        #[cfg(feature = "small-pt")]
        ModelKind::SMALL_PT => Ok(WordFreq::deserialize(DATA_SMALL_PT)?),
        #[cfg(feature = "small-ro")]
        ModelKind::SMALL_RO => Ok(WordFreq::deserialize(DATA_SMALL_RO)?),
        #[cfg(feature = "small-ru")]
        ModelKind::SMALL_RU => Ok(WordFreq::deserialize(DATA_SMALL_RU)?),
        #[cfg(feature = "small-sh")]
        ModelKind::SMALL_SH => Ok(WordFreq::deserialize(DATA_SMALL_SH)?),
        #[cfg(feature = "small-sk")]
        ModelKind::SMALL_SK => Ok(WordFreq::deserialize(DATA_SMALL_SK)?),
        #[cfg(feature = "small-sl")]
        ModelKind::SMALL_SL => Ok(WordFreq::deserialize(DATA_SMALL_SL)?),
        #[cfg(feature = "small-sv")]
        ModelKind::SMALL_SV => Ok(WordFreq::deserialize(DATA_SMALL_SV)?),
        #[cfg(feature = "small-ta")]
        ModelKind::SMALL_TA => Ok(WordFreq::deserialize(DATA_SMALL_TA)?),
        #[cfg(feature = "small-tr")]
        ModelKind::SMALL_TR => Ok(WordFreq::deserialize(DATA_SMALL_TR)?),
        #[cfg(feature = "small-uk")]
        ModelKind::SMALL_UK => Ok(WordFreq::deserialize(DATA_SMALL_UK)?),
        #[cfg(feature = "small-ur")]
        ModelKind::SMALL_UR => Ok(WordFreq::deserialize(DATA_SMALL_UR)?),
        #[cfg(feature = "small-vi")]
        ModelKind::SMALL_VI => Ok(WordFreq::deserialize(DATA_SMALL_VI)?),
        #[cfg(feature = "small-zh")]
        ModelKind::SMALL_ZH => Ok(WordFreq::deserialize(DATA_SMALL_ZH)?),
    }
}
