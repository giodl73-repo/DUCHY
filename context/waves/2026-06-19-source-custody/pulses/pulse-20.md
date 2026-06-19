# Pulse 20: Source Stub Generation

## Intent

Convert reviewed manifest rows into source metadata stubs without granting
fact-claim rights automatically.

## Changes

- Add `duchy-import source-stubs <manifest> <output.sources>`.
- Emit parseable blocked source records for reviewed candidates.
- Keep generated stubs review-required before fact extraction or promotion.

## Review Boundary

Generated source stubs are not accepted sources. They intentionally use blocked
review decisions until source custody replaces rights/use fields.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- source-stubs data/staging/example.manifest <temp sources>`
- `cargo fmt --check`
- `git diff --check`
