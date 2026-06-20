# Gorizia and Gradisca Austrian Parentage Review

## Candidate

- child: Gorizia and Gradisca (Q692946), accepted as `title-q692946`
- parents:
  - Austrian Empire (Q131964), accepted as `title-q131964`
  - Austria-Hungary (Q28513), accepted as `title-q28513`
- relation claims reviewed: `P131` for Austrian Empire, `P17` for Austria-Hungary
- relation screen: `data/staging/gorizia-gradisca-austrian-parentage-01-relations.tsv`

## Decision

After correcting Q692946 from `Empire` to `County`, promote two parentage facts:

- Gorizia and Gradisca -> Austrian Empire for 1804..1866
- Gorizia and Gradisca -> Austria-Hungary for 1867..1918

The spans use the existing Austrian parentage packet policy: where a child has
both Austrian Empire and Austria-Hungary parentage, the Austrian Empire relation
ends in 1866 to avoid an overlapping yearly answer at 1867.

## Boundary

This packet imports only the reviewed parentage relations. It does not import
borders, crown-land administrative status, holders, de facto control,
successor/predecessor relations, or post-1918 Italian/Yugoslav transition
details.
