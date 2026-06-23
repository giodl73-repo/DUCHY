# Pulse 200: Austria Bavaria-Munich Luxembourg Source Custody

## Intent

Add reviewed text support for existing Holy Roman Empire parentage facts.

## Changes

- Add `src-wikipedia-archduchy-austria`.
- Attach it to Archduchy of Austria -> Holy Roman Empire for `1358..1803`.
- Add `src-wikipedia-bavaria-munich`.
- Attach it to Bavaria-Munich -> Holy Roman Empire for `1392..1505`.
- Add `src-wikipedia-duchy-luxembourg`.
- Attach it to Duchy of Luxembourg -> Holy Roman Empire for `1353..1795`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Habsburg hereditary lands, Austrian Empire, Austria-Hungary,
imperial office, Bavaria-Landshut, Bavaria-Ingolstadt, Bavaria-Straubing,
Bavaria-Dachau, reunified Bavaria, Burgundian Netherlands, Habsburg
Netherlands, Spanish Netherlands, Austrian Netherlands, French occupation, or
Grand Duchy of Luxembourg successor claims.

## Current State

- reviewed sources: 492
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
