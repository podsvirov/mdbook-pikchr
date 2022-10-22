# Alignment

## Global

By default, the generated images appears centered.

You can change default behavior by setting the variable `align` in table `preprocesso.pikchr` at your `book.toml` file .

If you would prefer that the picturs be left-justifed, put the value `"left"`.

If you would prefer that the picturs be right-justifed, put the value `"right"`.

The value `"center"` also possible, but it will do nothing.

## Per-picture

Additionaly you can control alignment for individual picture by adding similar keyword after the `pikchr` class tag.
Thus:

~~~markdown
{{ #include align-left.md }}
~~~

Results in the following:

{{ #include align-left.md }}

Then:

~~~markdown
{{ #include align-center.md }}
~~~

Results in the following:

{{ #include align-center.md }}

And:


~~~markdown
{{ #include align-right.md }}
~~~


Reults in the following:

{{ #include align-right.md }}
