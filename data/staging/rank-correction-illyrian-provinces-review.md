# DUCHY Rank Correction: Illyrian Provinces

title_id: title-q699923
source_id: src-wikidata-q699923
previous_rank: Empire
corrected_rank: Province

## Review Basis

Wikidata Q699923 identifies Illyrian Provinces as a province-level entity and
links it to Q71084 First French Empire through structured `P131` and `P361`
claims. Treating it as an empire makes the parentage relation invalid and
misstates the title rank.

## Decision

Correct `fact-q699923-rank` to `Province`. The correction is limited to rank
semantics and does not alter the accepted title existence span.
