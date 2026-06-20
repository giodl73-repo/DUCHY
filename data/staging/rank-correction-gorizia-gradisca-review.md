# Gorizia and Gradisca Rank Correction Review

## Candidate

- title: Gorizia and Gradisca (Q692946)
- accepted source: `src-wikidata-q692946`
- accepted title: `title-q692946`
- previous rank fact: `Empire`
- corrected rank fact: `County`

## Decision

Correct the accepted rank fact from `Empire` to `County`.

The original candidate metadata described Gorizia and Gradisca as a crown land
of the empire of Austria and classified it with `entity_class: empire`. The live
structured source now reviewed for this packet identifies county/princely-county
classes (`P31` includes `Q353344`) and parent-style links to Austrian Empire and
Austria-Hungary. Keeping the rank as `Empire` makes those reviewed parentage
relations invalid under DUCHY rank validation.

## Boundary

This correction changes only the accepted rank fact for Q692946. It does not
change the title span, source record, borders, holders, or infer parentage by
itself.
