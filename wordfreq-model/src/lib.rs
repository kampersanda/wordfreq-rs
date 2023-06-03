use std::env;

use anyhow::Result;
use wordfreq_core::WordFreq;

pub enum ModelKind {
	#[cfg(feature = "large-ar")]
	LargeAr,
	#[cfg(feature = "large-bn")]
	LargeBn,
	#[cfg(feature = "large-ca")]
	LargeCa,
	#[cfg(feature = "large-cs")]
	LargeCs,
	#[cfg(feature = "large-de")]
	LargeDe,
	#[cfg(feature = "large-en")]
	LargeEn,
	#[cfg(feature = "large-es")]
	LargeEs,
	#[cfg(feature = "large-fi")]
	LargeFi,
	#[cfg(feature = "large-fr")]
	LargeFr,
	#[cfg(feature = "large-he")]
	LargeHe,
	#[cfg(feature = "large-it")]
	LargeIt,
	#[cfg(feature = "large-ja")]
	LargeJa,
	#[cfg(feature = "large-mk")]
	LargeMk,
	#[cfg(feature = "large-nb")]
	LargeNb,
	#[cfg(feature = "large-nl")]
	LargeNl,
	#[cfg(feature = "large-pl")]
	LargePl,
	#[cfg(feature = "large-pt")]
	LargePt,
	#[cfg(feature = "large-ru")]
	LargeRu,
	#[cfg(feature = "large-sv")]
	LargeSv,
	#[cfg(feature = "large-uk")]
	LargeUk,
	#[cfg(feature = "large-zh")]
	LargeZh,
	#[cfg(feature = "small-ar")]
	SmallAr,
	#[cfg(feature = "small-bg")]
	SmallBg,
	#[cfg(feature = "small-bn")]
	SmallBn,
	#[cfg(feature = "small-ca")]
	SmallCa,
	#[cfg(feature = "small-cs")]
	SmallCs,
	#[cfg(feature = "small-da")]
	SmallDa,
	#[cfg(feature = "small-de")]
	SmallDe,
	#[cfg(feature = "small-el")]
	SmallEl,
	#[cfg(feature = "small-en")]
	SmallEn,
	#[cfg(feature = "small-es")]
	SmallEs,
	#[cfg(feature = "small-fa")]
	SmallFa,
	#[cfg(feature = "small-fi")]
	SmallFi,
	#[cfg(feature = "small-fil")]
	SmallFil,
	#[cfg(feature = "small-fr")]
	SmallFr,
	#[cfg(feature = "small-he")]
	SmallHe,
	#[cfg(feature = "small-hi")]
	SmallHi,
	#[cfg(feature = "small-hu")]
	SmallHu,
	#[cfg(feature = "small-id")]
	SmallId,
	#[cfg(feature = "small-is")]
	SmallIs,
	#[cfg(feature = "small-it")]
	SmallIt,
	#[cfg(feature = "small-ja")]
	SmallJa,
	#[cfg(feature = "small-ko")]
	SmallKo,
	#[cfg(feature = "small-lt")]
	SmallLt,
	#[cfg(feature = "small-lv")]
	SmallLv,
	#[cfg(feature = "small-mk")]
	SmallMk,
	#[cfg(feature = "small-ms")]
	SmallMs,
	#[cfg(feature = "small-nb")]
	SmallNb,
	#[cfg(feature = "small-nl")]
	SmallNl,
	#[cfg(feature = "small-pl")]
	SmallPl,
	#[cfg(feature = "small-pt")]
	SmallPt,
	#[cfg(feature = "small-ro")]
	SmallRo,
	#[cfg(feature = "small-ru")]
	SmallRu,
	#[cfg(feature = "small-sh")]
	SmallSh,
	#[cfg(feature = "small-sk")]
	SmallSk,
	#[cfg(feature = "small-sl")]
	SmallSl,
	#[cfg(feature = "small-sv")]
	SmallSv,
	#[cfg(feature = "small-ta")]
	SmallTa,
	#[cfg(feature = "small-tr")]
	SmallTr,
	#[cfg(feature = "small-uk")]
	SmallUk,
	#[cfg(feature = "small-ur")]
	SmallUr,
	#[cfg(feature = "small-vi")]
	SmallVi,
	#[cfg(feature = "small-zh")]
	SmallZh,
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
		ModelKind::LargeAr => Ok(WordFreq::deserialize(DATA_LARGE_AR)?),
		#[cfg(feature = "large-bn")]
		ModelKind::LargeBn => Ok(WordFreq::deserialize(DATA_LARGE_BN)?),
		#[cfg(feature = "large-ca")]
		ModelKind::LargeCa => Ok(WordFreq::deserialize(DATA_LARGE_CA)?),
		#[cfg(feature = "large-cs")]
		ModelKind::LargeCs => Ok(WordFreq::deserialize(DATA_LARGE_CS)?),
		#[cfg(feature = "large-de")]
		ModelKind::LargeDe => Ok(WordFreq::deserialize(DATA_LARGE_DE)?),
		#[cfg(feature = "large-en")]
		ModelKind::LargeEn => Ok(WordFreq::deserialize(DATA_LARGE_EN)?),
		#[cfg(feature = "large-es")]
		ModelKind::LargeEs => Ok(WordFreq::deserialize(DATA_LARGE_ES)?),
		#[cfg(feature = "large-fi")]
		ModelKind::LargeFi => Ok(WordFreq::deserialize(DATA_LARGE_FI)?),
		#[cfg(feature = "large-fr")]
		ModelKind::LargeFr => Ok(WordFreq::deserialize(DATA_LARGE_FR)?),
		#[cfg(feature = "large-he")]
		ModelKind::LargeHe => Ok(WordFreq::deserialize(DATA_LARGE_HE)?),
		#[cfg(feature = "large-it")]
		ModelKind::LargeIt => Ok(WordFreq::deserialize(DATA_LARGE_IT)?),
		#[cfg(feature = "large-ja")]
		ModelKind::LargeJa => Ok(WordFreq::deserialize(DATA_LARGE_JA)?),
		#[cfg(feature = "large-mk")]
		ModelKind::LargeMk => Ok(WordFreq::deserialize(DATA_LARGE_MK)?),
		#[cfg(feature = "large-nb")]
		ModelKind::LargeNb => Ok(WordFreq::deserialize(DATA_LARGE_NB)?),
		#[cfg(feature = "large-nl")]
		ModelKind::LargeNl => Ok(WordFreq::deserialize(DATA_LARGE_NL)?),
		#[cfg(feature = "large-pl")]
		ModelKind::LargePl => Ok(WordFreq::deserialize(DATA_LARGE_PL)?),
		#[cfg(feature = "large-pt")]
		ModelKind::LargePt => Ok(WordFreq::deserialize(DATA_LARGE_PT)?),
		#[cfg(feature = "large-ru")]
		ModelKind::LargeRu => Ok(WordFreq::deserialize(DATA_LARGE_RU)?),
		#[cfg(feature = "large-sv")]
		ModelKind::LargeSv => Ok(WordFreq::deserialize(DATA_LARGE_SV)?),
		#[cfg(feature = "large-uk")]
		ModelKind::LargeUk => Ok(WordFreq::deserialize(DATA_LARGE_UK)?),
		#[cfg(feature = "large-zh")]
		ModelKind::LargeZh => Ok(WordFreq::deserialize(DATA_LARGE_ZH)?),
		#[cfg(feature = "small-ar")]
		ModelKind::SmallAr => Ok(WordFreq::deserialize(DATA_SMALL_AR)?),
		#[cfg(feature = "small-bg")]
		ModelKind::SmallBg => Ok(WordFreq::deserialize(DATA_SMALL_BG)?),
		#[cfg(feature = "small-bn")]
		ModelKind::SmallBn => Ok(WordFreq::deserialize(DATA_SMALL_BN)?),
		#[cfg(feature = "small-ca")]
		ModelKind::SmallCa => Ok(WordFreq::deserialize(DATA_SMALL_CA)?),
		#[cfg(feature = "small-cs")]
		ModelKind::SmallCs => Ok(WordFreq::deserialize(DATA_SMALL_CS)?),
		#[cfg(feature = "small-da")]
		ModelKind::SmallDa => Ok(WordFreq::deserialize(DATA_SMALL_DA)?),
		#[cfg(feature = "small-de")]
		ModelKind::SmallDe => Ok(WordFreq::deserialize(DATA_SMALL_DE)?),
		#[cfg(feature = "small-el")]
		ModelKind::SmallEl => Ok(WordFreq::deserialize(DATA_SMALL_EL)?),
		#[cfg(feature = "small-en")]
		ModelKind::SmallEn => Ok(WordFreq::deserialize(DATA_SMALL_EN)?),
		#[cfg(feature = "small-es")]
		ModelKind::SmallEs => Ok(WordFreq::deserialize(DATA_SMALL_ES)?),
		#[cfg(feature = "small-fa")]
		ModelKind::SmallFa => Ok(WordFreq::deserialize(DATA_SMALL_FA)?),
		#[cfg(feature = "small-fi")]
		ModelKind::SmallFi => Ok(WordFreq::deserialize(DATA_SMALL_FI)?),
		#[cfg(feature = "small-fil")]
		ModelKind::SmallFil => Ok(WordFreq::deserialize(DATA_SMALL_FIL)?),
		#[cfg(feature = "small-fr")]
		ModelKind::SmallFr => Ok(WordFreq::deserialize(DATA_SMALL_FR)?),
		#[cfg(feature = "small-he")]
		ModelKind::SmallHe => Ok(WordFreq::deserialize(DATA_SMALL_HE)?),
		#[cfg(feature = "small-hi")]
		ModelKind::SmallHi => Ok(WordFreq::deserialize(DATA_SMALL_HI)?),
		#[cfg(feature = "small-hu")]
		ModelKind::SmallHu => Ok(WordFreq::deserialize(DATA_SMALL_HU)?),
		#[cfg(feature = "small-id")]
		ModelKind::SmallId => Ok(WordFreq::deserialize(DATA_SMALL_ID)?),
		#[cfg(feature = "small-is")]
		ModelKind::SmallIs => Ok(WordFreq::deserialize(DATA_SMALL_IS)?),
		#[cfg(feature = "small-it")]
		ModelKind::SmallIt => Ok(WordFreq::deserialize(DATA_SMALL_IT)?),
		#[cfg(feature = "small-ja")]
		ModelKind::SmallJa => Ok(WordFreq::deserialize(DATA_SMALL_JA)?),
		#[cfg(feature = "small-ko")]
		ModelKind::SmallKo => Ok(WordFreq::deserialize(DATA_SMALL_KO)?),
		#[cfg(feature = "small-lt")]
		ModelKind::SmallLt => Ok(WordFreq::deserialize(DATA_SMALL_LT)?),
		#[cfg(feature = "small-lv")]
		ModelKind::SmallLv => Ok(WordFreq::deserialize(DATA_SMALL_LV)?),
		#[cfg(feature = "small-mk")]
		ModelKind::SmallMk => Ok(WordFreq::deserialize(DATA_SMALL_MK)?),
		#[cfg(feature = "small-ms")]
		ModelKind::SmallMs => Ok(WordFreq::deserialize(DATA_SMALL_MS)?),
		#[cfg(feature = "small-nb")]
		ModelKind::SmallNb => Ok(WordFreq::deserialize(DATA_SMALL_NB)?),
		#[cfg(feature = "small-nl")]
		ModelKind::SmallNl => Ok(WordFreq::deserialize(DATA_SMALL_NL)?),
		#[cfg(feature = "small-pl")]
		ModelKind::SmallPl => Ok(WordFreq::deserialize(DATA_SMALL_PL)?),
		#[cfg(feature = "small-pt")]
		ModelKind::SmallPt => Ok(WordFreq::deserialize(DATA_SMALL_PT)?),
		#[cfg(feature = "small-ro")]
		ModelKind::SmallRo => Ok(WordFreq::deserialize(DATA_SMALL_RO)?),
		#[cfg(feature = "small-ru")]
		ModelKind::SmallRu => Ok(WordFreq::deserialize(DATA_SMALL_RU)?),
		#[cfg(feature = "small-sh")]
		ModelKind::SmallSh => Ok(WordFreq::deserialize(DATA_SMALL_SH)?),
		#[cfg(feature = "small-sk")]
		ModelKind::SmallSk => Ok(WordFreq::deserialize(DATA_SMALL_SK)?),
		#[cfg(feature = "small-sl")]
		ModelKind::SmallSl => Ok(WordFreq::deserialize(DATA_SMALL_SL)?),
		#[cfg(feature = "small-sv")]
		ModelKind::SmallSv => Ok(WordFreq::deserialize(DATA_SMALL_SV)?),
		#[cfg(feature = "small-ta")]
		ModelKind::SmallTa => Ok(WordFreq::deserialize(DATA_SMALL_TA)?),
		#[cfg(feature = "small-tr")]
		ModelKind::SmallTr => Ok(WordFreq::deserialize(DATA_SMALL_TR)?),
		#[cfg(feature = "small-uk")]
		ModelKind::SmallUk => Ok(WordFreq::deserialize(DATA_SMALL_UK)?),
		#[cfg(feature = "small-ur")]
		ModelKind::SmallUr => Ok(WordFreq::deserialize(DATA_SMALL_UR)?),
		#[cfg(feature = "small-vi")]
		ModelKind::SmallVi => Ok(WordFreq::deserialize(DATA_SMALL_VI)?),
		#[cfg(feature = "small-zh")]
		ModelKind::SmallZh => Ok(WordFreq::deserialize(DATA_SMALL_ZH)?),
}
}
