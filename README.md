# wordfreq-rs

This library is a yet another Rust port of [wordfreq](https://github.com/rspeer/wordfreq),
allowing you to look up the frequencies of words in many languages.

## Repository structure

This repository contains three crates:

- [wordfreq](wordfreq) is a crate that provides an API to lookup word frequencies.
- [wordfreq-model](wordfreq-model) is a crate that provides a loader for pre-compiled wordfreq models.
- [wordfreq-example](wordfreq-example) is a crate that provides an example program.

## Getting started

As with the original wordfreq, you can start looking for word frequencies right away.

```rust
use wordfreq_model::load_wordfreq;
use wordfreq_model::ModelKind;

fn main() {
    let wf_lgen = load_wordfreq(ModelKind::LargeEn).unwrap();
    let wf_lgfr = load_wordfreq(ModelKind::LargeFr).unwrap();

    println!("cafe(en) = {:?}", wf_lgen.word_frequency("cafe"));
    // => 1.2481286e-5
    println!("café(en) = {:?}", wf_lgen.word_frequency("café"));
    // => 5.705049e-6
    println!("cafe(fr) = {:?}", wf_lgfr.word_frequency("cafe"));
    // => 1.533655e-6
    println!("café(fr) = {:?}", wf_lgfr.word_frequency("café"));
    // => 5.8307935e-5
}
```

We recommend to see the [documentation of wordfreq-model](https://docs.rs/wordfreq-model/) for quick start.
Then, [wordfreq-example](wordfreq-example) will be helpful to understand the behavior of this library.
More detailed specifications are available in the [documentation of wordfreq](https://docs.rs/wordfreq/).

## Licensing

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
