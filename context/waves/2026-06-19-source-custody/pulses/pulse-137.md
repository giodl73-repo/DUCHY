# Pulse 137: County Scale Full Shard Farm Review

## Intent

Finish the first 500-row county-scale farm pass by reviewing the remaining
shards and refreshing the aggregate result summary.

## Changes

- Add shard result and review files for `county-scale-003`.
- Add shard result and review files for `county-scale-005`.
- Add shard result and review files for `county-scale-007`.
- Add shard result and review files for `county-scale-008`.
- Add shard result and review files for `county-scale-010`.
- Refresh `data/staging/county-agent-results/SUMMARY.md`.

## Boundary

No accepted fixtures, source code, or queue inputs are modified. The shard
outputs are review artifacts only.

## Current State

- reviewed shard rows: 500
- already accepted seeds: 13
- ready title/title-follow-up candidates: 21
- rank-policy blockers: 50
- source-resolution blockers: 329
- rejected bad leads: 87
- ready parentage packets: 0

## Ready Title/Follow-Up Candidates

- County of Barcelona.
- Duchy of Benevento.
- Duchy of Brabant.
- Margraviate of Brandenburg.
- Kingdom of Breifne.
- Duchy of Teschen.
- Taifa of Denia.
- Kingdom of Desmond.
- Dorohoi County.
- Free imperial city of Dortmund.
- County Palatine of Durham.
- Kingdom of Dyfed.
- Kingdom of Essex.
- County of Foix.
- County of Forcalquier.
- County of Forez.
- Princely Abbey of Fulda.
- County of Furstenberg.
- County of Girona.
- Komárom County.
- County of La Marche.

## Validation

- `Import-Csv data/staging/county-agent-results/county-scale-*-results.tsv -Delimiter "`t" | Measure-Object`
- `Import-Csv data/staging/county-agent-results/county-scale-*-results.tsv -Delimiter "`t" | Group-Object decision`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `git diff --check`
