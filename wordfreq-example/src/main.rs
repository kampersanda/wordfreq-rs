use wordfreq_model::load_wordfreq;
use wordfreq_model::ModelKind;

fn main() {
    let wf = load_wordfreq(ModelKind::LargeEn).unwrap();
    println!("{:?}", wf.word_frequency("cafe"));
    println!("{:?}", wf.word_frequency("café"));

    println!("{:?}", wf.zipf_frequency("the"));
    println!("{:?}", wf.zipf_frequency("word"));
    println!("{:?}", wf.zipf_frequency("frequency"));
    println!("{:?}", wf.zipf_frequency("zipf"));

    println!("{:?}", wf.word_frequency("2022"));
    println!("{:?}", wf.word_frequency("1922"));
    println!("{:?}", wf.word_frequency("1022"));

    println!("{:?}", wf.word_frequency("90210"));
    println!("{:?}", wf.word_frequency("92222"));
    println!("{:?}", wf.word_frequency("802.11n"));
    println!("{:?}", wf.word_frequency("899.19n"));
}
