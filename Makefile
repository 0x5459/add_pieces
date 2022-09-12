build:
	cargo build --release
	cp $(shell cargo metadata --format-version=1 --manifest-path=./Cargo.toml | jq -r ".target_directory")/release/add_pieces ./add_pieces