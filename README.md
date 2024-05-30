# types_lte_3gpp [![badge](https://img.shields.io/crates/v/types_lte_3gpp)](https://crates.io/crates/types_lte_3gpp)

## Generate 3GPP types

```sh
python build.py

#compile only
python build.py --compile

#download only
python build.py --download
```

## Release

```sh
cargo publish -j 4
# force reduce the number of processes
```

## Tests

```sh
# install cargo hack
cargo hack test --feature-powerset --all-targets
```

## Notes

- some asn files should be manually patched (see `*_patch` functions in `build.py`)

## License

Licensed under the MIT License - [LICENSE](LICENSE)
