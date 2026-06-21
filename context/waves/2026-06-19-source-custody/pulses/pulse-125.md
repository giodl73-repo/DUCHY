# Pulse 125: Remaining High-Priority Blocker Review Update

## Intent

Update the remaining high-priority blocker review after modern Norway parentage
coverage closes Hordaland.

## Changes

- Update `data/staging/high-parentage-remaining-blockers.md`.
- Remove Q50625 Hordaland from the blocker set.
- Keep Q187035 Principality of Albania and Q779011 Principality of Montenegro
  as unresolved high-priority blockers.

## Boundary

No additional parentage facts are imported in this pulse. Albania and Montenegro
remain blocked until contested/de facto/de jure relation semantics exist.

## Current State

- sources: 410
- facts: 1260
- titles: 336
- parentage facts: 252
- titles with parentage: 207
- titles without parentage: 129
- high-priority parentage gaps remaining: 2

## Validation

- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
