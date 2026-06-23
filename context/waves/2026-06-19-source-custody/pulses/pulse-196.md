# Pulse 196: Anhalt-Bernburg Hohenzollern Source Custody

## Intent

Add reviewed text support for existing Anhalt-Bernburg,
Hohenzollern-Hechingen, and Hohenzollern-Sigmaringen parentage facts.

## Changes

- Add `src-wikipedia-anhalt-bernburg`.
- Attach it to Anhalt-Bernburg -> Holy Roman Empire for `1252..1806`.
- Attach it to Anhalt-Bernburg -> German Confederation for `1815..1863`.
- Add `src-wikipedia-hohenzollern-hechingen`.
- Attach it to Hohenzollern-Hechingen -> Holy Roman Empire for `1576..1806`.
- Attach it to Hohenzollern-Hechingen -> German Confederation for
  `1815..1850`.
- Add `src-wikipedia-hohenzollern-sigmaringen`.
- Attach it to Hohenzollern-Sigmaringen -> Holy Roman Empire for
  `1576..1806`.
- Attach it to Hohenzollern-Sigmaringen -> German Confederation for
  `1815..1850`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Anhalt partitions, Confederation of the Rhine, 1848
constitutional events, sale/abdication, Prussian incorporation, Province of
Hohenzollern, territorial lists, or dynastic succession claims.

## Current State

- reviewed sources: 480
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
