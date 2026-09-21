# `tame-index` local validation issue reproduction

As of version 0.26.3,
[`tame-index`](https://github.com/EmbarkStudios/tame-index)
seems to consider local registries where `.crate` files are missing (as opposed to existing and
being corrupt) valid.
More precisely, calling
[`LocalRegistry::open`](https://docs.rs/tame-index/0.26.3/tame_index/index/local/struct.LocalRegistry.html#method.open)
or
[`LocalRegistry::validate`](https://docs.rs/tame-index/0.26.3/tame_index/index/local/struct.LocalRegistry.html#method.validate)
with the second argument (`validate`) set to `true` returns an `Ok` value value when the registry
root contains no `.crate` file for a crate that has some versions in the index.

## Repository content

In the the directory `bad-registry` there is a crates registry that advertises the crate `krate`
in version `0.1.0`, but does not contain the associated `krate-0.1.0.crate` file.

The program in `src` calls `LocalRegistry::open` on the bad registry, retrieves the information
about the `krate` crate and checks if the associated `.crate` file is present.

In a clean checkout, `cargo run` should produce the following output:
```
name: krate, version: 0.1.0, checksum: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
`bad-registry/krate-0.1.0.crate` exists: false
```

To contrast this with the corrupt file validation, you can run
`touch bad-registry/krate-0.1.0.crate` (or create the file otherwise) and then `cargo run` to get
a validation error:
```
thread 'main' (170355) panicked at src/main.rs:12:72:
called `Result::unwrap()` on an `Err` value: Local(ChecksumMismatch { name: "krate", version: "0.1.0" })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```