# mdBook Pikchr

A [mdBook][1] preprocessor to render [Pikchr][2] code blocks as images in your book.

Taken from the [Pikchr][2] homepage:

> Pikchr (pronounced like "picture") is a [PIC][3]-like markup
> language for diagrams in technical documentation.  Pikchr is
> designed to be embedded in [fenced code blocks][4] of
> Markdown (or in similar mechanisms in other markup languages)
> to provide a convenient means of showing diagrams.

This [crate][5] use builtin copy of the [pikchr.c][6] file downloaded from that website.

## How it's work

<p align="center">
  <img src="https://raw.githubusercontent.com/podsvirov/mdbook-pikchr/master/demos/diagram.svg" width="424">
</p>

## Installation

Install it via `cargo`:

```shell
$ cargo install mdbook-pikchr
```

## Configuration

Just type next line in your `book.toml` file:

```toml
[preprocessor.pikchr]
```

## Source code

Source code available on [GitHub][7].

[1]: https://rust-lang.github.io/mdBook
[2]: https://pikchr.org
[3]: https://en.wikipedia.org/wiki/Pic_language
[4]: https://spec.commonmark.org/0.29/#fenced-code-blocks
[5]: https://crates.io/crates/mdbook-pikchr
[6]: https://pikchr.org/home/file?name=pikchr.c&ci=6d40a5f041311bbe
[7]: https://github.com/podsvirov/mdbook-pikchr
