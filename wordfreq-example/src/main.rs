use wordfreq_model::load_wordfreq;
use wordfreq_model::ModelKind;

fn main() {
    let wf_lgen = load_wordfreq(ModelKind::LargeEn).unwrap();
    let wf_lgfr = load_wordfreq(ModelKind::LargeFr).unwrap();
    let wf_smen = load_wordfreq(ModelKind::SmallEn).unwrap();

    println!("=== word_frequency ===");
    println!("cafe(en) = {:?}", wf_lgen.word_frequency("cafe"));
    println!("café(en) = {:?}", wf_lgen.word_frequency("café"));
    println!("cafe(fr) = {:?}", wf_lgfr.word_frequency("cafe"));
    println!("café(fr) = {:?}", wf_lgfr.word_frequency("café"));

    println!("=== zipf_frequency ===");
    println!("      the = {:?}", wf_lgen.zipf_frequency("the"));
    println!("     word = {:?}", wf_lgen.zipf_frequency("word"));
    println!("frequency = {:?}", wf_lgen.zipf_frequency("frequency"));
    println!(" zipf(lg) = {:?}", wf_lgen.zipf_frequency("zipf"));
    println!(" zipf(sm) = {:?}", wf_smen.zipf_frequency("zipf"));

    println!("=== Numbers (Years) ===");
    println!("2022 = {:?}", wf_lgen.word_frequency("2022"));
    println!("1922 = {:?}", wf_lgen.word_frequency("1922"));
    println!("1022 = {:?}", wf_lgen.word_frequency("1022"));

    println!("=== Numbers (Others) ===");
    println!("  90210 = {:?}", wf_lgen.word_frequency("90210"));
    println!("  92222 = {:?}", wf_lgen.word_frequency("92222"));
    println!("802.11n = {:?}", wf_lgen.word_frequency("802.11n"));
    println!("899.19n = {:?}", wf_lgen.word_frequency("899.19n"));
}
