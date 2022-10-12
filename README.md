# mdBook Pikchr

A [mdBook][1] preprocessor to render [Pikchr][2] code blocks as images in your book.

Taken from the [Pikchr][2] homepage:

> Pikchr (pronounced like "picture") is a [PIC][3]-like markup
> language for diagrams in technical documentation.  Pikchr is
> designed to be embedded in [fenced code blocks][4] of
> Markdown (or in similar mechanisms in other markup languages)
> to provide a convenient means of showing diagrams.

This crate use builtin copy of the [pikchr.c][5] file downloaded from that website.

## How it's work

<div style="max-width:397px">
<svg xmlns='http://www.w3.org/2000/svg' viewBox="0 0 397.901 195.84">
<polygon points="146,37 134,41 134,33" style="fill:rgb(0,0,0)"/>
<path d="M2,37L140,37"  style="fill:none;stroke-width:2.16;stroke:rgb(0,0,0);" />
<text x="74" y="25" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Markdown</text>
<text x="74" y="49" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Source</text>
<path d="M161,72L232,72A15 15 0 0 0 247 57L247,17A15 15 0 0 0 232 2L161,2A15 15 0 0 0 146 17L146,57A15 15 0 0 0 161 72Z"  style="fill:none;stroke-width:2.16;stroke:rgb(0,0,0);" />
<text x="196" y="17" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Markdown</text>
<text x="196" y="37" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Formatter</text>
<text x="196" y="57" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">(mdbook)</text>
<polygon points="391,37 379,41 379,33" style="fill:rgb(0,0,0)"/>
<path d="M247,37L385,37"  style="fill:none;stroke-width:2.16;stroke:rgb(0,0,0);" />
<text x="319" y="25" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">HTML+SVG</text>
<text x="319" y="49" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Output</text>
<polygon points="196,72 201,84 192,84" style="fill:rgb(0,0,0)"/>
<polygon points="196,123 192,111 201,111" style="fill:rgb(0,0,0)"/>
<path d="M196,78L196,117"  style="fill:none;stroke-width:2.16;stroke:rgb(0,0,0);" />
<path d="M136,193L256,193A15 15 0 0 0 271 178L271,138A15 15 0 0 0 256 123L136,123A15 15 0 0 0 121 138L121,178A15 15 0 0 0 136 193Z"  style="fill:none;stroke-width:2.16;stroke:rgb(0,0,0);" />
<text x="196" y="138" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Pikchr</text>
<text x="196" y="158" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Preprocessor</text>
<text x="196" y="178" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">(mdbook-pikchr)</text>
</svg>
</div>

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

Source code available on [GitHub][6].

[1]: https://rust-lang.github.io/mdBook
[2]: https://pikchr.org
[3]: https://en.wikipedia.org/wiki/Pic_language
[4]: https://spec.commonmark.org/0.29/#fenced-code-blocks
[5]: https://pikchr.org/home/file?name=pikchr.c&ci=beb9c85f38c9eebb
[6]: https://github.com/podsvirov/mdbook-pikchr
