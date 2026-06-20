# DUCHY Accepted Relation Bridges Parentage Packet 01 Review Notes

reviewed_parentage_facts: 4
accepted_source_basis: existing `fixtures/first-real.sources`
claim_basis: Wikidata structured relation scan across already accepted source records, limited to accepted child and parent titles connected by `P17` or `P361`.

## Boundary

This packet imports parentage facts only. It does not import new source records,
borders, holders, de facto control, successor/predecessor relations, dynastic
continuity, or raw Wikidata cache data.

## Parentage Facts

| Child | Parent | Span | Relation |
|---|---|---:|---|
| Kingdom of Hungary | Austrian Empire | 1804..1867 | `P17` |
| Kingdom of Portugal | Iberian Union | 1580..1640 | `P17` |
| Abaúj county | Kingdom of Hungary | 1201..1881 | `P17` |
| County of Aragon | Kingdom of Pamplona | 824..1035 | `P17` |

## Span Policy

Each span is the intersection of the accepted child title existence span, the
accepted parent title existence span, and the structured relation reviewed for
this packet. Candidates with likely simultaneous or conflicting parentage were
left out for later contested or overlapping-parentage modeling. Duchy of Mantua
was screened out because it overlaps an already accepted Holy Roman Empire
parentage span. March of Tuscany was also screened out because it already has an
accepted Holy Roman Empire parentage span and the current query model cannot
represent simultaneous parentage without ambiguity.
Duchy of Savoy was screened out for the same reason; it already has an accepted
Holy Roman Empire parentage span that overlaps the candidate Sardinia relation.
