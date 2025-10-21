CARGO	?=	$(shell which cargo)
INSTALL	?=	$(shell which install)
RM	?=	rm -f

PREFIX	?=	/usr/local
BINARY	:=	cs2

all:	target/debug/$(BINARY)

.PHONY:	cargo
cargo:
ifeq (, $(CARGO))
	@echo "Impossible to find cargo"
	@exit 1
endif

target/debug/$(BINARY):	cargo
	$(CARGO) build

target/release/$(BINARY):	cargo
	$(CARGO) build --release

.PHONY:	debug release
debug:	target/debug/$(BINARY)
release:	target/release/$(BINARY)

fclean:
	$(RM) target/debug/$(BINARY)
	$(RM) target/release/$(BINARY)

install:
	install -Dm755 target/release/$(BINARY) $(PREFIX)/bin/$(BINARY)
