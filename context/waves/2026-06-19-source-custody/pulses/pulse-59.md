# Pulse 59: Second Accepted Relation Bridges Parentage Packet

## Intent

Continue expanding real source-backed hierarchy coverage from accepted title
records without opening new source records or accepting overlapping parentage
claims.

## Changes

- Add a reviewed parentage-only staging packet for a second accepted `P17`
  relation bridge set.
- Promote 11 parentage facts:
  - Duklja -> Byzantine Empire, 854..1252.
  - East Francia -> Carolingian Empire, 843..887.
  - Prince-Bishopric of Warmia -> Crown of the Kingdom of Poland, 1386..1772.
  - Duchy of Swabia -> East Francia, 917..961.
  - Duchy of Alsace -> Francia, 700..800.
  - County of Ribagorza -> Kingdom of Aragon, 1035..1598.
  - County of Portugal -> Kingdom of Asturias, 868..909.
  - County of Portugal -> Kingdom of Galicia, 910..1139.
  - Duchy of Nysa -> Kingdom of Bohemia, 1290..1850.
  - County of Mark -> Kingdom of Prussia, 1701..1806.
  - Cetatea-Albă County -> Kingdom of Romania, 1925..1944.
- Leave the overlapping County of Portugal -> Kingdom of Leon relation for a
  later contested or overlapping-parentage review.
- Keep accepted source count unchanged at 334 while increasing accepted facts
  from 1113 to 1124 and parentage facts from 144 to 155.

## Review Boundary

This pulse imports parentage facts only. It does not import new source records,
borders, holders, de facto control, successor/predecessor relations, dynastic
continuity, raw Wikidata cache data, or overlapping parentage claims.

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/relation-bridges-parentage-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-bridges-parentage-02.sources data/staging/relation-bridges-parentage-02.facts`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/relation-bridges-parentage-02-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/relation-bridges-parentage-02.sources data/staging/relation-bridges-parentage-02.facts`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
