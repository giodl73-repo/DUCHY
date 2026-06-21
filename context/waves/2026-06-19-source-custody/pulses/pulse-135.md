# Pulse 135: County Scale 004 Farm Review

## Intent

Run the first farmed county-scale shard through the agent output contract and
capture blockers before promoting any new facts.

## Changes

- Add `data/staging/county-agent-results/county-scale-004-results.tsv`.
- Add `data/staging/county-agent-results/county-scale-004-review.md`.
- Record shard decisions and follow-up source/rank-policy needs.

## Boundary

No accepted sources, title facts, parentage facts, source code, or queue inputs
are modified by this shard review.

## Current State

- shard rows reviewed: 50
- already accepted seeds: 1
- rank policy blockers: 3
- source resolution blockers: 31
- rejected bad leads: 15

## Notable Rows

- Brycheiniog remains blocked because the plausible Deheubarth relation is
  kingdom-to-kingdom and current parentage validation requires parent rank above
  child rank.
- Byzantion remains blocked as ancient-city/polis semantics, not a current
  materialized county title rank.
- Cetatea-Alba remains an already accepted metrics seed.

## Validation

- `Import-Csv data/staging/county-agent-results/county-scale-004-results.tsv -Delimiter "`t" | Measure-Object`
- `Import-Csv data/staging/county-agent-results/county-scale-004-results.tsv -Delimiter "`t" | Group-Object decision`
- `git diff --check`
