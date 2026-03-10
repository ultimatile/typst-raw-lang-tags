# typst-raw-lang-tags

A tiny tool that prints the output of `typst::text::RawElem::languages()`.
Use it to inspect language names and tags (mostly extension-derived tokens)
accepted by Typst `raw` code-block highlighting.

You can see the available tags [here](https://github.com/ultimatile/typst-raw-lang-tags/blob/master/result.md).

## Usage

```bash
cargo run > result.txt
```

Running the command also writes `result.md` (Markdown table format).

## Output Files

- `result.txt`: TSV-like plain text from stdout, in `<Language>\t<tag1,tag2,...>` format.
- `result.md`: The same data formatted as a Markdown table.
These outputs correspond to the Typst version specified in `Cargo.toml`.
