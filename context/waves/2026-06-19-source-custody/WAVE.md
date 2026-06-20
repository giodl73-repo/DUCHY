# Wave: Source Custody Intake

## Goal

Move DUCHY from seed-only fixtures toward source-aware historical data without
importing real title facts before custody review.

## Thesis

DUCHY can only answer real European historical questions responsibly if every
source-backed claim carries source metadata, allowed-use posture, confidence,
and review evidence. The next safe implementation layer is metadata-only source
records and validation.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Metadata-only source records | complete | Add source records, allowed-use posture, review decisions, and validation. |
| 02 | Metadata source file format | complete | Define and parse a fixture file shape for source records without importing historical facts. |
| 03 | Source-backed fact gate | complete | Validate fact records against accepted source records and confidence labels. |
| 04 | First real source-backed facts | complete | Import only reviewed name, rank, and existence facts for Wikidata Q158445. |
| 05 | Source-backed title materialization | complete | Convert reviewed fact sets into a real `Title` record without parentage/control claims. |
| 06 | Source-backed title query | complete | Answer a source-backed title-path query for the first real title. |
| 07 | Fact fixture import | complete | Parse first real source-backed facts from `fixtures/first-real.facts`. |
| 08 | Contested-history review packet | complete | Represent conflicting source-backed claims without overwriting them. |
| 09 | Second reviewed source import | complete | Add Q20135 Grand Duchy of Hesse name/rank/existence facts. |
| 10 | Source-backed parentage review | complete | Materialize reviewed parentage facts while keeping real fixture hierarchy-free. |
| 11 | Reviewed source fixture import | complete | Parse reviewed real source records from `fixtures/first-real.sources`. |
| 12 | First real parentage source import | complete | Import Q20135 -> Q43287 and Q158445 -> Q43287 for 1871-1918 after source review. |
| 13 | Fixture-canonical import path | complete | Remove reviewed historical data literals from Rust; make fixtures the import source of truth. |
| 14 | Reviewed Prussia import packet | complete | Import Q27306 title facts and Q27306 -> Q43287 for 1871-1918 after source review. |
| 15 | Reviewed Saxony import packet | complete | Import Q153015 title facts and Q153015 -> Q43287 for 1871-1918 after source review. |
| 16 | Batch import staging gate | complete | Add CLI status/dry-run promotion and duplicate/conflict validation for staged batches. |
| 17 | Apply-mode promotion | complete | Let reviewed staging batches rewrite accepted fixture files after full validation. |
| 18 | Promotion review reports | complete | Emit promotion reports listing candidate titles, parentage, fact IDs, and merged counts. |
| 19 | Candidate manifest queue | complete | Parse and validate staging manifests before fact extraction. |
| 20 | Source stub generation | complete | Generate blocked source stubs from reviewed manifest candidates. |
| 21 | Rejected candidate audit | complete | Generate rejected-candidate reports before queue cleanup. |
| 22 | Active manifest cleanup | complete | Generate pending/reviewed-only manifests after audit/archive. |
| 23 | Archive manifest cleanup | complete | Generate promoted/rejected-only manifests for queue audit. |
| 24 | Manifest sharding | complete | Split large candidate manifests into fixed-size review batches. |
| 25 | Shard index | complete | Write per-shard status counts for large review queues. |
| 26 | Manifest review report | complete | List every candidate grouped by queue status for inspection. |
| 27 | Duplicate URL report | complete | Report repeated candidate source URLs before source review. |
| 28 | Manifest TSV export | complete | Write fixed-column candidate queue exports for batch tooling. |
| 29 | Manifest TSV import | complete | Convert fixed-column TSV candidate queues into validated manifests. |
| 30 | German bridge import | complete | Import reviewed 1815-1866, 1867-1870, and 1871-1918 German parentage spans. |
| 31 | Baden and Hanover import | complete | Import reviewed Baden and Hanover title facts and parentage spans. |
| 32 | Oldenburg and Brunswick import | complete | Import reviewed Oldenburg and Brunswick title facts and parentage spans. |
| 33 | Mass title source scale-up | complete | Import reviewed title facts until the accepted source catalog reaches 50 sources. |

## Success Criteria

- Source records validate required metadata.
- Metadata-only records cannot masquerade as accepted historical facts.
- Review decisions reference existing source records.
- Validation commands pass.
- Real title facts enter only through reviewed source records and fact gate validation.
- Reviewed historical IDs, names, spans, and relations live in fixtures rather
  than Rust literals.
- Candidate batches validate in staging before accepted fixture promotion.
