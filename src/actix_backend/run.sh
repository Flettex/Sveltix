cd ../sveltekit_app
pnpm i
pnpm build
cd ../actix_backend

export $(cat .env | xargs)
cargo watch -x run