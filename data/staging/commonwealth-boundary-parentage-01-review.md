# Commonwealth Boundary Parentage Review

## Candidate

- child: Kingdom of Poland (Q8890160), accepted as `title-q8890160`
- parent: Polish-Lithuanian Commonwealth (Q172107), accepted as `title-q172107`
- relation claim reviewed: `P131`
- relation screen: `data/staging/commonwealth-boundary-parentage-01-relations.tsv`

## Decision

Promote one boundary-year parentage fact for 1569..1569.

This is a year-granular transition record: the accepted child span ends in 1569
and the accepted parent span begins in 1569. DUCHY cannot represent sub-year
precision, so the overlap is a single year. The packet is kept separate from
the multi-year Commonwealth children packet to make the boundary semantics
visible in review.

## Boundary

This packet imports only the reviewed 1569 boundary overlap. It does not infer a
longer Kingdom of Poland -> Commonwealth parentage span.
