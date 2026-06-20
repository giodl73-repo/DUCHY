# Pulse 58: Accepted Relation Bridges Parentage Packet

## Intent

Expand real source-backed hierarchy coverage without adding new source records,
using only already accepted Wikidata records and bounded structured relations.

## Changes

- Add a reviewed parentage-only staging packet for accepted `P17` and `P361`
  relation bridges.
- Promote 4 parentage facts:
  - Kingdom of Hungary -> Austrian Empire, 1804..1867.
  - Kingdom of Portugal -> Iberian Union, 1580..1640.
  - Abaúj county -> Kingdom of Hungary, 1201..1881.
  - County of Aragon -> Kingdom of Pamplona, 824..1035.
- Screen out Duchy of Mantua -> Kingdom of Italy because it conflicts with an
  already accepted Holy Roman Empire parentage span under the current model.
- Screen out March of Tuscany -> Kingdom of Italy because it already has an
  accepted Holy Roman Empire parentage span and overlapping parentage is not yet
  modeled.
- Screen out Duchy of Savoy -> Kingdom of Sardinia for the same overlapping
  parentage reason.
- Keep accepted source count unchanged at 334 while increasing accepted facts
  from 1109 to 1113 and parentage facts from 140 to 144.

## Review Boundary

This pulse imports parentage facts only. It does not import new source records,
borders, holders, de facto control, successor/predecessor relations, dynastic
continuity, raw Wikidata cache data, or overlapping parentage claims.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-bridges-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-bridges-parentage-01.sources data/staging/relation-bridges-parentage-01.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-bridges-parentage-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-bridges-parentage-01.sources data/staging/relation-bridges-parentage-01.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
