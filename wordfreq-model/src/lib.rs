//! # wordfreq-model
//!
//! This crate provides a loader for pre-compiled wordfreq models,
//! allowing you to easily create [`WordFreq`] instances for various languages.
//!
//! ## Instructions
//!
//! The provided models are the same as those distributed in the [original Python package](https://doi.org/10.5281/zenodo.7199437).
//! See the [original documentation](https://github.com/rspeer/wordfreq/tree/v3.0.2#sources-and-supported-languages)
//! for the supported languages and their sources.
//!
//! You need to specify models you want to use with `features`.
//! The feature names are in the form of `large-xx` or `small-xx`, where `xx` is the language code.
//! For example, if you want to use the large-English and small-Japanese models,
//! specify `large-en` and `small-ja` as follows:
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies.wordfreq-model]
//! version = "0.1"
//! features = ["large-en", "small-ja"]
//! ```
//!
//! There is no default feature.
//! **Be sure to specify features you want to use.**
//!
//! ## Examples
//!
//! [`load_wordfreq`] can create a [`WordFreq`] instance from a [`ModelKind`] enum value.
//! [`ModelKind`] will have the specified feature names in CamelCase, such as `LargeEn` or `SmallJa`.
//!
//! By default, only [`ModelKind::ExampleEn`] appears for tests.
//!
//! ```
//! use approx::assert_relative_eq;
//! use wordfreq_model::load_wordfreq;
//! use wordfreq_model::ModelKind;
//!
//! let wf = load_wordfreq(ModelKind::ExampleEn).unwrap();
//! assert_relative_eq!(wf.word_frequency("las"), 0.25);
//! assert_relative_eq!(wf.word_frequency("vegas"), 0.75);
//! assert_relative_eq!(wf.word_frequency("Las"), 0.25); // Standardized
//! ```
//!
//! ## Standardization
//!
//! As the above example shows, the model automatically standardizes words before looking them up (i.e., `Las` is handled as `las`).
//! This is done by an instance [`Standardizer`] set up in the [`WordFreq`] instance.
//! [`load_wordfreq`] automatically sets up an appropriate [`Standardizer`] instance for each language.
//!
//! ## Notes
//!
//! This crate downloads specified model files and embeds the models directly into the source code.
//! **Specify as many models as you need** to avoid extra downloads and bloating the resulting binary.
//!
//! The actual model files to be used are placed [here](https://github.com/kampersanda/wordfreq-rs/releases/tag/models-v1) together with the credits.
//! If you do not desire automatic model downloads and binary embedding, you can create instances from these files directly.
//! See the instructions in [wordfreq].
use std::env;

use anyhow::Result;
use wordfreq::Standardizer;
use wordfreq::WordFreq;

/// Supported model kinds.
///
/// Since only specified feature names are available,
/// only [`ModelKind::ExampleEn`] will be displayed in docs.rs.
/// Normally, those you specify will be available such as `LargeEn` or `SmallJa`.
///
/// If models you want to use are not available,
/// specify the features following the Instructions.
pub enum ModelKind {
    /// Example data for tests.
    ExampleEn,
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

const DATA_EXAMPLE_EN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/example_en.bin"));
#[cfg(feature = "large-ar")]
const DATA_LARGE_AR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ar.bin"));
#[cfg(feature = "large-bn")]
const DATA_LARGE_BN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_bn.bin"));
#[cfg(feature = "large-ca")]
const DATA_LARGE_CA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ca.bin"));
#[cfg(feature = "large-cs")]
const DATA_LARGE_CS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_cs.bin"));
#[cfg(feature = "large-de")]
const DATA_LARGE_DE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_de.bin"));
#[cfg(feature = "large-en")]
const DATA_LARGE_EN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_en.bin"));
#[cfg(feature = "large-es")]
const DATA_LARGE_ES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_es.bin"));
#[cfg(feature = "large-fi")]
const DATA_LARGE_FI: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_fi.bin"));
#[cfg(feature = "large-fr")]
const DATA_LARGE_FR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_fr.bin"));
#[cfg(feature = "large-he")]
const DATA_LARGE_HE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_he.bin"));
#[cfg(feature = "large-it")]
const DATA_LARGE_IT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_it.bin"));
#[cfg(feature = "large-ja")]
const DATA_LARGE_JA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ja.bin"));
#[cfg(feature = "large-mk")]
const DATA_LARGE_MK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_mk.bin"));
#[cfg(feature = "large-nb")]
const DATA_LARGE_NB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_nb.bin"));
#[cfg(feature = "large-nl")]
const DATA_LARGE_NL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_nl.bin"));
#[cfg(feature = "large-pl")]
const DATA_LARGE_PL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_pl.bin"));
#[cfg(feature = "large-pt")]
const DATA_LARGE_PT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_pt.bin"));
#[cfg(feature = "large-ru")]
const DATA_LARGE_RU: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_ru.bin"));
#[cfg(feature = "large-sv")]
const DATA_LARGE_SV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_sv.bin"));
#[cfg(feature = "large-uk")]
const DATA_LARGE_UK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_uk.bin"));
#[cfg(feature = "large-zh")]
const DATA_LARGE_ZH: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/large_zh.bin"));
#[cfg(feature = "small-ar")]
const DATA_SMALL_AR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ar.bin"));
#[cfg(feature = "small-bg")]
const DATA_SMALL_BG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_bg.bin"));
#[cfg(feature = "small-bn")]
const DATA_SMALL_BN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_bn.bin"));
#[cfg(feature = "small-ca")]
const DATA_SMALL_CA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ca.bin"));
#[cfg(feature = "small-cs")]
const DATA_SMALL_CS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_cs.bin"));
#[cfg(feature = "small-da")]
const DATA_SMALL_DA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_da.bin"));
#[cfg(feature = "small-de")]
const DATA_SMALL_DE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_de.bin"));
#[cfg(feature = "small-el")]
const DATA_SMALL_EL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_el.bin"));
#[cfg(feature = "small-en")]
const DATA_SMALL_EN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_en.bin"));
#[cfg(feature = "small-es")]
const DATA_SMALL_ES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_es.bin"));
#[cfg(feature = "small-fa")]
const DATA_SMALL_FA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fa.bin"));
#[cfg(feature = "small-fi")]
const DATA_SMALL_FI: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fi.bin"));
#[cfg(feature = "small-fil")]
const DATA_SMALL_FIL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fil.bin"));
#[cfg(feature = "small-fr")]
const DATA_SMALL_FR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_fr.bin"));
#[cfg(feature = "small-he")]
const DATA_SMALL_HE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_he.bin"));
#[cfg(feature = "small-hi")]
const DATA_SMALL_HI: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_hi.bin"));
#[cfg(feature = "small-hu")]
const DATA_SMALL_HU: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_hu.bin"));
#[cfg(feature = "small-id")]
const DATA_SMALL_ID: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_id.bin"));
#[cfg(feature = "small-is")]
const DATA_SMALL_IS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_is.bin"));
#[cfg(feature = "small-it")]
const DATA_SMALL_IT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_it.bin"));
#[cfg(feature = "small-ja")]
const DATA_SMALL_JA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ja.bin"));
#[cfg(feature = "small-ko")]
const DATA_SMALL_KO: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ko.bin"));
#[cfg(feature = "small-lt")]
const DATA_SMALL_LT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_lt.bin"));
#[cfg(feature = "small-lv")]
const DATA_SMALL_LV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_lv.bin"));
#[cfg(feature = "small-mk")]
const DATA_SMALL_MK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_mk.bin"));
#[cfg(feature = "small-ms")]
const DATA_SMALL_MS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ms.bin"));
#[cfg(feature = "small-nb")]
const DATA_SMALL_NB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_nb.bin"));
#[cfg(feature = "small-nl")]
const DATA_SMALL_NL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_nl.bin"));
#[cfg(feature = "small-pl")]
const DATA_SMALL_PL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_pl.bin"));
#[cfg(feature = "small-pt")]
const DATA_SMALL_PT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_pt.bin"));
#[cfg(feature = "small-ro")]
const DATA_SMALL_RO: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ro.bin"));
#[cfg(feature = "small-ru")]
const DATA_SMALL_RU: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ru.bin"));
#[cfg(feature = "small-sh")]
const DATA_SMALL_SH: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sh.bin"));
#[cfg(feature = "small-sk")]
const DATA_SMALL_SK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sk.bin"));
#[cfg(feature = "small-sl")]
const DATA_SMALL_SL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sl.bin"));
#[cfg(feature = "small-sv")]
const DATA_SMALL_SV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_sv.bin"));
#[cfg(feature = "small-ta")]
const DATA_SMALL_TA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ta.bin"));
#[cfg(feature = "small-tr")]
const DATA_SMALL_TR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_tr.bin"));
#[cfg(feature = "small-uk")]
const DATA_SMALL_UK: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_uk.bin"));
#[cfg(feature = "small-ur")]
const DATA_SMALL_UR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_ur.bin"));
#[cfg(feature = "small-vi")]
const DATA_SMALL_VI: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_vi.bin"));
#[cfg(feature = "small-zh")]
const DATA_SMALL_ZH: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/small_zh.bin"));

/// Loads a pre-compiled [`WordFreq`] model.
pub fn load_wordfreq(kind: ModelKind) -> Result<WordFreq> {
    match kind {
        ModelKind::ExampleEn => {
            Ok(WordFreq::deserialize(DATA_EXAMPLE_EN)?.standardizer(Standardizer::new("en")?))
        }
        #[cfg(feature = "large-ar")]
        ModelKind::LargeAr => {
            Ok(WordFreq::deserialize(DATA_LARGE_AR)?.standardizer(Standardizer::new("ar")?))
        }
        #[cfg(feature = "large-bn")]
        ModelKind::LargeBn => {
            Ok(WordFreq::deserialize(DATA_LARGE_BN)?.standardizer(Standardizer::new("bn")?))
        }
        #[cfg(feature = "large-ca")]
        ModelKind::LargeCa => {
            Ok(WordFreq::deserialize(DATA_LARGE_CA)?.standardizer(Standardizer::new("ca")?))
        }
        #[cfg(feature = "large-cs")]
        ModelKind::LargeCs => {
            Ok(WordFreq::deserialize(DATA_LARGE_CS)?.standardizer(Standardizer::new("cs")?))
        }
        #[cfg(feature = "large-de")]
        ModelKind::LargeDe => {
            Ok(WordFreq::deserialize(DATA_LARGE_DE)?.standardizer(Standardizer::new("de")?))
        }
        #[cfg(feature = "large-en")]
        ModelKind::LargeEn => {
            Ok(WordFreq::deserialize(DATA_LARGE_EN)?.standardizer(Standardizer::new("en")?))
        }
        #[cfg(feature = "large-es")]
        ModelKind::LargeEs => {
            Ok(WordFreq::deserialize(DATA_LARGE_ES)?.standardizer(Standardizer::new("es")?))
        }
        #[cfg(feature = "large-fi")]
        ModelKind::LargeFi => {
            Ok(WordFreq::deserialize(DATA_LARGE_FI)?.standardizer(Standardizer::new("fi")?))
        }
        #[cfg(feature = "large-fr")]
        ModelKind::LargeFr => {
            Ok(WordFreq::deserialize(DATA_LARGE_FR)?.standardizer(Standardizer::new("fr")?))
        }
        #[cfg(feature = "large-he")]
        ModelKind::LargeHe => {
            Ok(WordFreq::deserialize(DATA_LARGE_HE)?.standardizer(Standardizer::new("he")?))
        }
        #[cfg(feature = "large-it")]
        ModelKind::LargeIt => {
            Ok(WordFreq::deserialize(DATA_LARGE_IT)?.standardizer(Standardizer::new("it")?))
        }
        #[cfg(feature = "large-ja")]
        ModelKind::LargeJa => {
            Ok(WordFreq::deserialize(DATA_LARGE_JA)?.standardizer(Standardizer::new("ja")?))
        }
        #[cfg(feature = "large-mk")]
        ModelKind::LargeMk => {
            Ok(WordFreq::deserialize(DATA_LARGE_MK)?.standardizer(Standardizer::new("mk")?))
        }
        #[cfg(feature = "large-nb")]
        ModelKind::LargeNb => {
            Ok(WordFreq::deserialize(DATA_LARGE_NB)?.standardizer(Standardizer::new("nb")?))
        }
        #[cfg(feature = "large-nl")]
        ModelKind::LargeNl => {
            Ok(WordFreq::deserialize(DATA_LARGE_NL)?.standardizer(Standardizer::new("nl")?))
        }
        #[cfg(feature = "large-pl")]
        ModelKind::LargePl => {
            Ok(WordFreq::deserialize(DATA_LARGE_PL)?.standardizer(Standardizer::new("pl")?))
        }
        #[cfg(feature = "large-pt")]
        ModelKind::LargePt => {
            Ok(WordFreq::deserialize(DATA_LARGE_PT)?.standardizer(Standardizer::new("pt")?))
        }
        #[cfg(feature = "large-ru")]
        ModelKind::LargeRu => {
            Ok(WordFreq::deserialize(DATA_LARGE_RU)?.standardizer(Standardizer::new("ru")?))
        }
        #[cfg(feature = "large-sv")]
        ModelKind::LargeSv => {
            Ok(WordFreq::deserialize(DATA_LARGE_SV)?.standardizer(Standardizer::new("sv")?))
        }
        #[cfg(feature = "large-uk")]
        ModelKind::LargeUk => {
            Ok(WordFreq::deserialize(DATA_LARGE_UK)?.standardizer(Standardizer::new("uk")?))
        }
        #[cfg(feature = "large-zh")]
        ModelKind::LargeZh => {
            Ok(WordFreq::deserialize(DATA_LARGE_ZH)?.standardizer(Standardizer::new("zh")?))
        }
        #[cfg(feature = "small-ar")]
        ModelKind::SmallAr => {
            Ok(WordFreq::deserialize(DATA_SMALL_AR)?.standardizer(Standardizer::new("ar")?))
        }
        #[cfg(feature = "small-bg")]
        ModelKind::SmallBg => {
            Ok(WordFreq::deserialize(DATA_SMALL_BG)?.standardizer(Standardizer::new("bg")?))
        }
        #[cfg(feature = "small-bn")]
        ModelKind::SmallBn => {
            Ok(WordFreq::deserialize(DATA_SMALL_BN)?.standardizer(Standardizer::new("bn")?))
        }
        #[cfg(feature = "small-ca")]
        ModelKind::SmallCa => {
            Ok(WordFreq::deserialize(DATA_SMALL_CA)?.standardizer(Standardizer::new("ca")?))
        }
        #[cfg(feature = "small-cs")]
        ModelKind::SmallCs => {
            Ok(WordFreq::deserialize(DATA_SMALL_CS)?.standardizer(Standardizer::new("cs")?))
        }
        #[cfg(feature = "small-da")]
        ModelKind::SmallDa => {
            Ok(WordFreq::deserialize(DATA_SMALL_DA)?.standardizer(Standardizer::new("da")?))
        }
        #[cfg(feature = "small-de")]
        ModelKind::SmallDe => {
            Ok(WordFreq::deserialize(DATA_SMALL_DE)?.standardizer(Standardizer::new("de")?))
        }
        #[cfg(feature = "small-el")]
        ModelKind::SmallEl => {
            Ok(WordFreq::deserialize(DATA_SMALL_EL)?.standardizer(Standardizer::new("el")?))
        }
        #[cfg(feature = "small-en")]
        ModelKind::SmallEn => {
            Ok(WordFreq::deserialize(DATA_SMALL_EN)?.standardizer(Standardizer::new("en")?))
        }
        #[cfg(feature = "small-es")]
        ModelKind::SmallEs => {
            Ok(WordFreq::deserialize(DATA_SMALL_ES)?.standardizer(Standardizer::new("es")?))
        }
        #[cfg(feature = "small-fa")]
        ModelKind::SmallFa => {
            Ok(WordFreq::deserialize(DATA_SMALL_FA)?.standardizer(Standardizer::new("fa")?))
        }
        #[cfg(feature = "small-fi")]
        ModelKind::SmallFi => {
            Ok(WordFreq::deserialize(DATA_SMALL_FI)?.standardizer(Standardizer::new("fi")?))
        }
        #[cfg(feature = "small-fil")]
        ModelKind::SmallFil => {
            Ok(WordFreq::deserialize(DATA_SMALL_FIL)?.standardizer(Standardizer::new("fil")?))
        }
        #[cfg(feature = "small-fr")]
        ModelKind::SmallFr => {
            Ok(WordFreq::deserialize(DATA_SMALL_FR)?.standardizer(Standardizer::new("fr")?))
        }
        #[cfg(feature = "small-he")]
        ModelKind::SmallHe => {
            Ok(WordFreq::deserialize(DATA_SMALL_HE)?.standardizer(Standardizer::new("he")?))
        }
        #[cfg(feature = "small-hi")]
        ModelKind::SmallHi => {
            Ok(WordFreq::deserialize(DATA_SMALL_HI)?.standardizer(Standardizer::new("hi")?))
        }
        #[cfg(feature = "small-hu")]
        ModelKind::SmallHu => {
            Ok(WordFreq::deserialize(DATA_SMALL_HU)?.standardizer(Standardizer::new("hu")?))
        }
        #[cfg(feature = "small-id")]
        ModelKind::SmallId => {
            Ok(WordFreq::deserialize(DATA_SMALL_ID)?.standardizer(Standardizer::new("id")?))
        }
        #[cfg(feature = "small-is")]
        ModelKind::SmallIs => {
            Ok(WordFreq::deserialize(DATA_SMALL_IS)?.standardizer(Standardizer::new("is")?))
        }
        #[cfg(feature = "small-it")]
        ModelKind::SmallIt => {
            Ok(WordFreq::deserialize(DATA_SMALL_IT)?.standardizer(Standardizer::new("it")?))
        }
        #[cfg(feature = "small-ja")]
        ModelKind::SmallJa => {
            Ok(WordFreq::deserialize(DATA_SMALL_JA)?.standardizer(Standardizer::new("ja")?))
        }
        #[cfg(feature = "small-ko")]
        ModelKind::SmallKo => {
            Ok(WordFreq::deserialize(DATA_SMALL_KO)?.standardizer(Standardizer::new("ko")?))
        }
        #[cfg(feature = "small-lt")]
        ModelKind::SmallLt => {
            Ok(WordFreq::deserialize(DATA_SMALL_LT)?.standardizer(Standardizer::new("lt")?))
        }
        #[cfg(feature = "small-lv")]
        ModelKind::SmallLv => {
            Ok(WordFreq::deserialize(DATA_SMALL_LV)?.standardizer(Standardizer::new("lv")?))
        }
        #[cfg(feature = "small-mk")]
        ModelKind::SmallMk => {
            Ok(WordFreq::deserialize(DATA_SMALL_MK)?.standardizer(Standardizer::new("mk")?))
        }
        #[cfg(feature = "small-ms")]
        ModelKind::SmallMs => {
            Ok(WordFreq::deserialize(DATA_SMALL_MS)?.standardizer(Standardizer::new("ms")?))
        }
        #[cfg(feature = "small-nb")]
        ModelKind::SmallNb => {
            Ok(WordFreq::deserialize(DATA_SMALL_NB)?.standardizer(Standardizer::new("nb")?))
        }
        #[cfg(feature = "small-nl")]
        ModelKind::SmallNl => {
            Ok(WordFreq::deserialize(DATA_SMALL_NL)?.standardizer(Standardizer::new("nl")?))
        }
        #[cfg(feature = "small-pl")]
        ModelKind::SmallPl => {
            Ok(WordFreq::deserialize(DATA_SMALL_PL)?.standardizer(Standardizer::new("pl")?))
        }
        #[cfg(feature = "small-pt")]
        ModelKind::SmallPt => {
            Ok(WordFreq::deserialize(DATA_SMALL_PT)?.standardizer(Standardizer::new("pt")?))
        }
        #[cfg(feature = "small-ro")]
        ModelKind::SmallRo => {
            Ok(WordFreq::deserialize(DATA_SMALL_RO)?.standardizer(Standardizer::new("ro")?))
        }
        #[cfg(feature = "small-ru")]
        ModelKind::SmallRu => {
            Ok(WordFreq::deserialize(DATA_SMALL_RU)?.standardizer(Standardizer::new("ru")?))
        }
        #[cfg(feature = "small-sh")]
        ModelKind::SmallSh => {
            Ok(WordFreq::deserialize(DATA_SMALL_SH)?.standardizer(Standardizer::new("sh")?))
        }
        #[cfg(feature = "small-sk")]
        ModelKind::SmallSk => {
            Ok(WordFreq::deserialize(DATA_SMALL_SK)?.standardizer(Standardizer::new("sk")?))
        }
        #[cfg(feature = "small-sl")]
        ModelKind::SmallSl => {
            Ok(WordFreq::deserialize(DATA_SMALL_SL)?.standardizer(Standardizer::new("sl")?))
        }
        #[cfg(feature = "small-sv")]
        ModelKind::SmallSv => {
            Ok(WordFreq::deserialize(DATA_SMALL_SV)?.standardizer(Standardizer::new("sv")?))
        }
        #[cfg(feature = "small-ta")]
        ModelKind::SmallTa => {
            Ok(WordFreq::deserialize(DATA_SMALL_TA)?.standardizer(Standardizer::new("ta")?))
        }
        #[cfg(feature = "small-tr")]
        ModelKind::SmallTr => {
            Ok(WordFreq::deserialize(DATA_SMALL_TR)?.standardizer(Standardizer::new("tr")?))
        }
        #[cfg(feature = "small-uk")]
        ModelKind::SmallUk => {
            Ok(WordFreq::deserialize(DATA_SMALL_UK)?.standardizer(Standardizer::new("uk")?))
        }
        #[cfg(feature = "small-ur")]
        ModelKind::SmallUr => {
            Ok(WordFreq::deserialize(DATA_SMALL_UR)?.standardizer(Standardizer::new("ur")?))
        }
        #[cfg(feature = "small-vi")]
        ModelKind::SmallVi => {
            Ok(WordFreq::deserialize(DATA_SMALL_VI)?.standardizer(Standardizer::new("vi")?))
        }
        #[cfg(feature = "small-zh")]
        ModelKind::SmallZh => {
            Ok(WordFreq::deserialize(DATA_SMALL_ZH)?.standardizer(Standardizer::new("zh")?))
        }
    }
}
