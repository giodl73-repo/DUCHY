# Pulse 10: Source-Backed Parentage Review

## Intent

Prepare DUCHY to import reviewed higher-title relations without inventing or
guessing real hierarchy facts.

## Implementation

- Add `source_backed_parentage_from_facts`.
- Extend `source_backed_timeline_from_facts` to materialize parentage facts
  after titles.
- Require parentage facts to cite accepted sources through the same fact gate.
- Require parentage facts to have a span.
- Require both child and parent titles to be materialized before the relation is
  accepted.
- Keep contested parentage blocked from normal materialization.

## Boundary

The current real fixture still has no source-backed parentage claims. Q158445
and Q20135 remain one-step title paths until a reviewed source packet adds a
parent title relation.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
