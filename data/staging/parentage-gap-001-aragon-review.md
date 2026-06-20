# Parentage Gap 001 Aragon Review

## Candidate

- child: County of Barcelona (Q1233672), accepted as `title-q1233672`
- parent: Crown of Aragon (Q204920), staged as `title-q204920`
- relation claim reviewed: `P361` from County of Barcelona to Crown of Aragon
- relation screen: `data/staging/parentage-gap-shards/batch-001-wikidata-relations.tsv`

## Decision

Promote Crown of Aragon as a composite `Crown` rank title and import one
bounded parentage fact for County of Barcelona.

The parent was previously scope-deferred in the broad 500-source queue because
the old rank vocabulary had no composite crown rank. This review reopens it for
a narrower accepted use: label, rank, inception/dissolution, and the Barcelona
relation bridge only.

## Spans

- Crown of Aragon title span: 1162..1715
- County of Barcelona accepted title span: 801..1164
- parentage span: 1162..1164, the year-level overlap of the accepted child span
  and reviewed parent span

## Boundary

This packet does not infer all Crown of Aragon sub-realms. It imports only the
reviewed County of Barcelona bridge from shard 001.
