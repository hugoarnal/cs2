PREFIX	?=	/usr/local
BINARY	:=	cs2

all:	target/debug/$(BINARY)

target/debug/$(BINARY):
	cargo build

target/release/$(BINARY):	fclean
	cargo build --release

.PHONY:	debug release
debug:	target/debug/$(BINARY)
release:	target/release/$(BINARY)

fclean:
	$(RM) target/debug/$(BINARY)
	$(RM) target/release/$(BINARY)

.PHONY: install
install:
	install -Dm755 target/release/$(BINARY) $(PREFIX)/bin/$(BINARY)
	@echo "Make sure that $(PREFIX)/bin is in your PATH"
