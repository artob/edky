GEM = gem
READMER = readmer

all: README.md

.gemspec.yaml: edky.gemspec
	$(GEM) build -o $@.gem $<
	$(GEM) spec $@.gem > $@
	@rm -f $@.gem

README.md: ../.config/readmer/ruby/README.md.liquid .gemspec.yaml
	$(READMER) render $< > $@

clean:
	rm -rf pkg .bundle *.gem
	find . -maxdepth 1 -name '*.gem' -exec rm -f {} + || true

maintainer-clean:
	rm -f Gemfile.lock

.PHONY: all clean maintainer-clean
.SECONDARY:
.SUFFIXES:
