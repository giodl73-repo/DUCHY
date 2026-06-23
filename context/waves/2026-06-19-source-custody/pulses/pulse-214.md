# Pulse 214 - Relation Trace Slice

Date: 2026-06-23

## Scope

- Continue `WP-007` after the relation materializer slice.
- Surface typed relation facts in title-path query traces without changing the
  parentage path.

## Results

- Added `relation_context` trace notes to title-path query envelopes.
- Added stable relation-kind labels for trace detail.
- Added year-span formatting for relation trace output.
- Added a regression test proving relation trace context does not mutate the
  returned title path.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
git diff --check
```

## Notes

- Relation facts remain explanatory trace context beside parentage.
- Rank-skip report classification is still the remaining `WP-007` surface.
