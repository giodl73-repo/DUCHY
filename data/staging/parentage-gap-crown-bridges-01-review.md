# Parentage Gap Crown Bridges 01 Review

## Candidate Set

This packet imports parentage-only facts for accepted kingdom titles whose
accepted parent crowns are already materialized:

- Crown of Aragon (Q204920)
- Crown of Castile (Q217196)

## Decision

Promote ten bounded parentage facts from the gap queue. Every child and parent
source record is already accepted, every child and parent title is already
materialized, and every parent has rank `Crown`, which is higher than the child
rank `Kingdom`.

The relation evidence is captured in
`data/staging/parentage-gap-crown-bridges-01-relations.tsv`.

## Boundary

This packet does not add new sources or title facts. It imports only the listed
reviewed crown relation bridges and uses the overlap of accepted child and
parent title spans for each parentage fact.
