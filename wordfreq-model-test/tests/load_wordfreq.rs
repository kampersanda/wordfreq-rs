use wordfreq_model::load_wordfreq;
use wordfreq_model::ModelKind;

#[test]
fn test_large_ar() {
    assert!(load_wordfreq(ModelKind::LargeAr).is_ok());
}
#[test]
fn test_large_bn() {
    assert!(load_wordfreq(ModelKind::LargeBn).is_ok());
}
#[test]
fn test_large_ca() {
    assert!(load_wordfreq(ModelKind::LargeCa).is_ok());
}
#[test]
fn test_large_cs() {
    assert!(load_wordfreq(ModelKind::LargeCs).is_ok());
}
#[test]
fn test_large_de() {
    assert!(load_wordfreq(ModelKind::LargeDe).is_ok());
}
#[test]
fn test_large_en() {
    assert!(load_wordfreq(ModelKind::LargeEn).is_ok());
}
#[test]
fn test_large_es() {
    assert!(load_wordfreq(ModelKind::LargeEs).is_ok());
}
#[test]
fn test_large_fi() {
    assert!(load_wordfreq(ModelKind::LargeFi).is_ok());
}
#[test]
fn test_large_fr() {
    assert!(load_wordfreq(ModelKind::LargeFr).is_ok());
}
#[test]
fn test_large_he() {
    assert!(load_wordfreq(ModelKind::LargeHe).is_ok());
}
#[test]
fn test_large_it() {
    assert!(load_wordfreq(ModelKind::LargeIt).is_ok());
}
#[test]
fn test_large_ja() {
    assert!(load_wordfreq(ModelKind::LargeJa).is_ok());
}
#[test]
fn test_large_mk() {
    assert!(load_wordfreq(ModelKind::LargeMk).is_ok());
}
#[test]
fn test_large_nb() {
    assert!(load_wordfreq(ModelKind::LargeNb).is_ok());
}
#[test]
fn test_large_nl() {
    assert!(load_wordfreq(ModelKind::LargeNl).is_ok());
}
#[test]
fn test_large_pl() {
    assert!(load_wordfreq(ModelKind::LargePl).is_ok());
}
#[test]
fn test_large_pt() {
    assert!(load_wordfreq(ModelKind::LargePt).is_ok());
}
#[test]
fn test_large_ru() {
    assert!(load_wordfreq(ModelKind::LargeRu).is_ok());
}
#[test]
fn test_large_sv() {
    assert!(load_wordfreq(ModelKind::LargeSv).is_ok());
}
#[test]
fn test_large_uk() {
    assert!(load_wordfreq(ModelKind::LargeUk).is_ok());
}
#[test]
fn test_large_zh() {
    assert!(load_wordfreq(ModelKind::LargeZh).is_ok());
}
#[test]
fn test_small_ar() {
    assert!(load_wordfreq(ModelKind::SmallAr).is_ok());
}
#[test]
fn test_small_bg() {
    assert!(load_wordfreq(ModelKind::SmallBg).is_ok());
}
#[test]
fn test_small_bn() {
    assert!(load_wordfreq(ModelKind::SmallBn).is_ok());
}
#[test]
fn test_small_ca() {
    assert!(load_wordfreq(ModelKind::SmallCa).is_ok());
}
#[test]
fn test_small_cs() {
    assert!(load_wordfreq(ModelKind::SmallCs).is_ok());
}
#[test]
fn test_small_da() {
    assert!(load_wordfreq(ModelKind::SmallDa).is_ok());
}
#[test]
fn test_small_de() {
    assert!(load_wordfreq(ModelKind::SmallDe).is_ok());
}
#[test]
fn test_small_el() {
    assert!(load_wordfreq(ModelKind::SmallEl).is_ok());
}
#[test]
fn test_small_en() {
    assert!(load_wordfreq(ModelKind::SmallEn).is_ok());
}
#[test]
fn test_small_es() {
    assert!(load_wordfreq(ModelKind::SmallEs).is_ok());
}
#[test]
fn test_small_fa() {
    assert!(load_wordfreq(ModelKind::SmallFa).is_ok());
}
#[test]
fn test_small_fi() {
    assert!(load_wordfreq(ModelKind::SmallFi).is_ok());
}
#[test]
fn test_small_fil() {
    assert!(load_wordfreq(ModelKind::SmallFil).is_ok());
}
#[test]
fn test_small_fr() {
    assert!(load_wordfreq(ModelKind::SmallFr).is_ok());
}
#[test]
fn test_small_he() {
    assert!(load_wordfreq(ModelKind::SmallHe).is_ok());
}
#[test]
fn test_small_hi() {
    assert!(load_wordfreq(ModelKind::SmallHi).is_ok());
}
#[test]
fn test_small_hu() {
    assert!(load_wordfreq(ModelKind::SmallHu).is_ok());
}
#[test]
fn test_small_id() {
    assert!(load_wordfreq(ModelKind::SmallId).is_ok());
}
#[test]
fn test_small_is() {
    assert!(load_wordfreq(ModelKind::SmallIs).is_ok());
}
#[test]
fn test_small_it() {
    assert!(load_wordfreq(ModelKind::SmallIt).is_ok());
}
#[test]
fn test_small_ja() {
    assert!(load_wordfreq(ModelKind::SmallJa).is_ok());
}
#[test]
fn test_small_ko() {
    assert!(load_wordfreq(ModelKind::SmallKo).is_ok());
}
#[test]
fn test_small_lt() {
    assert!(load_wordfreq(ModelKind::SmallLt).is_ok());
}
#[test]
fn test_small_lv() {
    assert!(load_wordfreq(ModelKind::SmallLv).is_ok());
}
#[test]
fn test_small_mk() {
    assert!(load_wordfreq(ModelKind::SmallMk).is_ok());
}
#[test]
fn test_small_ms() {
    assert!(load_wordfreq(ModelKind::SmallMs).is_ok());
}
#[test]
fn test_small_nb() {
    assert!(load_wordfreq(ModelKind::SmallNb).is_ok());
}
#[test]
fn test_small_nl() {
    assert!(load_wordfreq(ModelKind::SmallNl).is_ok());
}
#[test]
fn test_small_pl() {
    assert!(load_wordfreq(ModelKind::SmallPl).is_ok());
}
#[test]
fn test_small_pt() {
    assert!(load_wordfreq(ModelKind::SmallPt).is_ok());
}
#[test]
fn test_small_ro() {
    assert!(load_wordfreq(ModelKind::SmallRo).is_ok());
}
#[test]
fn test_small_ru() {
    assert!(load_wordfreq(ModelKind::SmallRu).is_ok());
}
#[test]
fn test_small_sh() {
    assert!(load_wordfreq(ModelKind::SmallSh).is_ok());
}
#[test]
fn test_small_sk() {
    assert!(load_wordfreq(ModelKind::SmallSk).is_ok());
}
#[test]
fn test_small_sl() {
    assert!(load_wordfreq(ModelKind::SmallSl).is_ok());
}
#[test]
fn test_small_sv() {
    assert!(load_wordfreq(ModelKind::SmallSv).is_ok());
}
#[test]
fn test_small_ta() {
    assert!(load_wordfreq(ModelKind::SmallTa).is_ok());
}
#[test]
fn test_small_tr() {
    assert!(load_wordfreq(ModelKind::SmallTr).is_ok());
}
#[test]
fn test_small_uk() {
    assert!(load_wordfreq(ModelKind::SmallUk).is_ok());
}
#[test]
fn test_small_ur() {
    assert!(load_wordfreq(ModelKind::SmallUr).is_ok());
}
#[test]
fn test_small_vi() {
    assert!(load_wordfreq(ModelKind::SmallVi).is_ok());
}
#[test]
fn test_small_zh() {
    assert!(load_wordfreq(ModelKind::SmallZh).is_ok());
}
