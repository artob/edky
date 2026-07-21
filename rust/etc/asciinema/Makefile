# See: https://github.com/asciinema/agg

AGG = agg --quiet
AGGFLAGS = --cols 100 --rows 30 --theme dracula --renderer swash

all: install.gif list.gif

install.gif: install.cast
	$(AGG) $(AGGFLAGS) $< $@

list.gif: list.cast
	$(AGG) $(AGGFLAGS) $< $@

clean:
	rm -f *~ *.gif

maintainer-clean: clean
	rm -f *.cast

.PHONY: all clean maintainer-clean
.SECONDARY:
.SUFFIXES:
