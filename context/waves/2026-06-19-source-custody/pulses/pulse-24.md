# Pulse 24: Manifest Sharding

## Intent

Split larger candidate import queues into fixed-size review batches before
source-custody review and fact extraction.

## Changes

- Add `duchy-import shard-manifest <manifest> <output-dir> <chunk-size>`.
- Validate the source manifest before sharding.
- Write `batch-001.manifest` style shard files.
- Reparse and validate every generated shard before writing it.
- Reject zero-sized chunks and empty candidate manifests.

## Review Boundary

Manifest shards are staging artifacts. They do not promote source records,
accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- shard-manifest data/staging/example.manifest <temp dir> 2`
- `cargo fmt --check`
- `git diff --check`
