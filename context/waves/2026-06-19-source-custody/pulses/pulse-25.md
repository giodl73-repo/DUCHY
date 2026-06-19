# Pulse 25: Shard Index

## Intent

Give source-custody reviewers a compact audit table for large manifest shard
directories.

## Changes

- Extend `duchy-import shard-manifest` to write `INDEX.md`.
- Include source manifest path, total candidate count, chunk size, shard count,
  and per-shard status counts.
- Keep shard files parseable manifests and keep the index as Markdown review
  evidence.

## Review Boundary

The shard index is a review artifact. It does not promote source records,
accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- shard-manifest data/staging/example.manifest <temp dir> 2`
- inspect generated `<temp dir>/INDEX.md`
- `cargo fmt --check`
- `git diff --check`
