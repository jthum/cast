# Cast Font Tool

`cast-font-tool` is a workspace utility for downloading font assets that can be installed into egui through `cast-ui`'s `FontStack` APIs.

```sh
cargo run -p cast-font-tool -- google-font Inter --out crates/cast-ui/assets/fonts/inter --weights 400,500,600
```

The tool resolves family names against the `google/fonts` GitHub repository, searches `ofl`, `apache`, and `ufl`, downloads matching `.ttf` files, and copies the family license when available. If static TTF files for the requested weights are not present, it falls back to a variable `wght` TTF file when one exists.

egui expects `.ttf` or `.otf` font bytes. This tool intentionally does not download from the Google Fonts CSS endpoint because that path commonly returns webfont formats such as WOFF2.
