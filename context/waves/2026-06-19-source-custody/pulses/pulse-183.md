# Pulse 183: Palatinate Mainz Cologne HRE Source Custody

## Intent

Add reviewed text support for three existing active Holy Roman Empire
parentage facts involving electorates.

## Changes

- Add `src-wikipedia-electoral-palatinate`.
- Attach it to Electoral Palatinate -> Holy Roman Empire for `1085..1803`.
- Add `src-wikipedia-electorate-cologne`.
- Attach it to Electorate of Cologne -> Holy Roman Empire for `962..1803`.
- Add `src-wikipedia-electorate-mainz`.
- Attach it to Electorate of Mainz -> Holy Roman Empire for `962..1803`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import ecclesiastical offices, elector college mechanics, Free City
episodes, French occupation, secularization transfers, or successor claims.

## Current State

- reviewed sources: 451
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
