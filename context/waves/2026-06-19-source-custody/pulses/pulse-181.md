# Pulse 181: Holland HRE Source Custody

## Intent

Add reviewed text support for one existing active HRE county parentage while
holding a nearby split-parent case for later modeling.

## Changes

- Add `src-wikipedia-county-holland`.
- Attach it to County of Holland -> Holy Roman Empire for `962..1795`.
- Add `data/staging/county-flanders-split-parentage-held-candidate.md`.

## Boundary

This pulse adds source support only for Holland. Flanders is held because the
source indicates French and Imperial fiefdom semantics that DUCHY should not
flatten into one simple full-span parentage claim.

## Current State

- reviewed sources: 446
- reviewed facts: 1327
- active parentage facts: 278
- superseded parentage facts: 2
- rank-skip rows: 220
- bridge rows: 160
- temporal parent conflicts: 0
- snapshot cycle years: 0

## Validation

- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
