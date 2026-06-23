# Pulse 199: Brabant Bremen-Verden Carniola Source Custody

## Intent

Add reviewed text support for existing Holy Roman Empire parentage facts.

## Changes

- Add `src-wikipedia-duchy-brabant`.
- Attach it to Duchy of Brabant -> Holy Roman Empire for `1183..1795`.
- Add `src-wikipedia-bremen-verden`.
- Attach it to Duchy of Bremen and Verden -> Holy Roman Empire for
  `1648..1806`.
- Add `src-wikipedia-duchy-carniola`.
- Attach it to Duchy of Carniola -> Holy Roman Empire for `1364..1803`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import predecessor landgraviates/marches/prince-bishoprics,
Burgundian or Habsburg Netherlands, Swedish/Hanoverian personal unions,
French/Westphalian administration, Inner Austria, Austrian Circle, Illyrian
Provinces, Austrian Empire, Austria-Hungary, or successor-state claims.

## Current State

- reviewed sources: 489
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
