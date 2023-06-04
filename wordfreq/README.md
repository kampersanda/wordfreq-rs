# wordfreq

This crate is a yet another Rust port of [wordfreq](https://github.com/rspeer/wordfreq),
allowing you to look up the frequencies of words in many languages.

## Documentation

https://docs.rs/wordfreq/

## Licensing

Source code is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

In addition, this crate contains model files copied from the [wordfreq](https://github.com/rspeer/wordfreq) repository in the [resources](resources) directory,
which are licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).

## Development guide

We describe how to reproduce the model files and source code (especially for myself).

Download the wordfreq repository and checkout the version 3.0.2 (if you want to reproduce our environment).

```shell
$ git clone https://github.com/rspeer/wordfreq.git wordfreq-orig
$ cd wordfreq-orig
$ git checkout v3.0.2
$ cd ..
```

Convert the model files to this crate's format.

```shell
$ python -m venv venv
$ source ./venv/bin/activate
$ pip install -r scripts/requirements.txt
$ scripts/convert.sh
```

Generate the source code according to the model files.

```shell
$ python scripts/gen_codes.py
```
