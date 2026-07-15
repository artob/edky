READMER = readmer

all: README.md

README.md: ../.config/readmer/ruby/README.md.liquid
	$(READMER) render $< > $@

clean:
	rm -rf pkg .bundle *.gem
	find . -maxdepth 1 -name '*.gem' -exec rm -f {} + || true

maintainer-clean:
	rm -f Gemfile.lock

.PHONY: all clean maintainer-clean
.SECONDARY:
.SUFFIXES:
