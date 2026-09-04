.PHONY: run build deploy

run:
	cargo run -p gallery

build:
	cargo build --release

deploy: build
	git checkout main
	git add -A
	git commit -m "Deploy: update RustTheme workspace"
	git push origin main
