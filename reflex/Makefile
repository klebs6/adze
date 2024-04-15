RUSTFLAGS       := "-Awarnings -Dunused_must_use"
DEBUG_RUSTFLAGS := "-Awarnings -Dunused_must_use -C debuginfo=2"
CARGO           := MAKEFLAGS= env CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo

ACTIVE_TEST := xxx

ACTIVE_BINARY  := reflex
ACTIVE_PACKAGE := reflex

RUST_LOG       := info
RUST_BACKTRACE := full 

YOUTUBE_URL := https://www.youtube.com/watch?v=r5ipVMV6Uqs

#DEFAULT := all_tests
#DEFAULT := tests
DEFAULT := run
#DEFAULT := gui
#DEFAULT := serve

default: $(DEFAULT)

all: 
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) build 

build:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) build -p $(ACTIVE_PACKAGE)

build_test:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(DEBUG_RUSTFLAGS) $(CARGO) test --no-run -p $(ACTIVE_PACKAGE) $(ACTIVE_TEST)

test:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) test -p $(ACTIVE_PACKAGE) $(ACTIVE_TEST) -- --nocapture

tests:
	RUST_TEST_THREADS=1 RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) test -p $(ACTIVE_PACKAGE) -- --nocapture

all_tests:
	RUST_TEST_THREADS=1 RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) test -- --nocapture

run:
	YOUTUBE_URL=$(YOUTUBE_URL) RUST_BACKTRACE=1 RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) run -p $(ACTIVE_PACKAGE) -- $(ACTIVE_BINARY)

gui:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) build -p reflex
	cd reflex && trunk build

serve:
	cd reflex/dist trunk serve

