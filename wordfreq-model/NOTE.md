# Development notes

Here we put some notes for the development of this library (especially for myself).

## Model generation

Download the wordfreq repository and checkout the version 3.0.2 (if you want to reproduce our environment).

```shell
$ wget https://zenodo.org/record/7199437/files/rspeer/wordfreq-v3.0.2.zip
$ unzip wordfreq-v3.0.2.zip
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
