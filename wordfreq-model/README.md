# wordfreq-model

## For developers

Download the wordfreq repository and checkout the version 3.0.2 (if you want to reproduce our environment).

```shell
$ git clone https://github.com/rspeer/wordfreq.git wordfreq-orig
$ cd wordfreq-orig
$ git checkout v3.0.2
$ cd ..
```

```shell
$ python -m venv venv
$ source ./venv/bin/activate
$ pip install -r scripts/requirements.txt
$ scripts/compile_all.sh
```
