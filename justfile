build-db:
  cd indexer_cli && cargo run -- build-database --reset

update-indexes:
  cd indexer_cli && cargo run -- update-indexes

wasm:
  cd web && pnpm wasm

dev:
  cd web && pnpm dev

dev-code-server:
  cd web && pnpm dev-code-server