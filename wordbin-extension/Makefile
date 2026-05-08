DIST = dist

build:
	wasm-pack build --target web --out-dir $(DIST)/pkg --release
	cp manifest.json index.html popup.css bootstrap.js $(DIST)/
	cp -r icons $(DIST)/icons

dev:
	wasm-pack build --target web --out-dir $(DIST)/pkg
	cp manifest.json index.html popup.css bootstrap.js $(DIST)/
	cp -r icons $(DIST)/icons

clean:
	rm -rf $(DIST)