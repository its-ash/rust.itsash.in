.PHONY: run build deploy

run:
	cargo run -p gallery

build:
	cargo build --workspace --release
	cd examples/gallery && trunk build --release --dist ../../docs
	touch docs/.nojekyll
	echo "rust.itsash.in" > docs/CNAME

deploy: build
	git checkout main
	git add -A
	git commit -m "$$(copilot -sp 'Analyze the staged git changes and generate a concise commit message. Output ONLY the commit message. Do not execute any commands. Do not include quotes, markdown, explanation, or bullet points.')"
	git push origin main
