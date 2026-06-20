# Parentage Gap 001 Castile Review

## Candidate

- child: Kingdom of Murcia (Q1164500), accepted as `title-q1164500`
- parent: Crown of Castile (Q217196), staged as `title-q217196`
- relation claims reviewed: `P17` and `P361` from Kingdom of Murcia to Crown of Castile
- relation screen: `data/staging/parentage-gap-shards/batch-001-wikidata-relations.tsv`

## Decision

Promote Crown of Castile as a composite `Crown` rank title and import one
bounded parentage fact for Kingdom of Murcia.

The parent was previously scope-deferred in the broad 500-source queue because
the old rank vocabulary had no composite crown rank. The parentage gap review
reopens it for a narrower accepted use: label, rank, inception/dissolution, and
the Murcia relation bridge only.

## Spans

- Crown of Castile title span: 1230..1715
- Kingdom of Murcia accepted title span: 1258..1833
- parentage span: 1258..1715, the overlap of the accepted child span and the
  reviewed parent span

## Boundary

This packet does not infer all Crown of Castile sub-realms. It imports only the
reviewed Kingdom of Murcia bridge from shard 001.
