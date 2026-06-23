# Pulse 197: Anhalt Dessau Kothen Brunswick-Luneburg Source Custody

## Intent

Add reviewed text support for existing Anhalt-Dessau, Anhalt-Kothen, and
Brunswick-Luneburg parentage facts.

## Changes

- Add `src-wikipedia-anhalt-dessau`.
- Attach it to Anhalt-Dessau -> Holy Roman Empire for `1396..1806`.
- Attach it to Anhalt-Dessau -> German Confederation for `1815..1853`.
- Add `src-wikipedia-anhalt-kothen`.
- Attach it to Anhalt-Kothen -> Holy Roman Empire for `1396..1806`.
- Attach it to Anhalt-Kothen -> German Confederation for `1815..1863`.
- Add `src-wikipedia-duchy-brunswick-luneburg`.
- Attach it to Brunswick-Luneburg -> Holy Roman Empire for `1235..1806`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Anhalt partitions, Anhalt-Pless, Anhalt-Plotzkau, ducal
elevations, line extinctions, Brunswick-Luneburg partitions, successor Hanover
or Brunswick states, Electoral Rhenish Circle, or modern titular usage.

## Current State

- reviewed sources: 483
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
