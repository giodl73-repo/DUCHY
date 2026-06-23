# Pulse 191: Saxe-Weimar Schwarzburg-Sondershausen Source Custody

## Intent

Add reviewed text support for existing Saxe-Weimar-Eisenach and
Schwarzburg-Sondershausen parentage facts.

## Changes

- Add `src-wikipedia-saxe-weimar-eisenach`.
- Attach it to Saxe-Weimar-Eisenach -> German Confederation for `1815..1866`.
- Attach it to Saxe-Weimar-Eisenach -> North German Confederation for
  `1867..1870`.
- Add `src-wikipedia-schwarzburg-sondershausen`.
- Attach it to Schwarzburg-Sondershausen -> Holy Roman Empire for `1599..1806`.
- Attach it to Schwarzburg-Sondershausen -> German Confederation for
  `1815..1866`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import personal unions, princely elevation, ruler lists, currency,
Confederation of the Rhine, German Empire, Weimar Republic, or Thuringia
successor claims.

## Current State

- reviewed sources: 467
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
