RUSTC=rustc
RUSTDOC=rustdoc
SOURCES=xdr.rs
TEST_HARNESS=xdr_tests

all: test build_lib doc

build_lib:
	$(RUSTC) $(SOURCES)

test: build_harness
	./$(TEST_HARNESS)

build_harness:
	$(RUSTC) --test -o $(TEST_HARNESS) $(SOURCES)

doc: clean_doc
	$(RUSTDOC) $(SOURCES)

clean_doc:
	-@rm -rf doc

clean: clean_doc
	-@rm $(TEST_HARNESS)
	-@rm libxdr*



