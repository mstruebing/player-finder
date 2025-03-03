build: build-frontend build-backend

build-frontend: 
	npm run build --prefix frontend

build-extension:
	cd extension && \
		rm -rf dist && \
		elm make src/Main.elm --output=dist/elm.js && \
		cp manifest.json dist/manifest.json && \
		cp background.js dist/background.js && \
		cp content-script.js dist/content-script.js


watch-frontend: build-frontend
	while inotifywait -r frontend/src frontend/assets -e modify; do { make build-frontend; }; done

build-backend:
	cargo build

start-server:
	ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=8000	cargo run

docker:
	docker build -t mstruebing/csportal-player-finder .

format:
	rustfmt --edition 2021 src/main.rs
	npm run --prefix frontend format

test:
	echo TODO

request:
	curl 'http://0.0.0.0:8000/player/https%3A%2F%2Fsteamcommunity.com%2Fid%2Finsi--'
