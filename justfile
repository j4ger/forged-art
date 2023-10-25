dev:
	zellij run -- pnpm dlx @unocss/cli src/**/*.rs src/app.rs src/main.rs --out-file public/uno.css --watch
	zellij run -- cargo leptos watch
