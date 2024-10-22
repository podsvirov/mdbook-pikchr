docs/demos/diagram.svg: docs/demos/diagram.pikchr target/release/pikchr
	target/release/pikchr --svg-only $< > $@

target/release/pikchr: src/pikchr.c
	mkdir -p target/release
	${CC} -O2 -DPIKCHR_SHELL -o $@ $< -lm
