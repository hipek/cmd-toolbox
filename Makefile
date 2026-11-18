.PHONY: build install test clean help

help:
	@echo "Available tasks:"
	@awk -F: '/^[[:alnum:]_-]+:/ { print "  make " $$1 }' Makefile | sort -u

build:
	$(MAKE) -C video-project-creator build
	$(MAKE) -C twitch-video-project build

install: build
	$(MAKE) -C video-project-creator install
	$(MAKE) -C twitch-video-project install

test:
	$(MAKE) -C video-project-creator test
	$(MAKE) -C twitch-video-project test

clean:
	find . -type d -name target -exec rm -rf {} + 2>/dev/null || true
