lint:
	cargo clippy
run:
	cargo watch -x run
test_env:
	cd test_env; docker compose up 
test:
	cargo test
build:
	cargo build
clean:
	cargo clean
build_docker:
	docker build -t pet_tracker . -f deployment/Dockerfile