# Pulse 198: Brunswick-Wolfenbuttel Burgundy Nassau Source Custody

## Intent

Add reviewed text support for existing Holy Roman Empire parentage facts.

## Changes

- Add `src-wikipedia-brunswick-wolfenbuttel`.
- Attach it to Brunswick-Wolfenbuttel -> Holy Roman Empire for `1269..1806`.
- Add `src-wikipedia-county-burgundy`.
- Attach it to County of Burgundy -> Holy Roman Empire for `982..1678`.
- Add `src-wikipedia-county-nassau`.
- Attach it to County of Nassau -> Holy Roman Empire for `1160..1806`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Welf partitions, Burgundian personal unions, Burgundian Circle,
Habsburg transfer, French cession, Nassau internal partitions, princely rank,
Confederation of the Rhine, or successor-state claims.

## Current State

- reviewed sources: 486
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
