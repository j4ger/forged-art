dev:
	cargo leptos build
	zellij run -- pnpm exec unocss src/**/*.rs src/**/**/*.rs src/app.rs src/main.rs --out-file public/uno.css --watch
	zellij run -- cargo leptos watch
