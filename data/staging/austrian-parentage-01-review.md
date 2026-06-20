# DUCHY Austrian Parentage Packet 01 Review Notes

reviewed_parentage_facts: 15
accepted_source_basis: existing `fixtures/first-real.sources`
claim_basis: Wikidata structured parent/membership-style claims (`P131`, `P361`, `P463`, or `P17`) linking an accepted child title to an accepted parent title.

## Parent Titles

- Q131964 | Austrian Empire | 1804..1867
- Q28513 | Austria-Hungary | 1867..1918

## Boundary

This packet imports parentage facts only. It does not import borders, holders,
dynastic continuity, de facto control, successor/predecessor relations, or new
source records.

## Span Policy

Each parentage span is the intersection of the accepted child title existence
span and the accepted parent title existence span, further limited to the
structured Wikidata link observed during review. Where a child has both
Austrian Empire and Austria-Hungary parentage, the Austrian Empire relation ends
in 1866 so yearly parentage queries do not overlap the 1867 Austria-Hungary
relation.
