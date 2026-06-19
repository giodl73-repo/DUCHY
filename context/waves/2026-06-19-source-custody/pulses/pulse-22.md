# Pulse 22: Active Manifest Cleanup

## Intent

Regenerate active staging queues after promoted or rejected candidates have been
archived.

## Changes

- Add `duchy-import active-manifest <manifest> <output.manifest>`.
- Keep only `pending` and `reviewed` manifest rows in the generated queue.
- Reparse and validate the generated manifest before writing it.
- Require at least one active candidate so empty queues are not mistaken for
  useful staging evidence.

## Review Boundary

Active manifests are queue cleanup artifacts. They do not promote source
records, accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- active-manifest data/staging/example.manifest <temp manifest>`
- `cargo fmt --check`
- `git diff --check`
