#!/bin/bash

set -eux

data_dir="rspeer-wordfreq-372f6db/wordfreq/data/"

targets=(
    "large_ar"
    "large_bn"
    "large_ca"
    "large_cs"
    "large_de"
    "large_en"
    "large_es"
    "large_fi"
    "large_fr"
    "large_he"
    "large_it"
    "large_ja"
    "large_mk"
    "large_nb"
    "large_nl"
    "large_pl"
    "large_pt"
    "large_ru"
    "large_sv"
    "large_uk"
    "large_zh"
    "small_ar"
    "small_bg"
    "small_bn"
    "small_ca"
    "small_cs"
    "small_da"
    "small_de"
    "small_el"
    "small_en"
    "small_es"
    "small_fa"
    "small_fi"
    "small_fil"
    "small_fr"
    "small_he"
    "small_hi"
    "small_hu"
    "small_id"
    "small_is"
    "small_it"
    "small_ja"
    "small_ko"
    "small_lt"
    "small_lv"
    "small_mk"
    "small_ms"
    "small_nb"
    "small_nl"
    "small_pl"
    "small_pt"
    "small_ro"
    "small_ru"
    "small_sh"
    "small_sk"
    "small_sl"
    "small_sv"
    "small_ta"
    "small_tr"
    "small_uk"
    "small_ur"
    "small_vi"
    "small_zh"
)

output_dir="resources"
mkdir ${output_dir}

for target in "${targets[@]}" ; do
    echo "Processing ${target}"
    python scripts/parse_msgpack.py ${data_dir}/${target}.msgpack.gz ${output_dir}/${target}.txt.zst
done
