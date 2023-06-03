fn main() {
    let wf = wordfreq_model::load_wordfreq(wordfreq_model::ModelKind::LargeEn).unwrap();
    println!("{}", wf.get("the"));
}
