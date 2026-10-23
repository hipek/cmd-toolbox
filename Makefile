.PHONY: build install clean

build:
	$(MAKE) -C video-project-creator build

install: build
	$(MAKE) -C video-project-creator install

clean:
	find . -type d -name target -exec rm -rf {} + 2>/dev/null || true
