dev:
    cd client && pnpm i && pnpm run build && rm -rf ../server/static && mkdir ../server/static && cp -r ./dist/* ../server/static
    cd server && RUST_LOG=debug cargo run

buildr:
    cd server && cargo build

build:
    cd client && pnpm install --package-lock-only
    cd server && cargo generate-lockfile
    sudo docker build --tag=youraccount/app:$(VER) .

push:
    sudo docker push youraccount/app:$(VER)
