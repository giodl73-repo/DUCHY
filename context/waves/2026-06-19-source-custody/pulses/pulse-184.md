# Pulse 184: Bavaria Hanover Saxony Electorate Source Custody

## Intent

Add reviewed text support for three existing active Holy Roman Empire
electorate parentage facts.

## Changes

- Add `src-wikipedia-electorate-bavaria`.
- Attach it to Electorate of Bavaria -> Holy Roman Empire for `1623..1805`.
- Add `src-wikipedia-electorate-hanover`.
- Attach it to Electorate of Hanover -> Holy Roman Empire for `1692..1806`.
- Add `src-wikipedia-electorate-saxony`.
- Attach it to Electorate of Saxony -> Holy Roman Empire for `1356..1806`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import predecessor duchies, personal unions, imperial offices,
occupations, acquisitions, or kingdom transitions.

## Current State

- reviewed sources: 454
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
