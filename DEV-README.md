# Building documentation locally

Building the documentation for the crate currently requires nightly compiler. Check the details at [`include_display_mode_tex` repo][`include_display_mode_tex`].

The following command will fail miserably

```text
cargo doc --open
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

In addition, in order to avoid compiling nightly [`include_display_mode_tex`] when building on stable toolchain, this dependency had to be feature-gated by `doc` feature.

Instead, one should run

```text
cargo doc --no-deps --features=doc --open
```

as it builds the documentation only for our crate and uses `doc` feature that enables compilation of [`include_display_mode_tex`].

# Read more
* https://github.com/victe/rust-latex-doc-minimal-example/issues/1
* https://crates.io/crates/include_display_mode_tex

[`include_display_mode_tex`]: https://github.com/JohnScience/include_display_mode_tex