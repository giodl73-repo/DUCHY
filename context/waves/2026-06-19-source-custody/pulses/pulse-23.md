# Pulse 23: Archive Manifest Cleanup

## Intent

Keep promoted and rejected import candidates as parseable queue audit records
after the active staging manifest is regenerated.

## Changes

- Add `duchy-import archive-manifest <manifest> <output.manifest>`.
- Keep only `promoted` and `rejected` manifest rows in the generated archive.
- Reparse and validate the generated archive before writing it.
- Require at least one archived candidate so empty archive files are not
  mistaken for useful evidence.

## Review Boundary

Archive manifests are audit artifacts. They do not promote source records,
accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- archive-manifest data/staging/example.manifest <temp manifest>`
- `cargo fmt --check`
- `git diff --check`
