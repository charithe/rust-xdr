RUSTC=rustc
SOURCES=xdr.rs
TEST_HARNESS=xdr_tests

all: test build_lib

build_lib:
	$(RUSTC) $(SOURCES)

test: build_harness
	./$(TEST_HARNESS)

build_harness:
	$(RUSTC) --test -o $(TEST_HARNESS) $(SOURCES)

clean:
	-rm $(TEST_HARNESS)
	-rm libxdr*



