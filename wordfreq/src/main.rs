fn main() {
    let wf = wordfreq_data::load_wordfreq().unwrap();
    println!("{}", wf.get("the"));
}
