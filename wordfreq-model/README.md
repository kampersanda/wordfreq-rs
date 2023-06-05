# wordfreq-model

This crate provides a loader for pre-compiled [wordfreq models](https://github.com/rspeer/wordfreq/tree/v3.0.2#sources-and-supported-languages),
allowing you to easily create wordfreq instances for various languages.

## Documentation

https://docs.rs/wordfreq-model/

## Licensing

The source code is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The model files are distributed [here](https://github.com/kampersanda/wordfreq-rs/releases/tag/models-v1) together with the credits.

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

Generate the source code according to the model files
(maybe this should be done in rust-macro).

```shell
$ python scripts/gen_codes.py
```
