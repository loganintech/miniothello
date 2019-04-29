.PHONY: othello
othello: clean
	@cargo build --release
	@mv target/release/othello .
	@echo "\nThe othello command was added to this folder"

with_random: clean
	@cargo build --release --features with_random
	@mv target/release/othello .
	@echo "\nThe othello command was added to this folder"

clean:
	@rm -f othello

deepclean:
	@rm -f othello
	@rm -r target
