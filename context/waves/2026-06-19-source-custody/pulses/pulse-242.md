# Pulse 242: German State Relation Batch 02

Date: 2026-06-23

## Scope

Promote a second German state membership relation packet from accepted
text-backed grand-duchy and Saxe-state parentage rows. Keep source-only and
review-limited rows out of this packet.

## Results

- Added `german-state-relation-batch-02` with twelve relation facts:
  - Grand Duchy of Hesse as `confederation_member` for the German
    Confederation and `federal_state_member` for the German Empire.
  - Grand Duchy of Mecklenburg-Schwerin as `confederation_member` for the
    German and North German Confederations and `federal_state_member` for the
    German Empire.
  - Grand Duchy of Mecklenburg-Strelitz as `confederation_member` for the
    German and North German Confederations.
  - Saxe-Altenburg as `imperial_state` for the Holy Roman Empire and
    `confederation_member` for the German Confederation.
  - Saxe-Weimar-Eisenach as `confederation_member` for the German and North
    German Confederations.
  - Saxe-Lauenburg as `imperial_state` for the Holy Roman Empire.
- Regenerated rank-skip, relation, and graph reports.
- Left parentage structure unchanged.

## Measurements

- Sources: 540.
- Facts: 1487.
- Titles: 356.
- Parentage facts: 289.
- Relation facts: 124.
- Rank-skip rows: 231.
- Relation-explained rows: 120.
- Unexplained rank-skip rows: 111.
- Parentage edges: 292.
- Title edge fill: 72.70%.
- Weighted span coverage: 58.72%.
- Temporal parent conflicts: 0.

## Validation

```powershell
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/german-state-relation-batch-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-state-relation-batch-02.sources data/staging/german-state-relation-batch-02.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/german-state-relation-batch-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-state-relation-batch-02.sources data/staging/german-state-relation-batch-02.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
cargo run --quiet --bin duchy-import -- parentage-rank-skip-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-targets.tsv
cargo run --quiet --bin duchy-import -- parentage-rank-skip-relation-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-relation-report.md
cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md
cargo fmt --check
cargo test --quiet
git diff --check
```
