# DUCHY Accepted Relation Bridges Parentage Packet 02 Review Notes

reviewed_parentage_facts: 11
accepted_source_basis: existing `fixtures/first-real.sources`
claim_basis: Wikidata structured relation scan across already accepted source records, limited to accepted child and parent titles connected by `P17`.

## Boundary

This packet imports parentage facts only. It does not import new source records,
borders, holders, de facto control, successor/predecessor relations, dynastic
continuity, raw Wikidata cache data, or overlapping parentage claims.

## Parentage Facts

| Child | Parent | Span | Relation |
|---|---|---:|---|
| Duklja | Byzantine Empire | 854..1252 | `P17` |
| East Francia | Carolingian Empire | 843..887 | `P17` |
| Prince-Bishopric of Warmia | Crown of the Kingdom of Poland | 1386..1772 | `P17` |
| Duchy of Swabia | East Francia | 917..961 | `P17` |
| Duchy of Alsace | Francia | 700..800 | `P17` |
| County of Ribagorza | Kingdom of Aragon | 1035..1598 | `P17` |
| County of Portugal | Kingdom of Asturias | 868..909 | `P17` |
| County of Portugal | Kingdom of Galicia | 910..1139 | `P17` |
| Duchy of Nysa | Kingdom of Bohemia | 1290..1850 | `P17` |
| County of Mark | Kingdom of Prussia | 1701..1806 | `P17` |
| Cetatea-Albă County | Kingdom of Romania | 1925..1944 | `P17` |

## Span Policy

Each span is the intersection of the accepted child title existence span and the
accepted parent title existence span, trimmed by one year where an adjacent
accepted parentage starts on the same boundary year under current query
semantics. The packet excludes candidates whose child already has overlapping
accepted parentage in that span. County of Portugal had two overlapping post-910
alternatives in the relation scan; this packet keeps Kingdom of Galicia and
leaves Kingdom of Leon for a later contested or overlapping-parentage review.
