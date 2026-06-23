# Pulse 194: Nassau Saxe-Lauenburg Source Custody

## Intent

Add reviewed text support for existing Duchy of Nassau and Saxe-Lauenburg
parentage facts.

## Changes

- Add `src-wikipedia-duchy-nassau`.
- Attach it to Duchy of Nassau -> German Confederation for `1815..1866`.
- Add `src-wikipedia-saxe-lauenburg`.
- Attach it to Saxe-Lauenburg -> Holy Roman Empire for `1296..1806`.
- Attach it to Saxe-Lauenburg -> German Confederation for `1815..1866`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import Confederation of the Rhine, 1848-1849 German Empire episode,
interim independence, annexation, personal unions, North German Confederation,
German Empire, Prussian district conversion, or later honorific title claims.

## Current State

- reviewed sources: 474
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
