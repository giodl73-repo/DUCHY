# Pulse 204 - Burgundian Netherlands Partial Replacement Packet

Date: 2026-06-23

## Scope

- Added partial parentage replacement materialization so a narrower reviewed
  replacement can supersede part of an older broad parentage fact while leaving
  uncovered residual spans active in the timeline.
- Added reviewed Burgundian Netherlands text source custody.
- Added bounded replacement parentage for:
  - County of Flanders -> Burgundian Netherlands, 1384..1482.
  - County of Holland -> Burgundian Netherlands, 1433..1482.
  - County of Namur -> Burgundian Netherlands, 1421..1482.

## Results

- Reviewed sources: 520.
- Facts: 1330.
- Titles: 349.
- Active parentage fact rows: 278.
- Superseded parentage facts retained for audit: 5.
- Active fact-row rank skips: 217.
- Timeline: valid.

## Validation

```powershell
cargo test --quiet
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
```

All validation passed with zero temporal parent conflicts and zero snapshot
cycle years.

## Notes

- The graph report still summarizes active fact rows. Timeline materialization
  now has finer behavior for partial supersession than the fact-row count alone
  can show.
- This packet reduces the rank-skip queue by three rows and proves the next
  scale path: source-backed child-to-intermediate packets, not overlap-only
  bridge inference.
