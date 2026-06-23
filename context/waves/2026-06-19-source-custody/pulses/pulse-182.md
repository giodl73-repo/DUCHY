# Pulse 182: Mantua Savoy HRE Source Custody

## Intent

Add reviewed text support for two existing active HRE duchy parentage facts
without changing hierarchy shape.

## Changes

- Add `src-wikipedia-duchy-mantua`.
- Attach it to Duchy of Mantua -> Holy Roman Empire for `1530..1797`.
- Add `src-wikipedia-duchy-savoy`.
- Attach it to Duchy of Savoy -> Holy Roman Empire for `1416..1806`.
- Add `data/staging/duchy-milan-hre-held-candidate.md`.

## Boundary

This pulse adds source support only for Mantua and Savoy. Milan is held because
the article lead is not enough to safely support a simple full-span HRE
parentage claim without Italian/Habsburg split review.

## Current State

- reviewed sources: 448
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
