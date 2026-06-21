# Pulse 136: County Scale Priority Shard Farm Review

## Intent

Review the remaining rank-semantics-heavy county-scale shards in parallel and
summarize the first 250 farm-reviewed rows.

## Changes

- Add shard result and review files for `county-scale-001`.
- Add shard result and review files for `county-scale-002`.
- Add shard result and review files for `county-scale-006`.
- Add shard result and review files for `county-scale-009`.
- Add `data/staging/county-agent-results/SUMMARY.md`.

## Boundary

No accepted fixtures, source code, or queue inputs are modified. The shard
outputs are review artifacts only.

## Current State

- reviewed shard rows: 250
- already accepted seeds: 5
- ready title/title-follow-up candidates: 6
- rank-policy blockers: 18
- source-resolution blockers: 178
- rejected bad leads: 43
- ready parentage packets: 0

## Ready Title/Follow-Up Candidates

- County of Barcelona.
- Dorohoi County.
- Free imperial city of Dortmund.
- County Palatine of Durham.
- Kingdom of Dyfed.
- Kingdom of Essex.

## Validation

- `Import-Csv data/staging/county-agent-results/county-scale-001-results.tsv -Delimiter "`t" | Measure-Object`
- `Import-Csv data/staging/county-agent-results/county-scale-002-results.tsv -Delimiter "`t" | Measure-Object`
- `Import-Csv data/staging/county-agent-results/county-scale-006-results.tsv -Delimiter "`t" | Measure-Object`
- `Import-Csv data/staging/county-agent-results/county-scale-009-results.tsv -Delimiter "`t" | Measure-Object`
- `Import-Csv data/staging/county-agent-results/county-scale-*-results.tsv -Delimiter "`t" | Group-Object decision`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `git diff --check`
