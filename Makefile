.PHONY: othello
othello: clean
	@cargo build --release
	@mv target/release/othello .

with_random: clean
	@cargo build --release --features with_random
	@mv target/release/othello .

clean:
	@rm -f othello
