SHELL = /bin/sh
PROFILE = lto

.PHONY: all build 20*

all:
	@for year in 20*; do $(MAKE) --no-print-directory $$year || exit 1; done

build:
	@for year in 20*; do cargo build --profile $(PROFILE) -p advent-$$year || exit 1; done

20*:
	@cargo build --profile $(PROFILE) -p advent-$@
	@for source in $@/src/bin/day*; do \
	     day=`basename $$source .rs`; \
	     $(MAKE) -s $@/input/$$day.txt; \
	     echo -e '\n\x1b[01m'$@ - $$day'\x1b[0m'; \
	     target/$(PROFILE)/$$day $@/input/$$day.txt || exit 1; \
	 done

2024/input/day%.txt:
	curl -s --cookie session=$(shell cat TOKEN) 'https://adventofcode.com/2024/day/$*/input' > $@
