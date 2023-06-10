import argparse
import gzip

import msgpack


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=str)
    args = parser.parse_args()

    with gzip.open(args.input, "rb") as infile:
        data = msgpack.load(infile, raw=False, strict_map_key=False)

    print("const TRADITIONAL_TO_SIMPLIFIED: &[(char, char)] = &[")
    for traditional, simplified in data.items():
        print(f"    ('{chr(traditional)}', '{simplified}'),")
    print("];")


if __name__ == "__main__":
    main()
