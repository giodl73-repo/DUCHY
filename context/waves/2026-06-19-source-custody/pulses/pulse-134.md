# Pulse 134: County Parentage Scale Queue

## Intent

Prepare CK3 Europe county search-driver rows for parallel review while keeping
source custody explicit.

## Changes

- Generate `data/staging/county-parentage-scale-queue.tsv`.
- Generate ten 50-row shards under
  `data/staging/county-parentage-scale-shards/`.
- Add `data/staging/county-parentage-scale-queue-summary.md`.
- Add `data/staging/county-parentage-agent-brief.md` with worker rules and an
  output contract.

## Boundary

No historical sources, title facts, parentage facts, or accepted fixtures are
imported. CK3 rows remain search-driver rows only.

## Current State

- queue rows: 500
- shards: 10
- accepted parentage seeds: 4
- title needs parentage review: 1
- rank semantics review rows: 6
- source resolution deferred rows: 462
- source resolution needed rows: 27

## Validation

- `Import-Csv data/staging/county-parentage-scale-queue.tsv -Delimiter "`t" | Group-Object scale_status`
- `Import-Csv data/staging/county-parentage-scale-queue.tsv -Delimiter "`t" | Measure-Object`
- `git diff --check`
