# Configuration

The `preprocessor.pikchr` table used for preprocessor configuration.

Supported variables:

| Variable | Possible values | Default value |
| :--- | :--- | :--- |
| [align](align.md) | "left", "center", "right" | "center" |

Here is an example of what a `book.toml` file might look like:

```toml
[book]
title = "Example book"
authors = ["John Doe"]
description = "The example book covers examples."

[preprocessor.pikchr]
align = "left"
```
