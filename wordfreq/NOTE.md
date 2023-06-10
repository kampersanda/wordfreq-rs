# Development notes

Here we put some notes for the development of this library (especially for myself).

## Generation for LIKELY_SUBTAGS in language.rs

Download the source files of Unicode CLDR v43 and parse `common/supplemental/likelySubtags.xml`.

```
$ http://unicode.org/Public/cldr/43/cldr-common-43.0.zip
$ unzip cldr-common-43.0.zip
$ python parse_likely_subtags.py
```

## Generation for TRADITIONAL_TO_SIMPLIFIED in chinese.rs

Download the source files of wordfreq v3.0.2.

```shell
$ wget https://zenodo.org/record/7199437/files/rspeer/wordfreq-v3.0.2.zip
$ unzip wordfreq-v3.0.2.zip
```

Parse the mapping file.

```shell
$ python -m venv venv
$ source ./venv/bin/activate
$ pip install -r scripts/requirements.txt
$ python scripts/parse_chinese.py rspeer-wordfreq-372f6db/wordfreq/data/_chinese_mapping.msgpack.gz
```
