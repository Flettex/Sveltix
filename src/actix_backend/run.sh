cd ../sveltekit_app
node buildScript.js
cd ../actix_backend

export $(cat .env | xargs)
cargo watch -x run