# Building documentation locally

The following command will fail miserably

```text
cargo doc
```

if any [dependencies](https://doc.rust-lang.org/cargo/guide/dependencies.html) are present
in [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) because
[`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) uses 
[`--html-in-header`](https://doc.rust-lang.org/rustdoc/command-line-arguments.html#--html-in-header-include-more-html-in-head)
[rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) flag for
displaying [LaTeX](https://en.wikibooks.org/wiki/LaTeX/Mathematics) equations. The argument
for the flag is a relative path to the `.html` document and the flag is applied to **all** crates.
Given only our crate surely has `.src/html/docs-header.html`, `rustdoc` cannot find the headers in
other crates and fails building documentation.

Instead, one should run

```
cargo doc --no-deps
```

as it builds the documentation only for our crate.

# Read more
* https://github.com/victe/rust-latex-doc-minimal-example/issues/1
* https://crates.io/crates/include_display_mode_tex