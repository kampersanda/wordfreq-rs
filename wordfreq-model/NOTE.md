# Development notes

Here we put some notes for the development of this library (especially for myself).

## Model generation

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

## Code generation

Generate the source code according to the model files
(maybe this should be done in rust-macro).

```shell
$ python scripts/gen_codes.py
```
