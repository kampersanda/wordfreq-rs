fn main() {
    let wf = wordfreq_data::load_wordfreq(wordfreq_data::ModelKind::LargeEn).unwrap();
    println!("{}", wf.get("the"));
}
