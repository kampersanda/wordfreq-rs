use wordfreq_model::load_wordfreq;
use wordfreq_model::ModelKind;

fn main() {
    let wf = load_wordfreq(ModelKind::LargeEn).unwrap();
    println!("{:?}", wf.word_frequency("the"));
    println!("{:?}", wf.zipf_frequency("the"));
}
