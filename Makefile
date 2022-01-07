.PHONY: debug build run analyze clean

ifdef alt

main := tr_count_incidence
CC := gcc
LDLIBS += -lm

build: $(main)

run: $(main)
	./$(main) $(num_samples) $(filename)

else

debug:
	cargo clippy
	cargo run

build:
	cargo build --release

run:
	cargo run --release

endif

%.csv: .FORCE
	bash -mc "while true; do cargo run --release | sed -z 's/\n/, /' >> "$@"; done"


analyze:
	for f in *.csv; do echo "$$f"; ./csv.r "$$f"; done

clean:
	rm -rf target
	rm -f tr_count_incidence
	rm -f *.o
	rm -f *.csv
	rm -f *.pdf

.PHONY: .FORCE
.FORCE:

