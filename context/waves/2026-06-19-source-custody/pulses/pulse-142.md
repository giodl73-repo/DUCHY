# Pulse 142: County Title Harvest Closure

## Intent

Close the 21 ready title/title-follow-up candidates from the county-scale farm
so the next milestone can move to parentage sourcing instead of repeated title
dedupe.

## Changes

- Add `data/staging/county-title-harvest-closure.md`.
- Record the 7 candidates already accepted before the harvest.
- Record the 9 candidates promoted by county title harvest batches 01-04.
- Record the 5 candidates deferred by current source or rank/span policy.

## Boundary

No accepted fixtures or generated parentage queues are modified in this pulse.
The closure artifact is review bookkeeping only.

## Current State

- ready title/title-follow-up candidates reviewed: 21
- already accepted: 7
- promoted in harvest batches: 9
- deferred: 5

## Validation

- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `git diff --check`
