# types_lte_3gpp

## Generate 3GPP types

```sh
python build.py

#compile only
python build.py --compile
```

## Release

```sh
cargo publish -j 4
# force reduce the number of processes
```

## Notes

- some asn files should be manually patched (see `*_patch` functions in `build.py`)

## License

Licensed under the MIT License - [LICENSE](LICENSE)
