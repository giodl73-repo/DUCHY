# Pulse 192: Coburg-Gotha Schwarzburg-Rudolstadt Source Custody

## Intent

Add reviewed text support for existing German Confederation parentage facts.

## Changes

- Add `src-wikipedia-saxe-coburg-gotha`.
- Attach it to Saxe-Coburg and Gotha -> German Confederation for `1826..1866`.
- Add `src-wikipedia-schwarzburg-rudolstadt`.
- Attach it to Schwarzburg-Rudolstadt -> German Confederation for
  `1815..1866`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import partitions, personal union mechanics, German Empire status,
military treaties, princely elevation, ruler lists, Weimar Republic status, or
successor Free State claims.

## Current State

- reviewed sources: 469
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
