# DUCHY Staging Area

This directory is for candidate import batches before source-custody review.

Staging files are not accepted history. A batch is eligible for promotion only
after:

- source metadata is present,
- every source has a review decision that allows fact claims,
- fact IDs are unique,
- accepted facts do not contradict existing accepted facts,
- parentage/control spans fit materialized titles after merge, and
- `duchy-promote --dry-run` passes.

Manifest queues track candidate sources before fact extraction:

```powershell
cargo run --bin duchy-import -- manifest data/staging/example.manifest
cargo run --bin duchy-import -- source-stubs data/staging/example.manifest data/staging/generated.sources
cargo run --bin duchy-import -- rejected-report data/staging/example.manifest data/staging/rejected.md
cargo run --bin duchy-import -- active-manifest data/staging/example.manifest data/staging/active.manifest
cargo run --bin duchy-import -- archive-manifest data/staging/example.manifest data/staging/archive.manifest
cargo run --bin duchy-import -- manifest-report data/staging/example.manifest data/staging/manifest-report.md
cargo run --bin duchy-import -- duplicate-url-report data/staging/example.manifest data/staging/duplicate-urls.md
cargo run --bin duchy-import -- manifest-tsv data/staging/example.manifest data/staging/manifest.tsv
cargo run --bin duchy-import -- manifest-from-tsv data/staging/manifest.tsv data/staging/from-tsv.manifest
cargo run --bin duchy-import -- shard-manifest data/staging/example.manifest data/staging/shards 2
```

The 500-source candidate queue uses the same tooling at larger review-batch
size:

```powershell
cargo run --bin duchy-import -- manifest data/staging/candidates-500.manifest
cargo run --bin duchy-import -- manifest-report data/staging/candidates-500.manifest data/staging/candidates-500-report.md
cargo run --bin duchy-import -- duplicate-url-report data/staging/candidates-500.manifest data/staging/candidates-500-duplicate-urls.md
cargo run --bin duchy-import -- rejected-report data/staging/candidates-500.manifest data/staging/candidates-500-rejected.md
cargo run --bin duchy-import -- active-manifest data/staging/candidates-500.manifest data/staging/candidates-500-active.manifest
cargo run --bin duchy-import -- archive-manifest data/staging/candidates-500.manifest data/staging/candidates-500-archive.manifest
cargo run --bin duchy-import -- shard-manifest data/staging/candidates-500.manifest data/staging/candidates-500-shards 50
```

The CK3 county discovery queue uses the saved CK3 wiki county list as a search
driver for 500 Europe-bucket county candidates. CK3 rows are pending discovery
records only; they cannot support accepted historical facts until each row is
resolved to an independent reviewed historical source:

```powershell
cargo run --bin duchy-import -- manifest data/staging/ck3-counties-500.manifest
cargo run --bin duchy-import -- manifest-report data/staging/ck3-counties-500.manifest data/staging/ck3-counties-500-report.md
cargo run --bin duchy-import -- duplicate-url-report data/staging/ck3-counties-500.manifest data/staging/ck3-counties-500-duplicate-urls.md
cargo run --bin duchy-import -- shard-manifest data/staging/ck3-counties-500.manifest data/staging/ck3-counties-500-shards 50
```

The first CK3 shard has an external source-resolution lead packet:

```text
data/staging/ck3-counties-batch-001-wikidata-research.tsv
data/staging/ck3-counties-batch-001-wikidata-research.md
```

These files record search leads only. Exact label matches still require manual
identity review before any source record or fact packet is created.

Generated source stubs are blocked by default and must be reviewed before they
can support fact promotion.
Rejected reports preserve rejected candidates and notes for audit.
Active manifests keep only pending and reviewed candidates after promoted or
rejected rows have been archived.
Archive manifests keep promoted and rejected candidates as parseable audit
records outside the working queue.
Manifest reports list every candidate grouped by queue status for reviewer
inspection.
Duplicate URL reports flag repeated source pointers before source-custody
review.
Manifest TSV exports provide a machine-readable queue summary for batch
tooling.
Manifest TSV imports convert fixed-column batch files back into validated
candidate manifests.
Manifest shards split larger queues into fixed-size, parseable review batches
and write `INDEX.md` with per-shard status counts.

Reviewed and promoted manifest rows must carry scale metadata before fact
extraction:

- `review_batch_id`: stable review batch identifier.
- `import_scope`: one of `title_identity_only`, `parentage_ready`,
  `territory_ready`, `holder_ready`, or `contested_review`.
- `rank_basis`: one of `literal`, `normalized`, `approximate`, or
  `unsupported`.
- `entity_class`: one of `county`, `duchy`, `kingdom`, `principality`,
  `free_city`, `theocratic_state`, `confederation`, `empire`,
  `administrative_region`, or `other`.
- `source_claims_used`: comma-separated structured claims accepted for import.
- `confidence_detail`: one of `wikidata_structured_single`,
  `wikidata_plus_text_crosscheck`, `multi_source_agreement`, `date_conflict`,
  or `unsupported`.
- `parentage_status`: one of `none_reviewed`, `candidate_available`,
  `accepted_partial`, `accepted_full`, or `contested`.
- `query_readiness`: one of `existence_only`, `title_path`, `transfer`,
  `lineage_event`, or `unsupported`.

Rejected rows must carry `exclusion_reason`: one of `unsupported_rank`,
`non_title_polity`, `ambiguous_entity`, `date_conflict`,
`successor_predecessor_issue`, `rights_blocked`, `quality_blocked`, or
`scope_deferred`.

Example dry run:

```powershell
cargo run --bin duchy-promote -- --dry-run fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
```

Reviewed real packets can also be staged before fixture promotion:

```powershell
cargo run --bin duchy-promote -- --dry-run --report data/staging/german-bridge-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-bridge.sources data/staging/german-bridge.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/baden-hanover-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/baden-hanover.sources data/staging/baden-hanover.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/oldenburg-brunswick-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/oldenburg-brunswick.sources data/staging/oldenburg-brunswick.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/mass-title-50-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/mass-title-50.sources data/staging/mass-title-50.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/batch-001-title-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/batch-001-title.sources data/staging/batch-001-title.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/batch-002-title-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/batch-002-title.sources data/staging/batch-002-title.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/remaining-title-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/remaining-title.sources data/staging/remaining-title.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/german-parentage-02-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-parentage-02.sources data/staging/german-parentage-02.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/austrian-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/austrian-parentage-01.sources data/staging/austrian-parentage-01.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/hre-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/hre-parentage-01.sources data/staging/hre-parentage-01.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/france-parentage-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/france-parentage-01.sources data/staging/france-parentage-01.facts
cargo run --bin duchy-promote -- --dry-run --report data/staging/german-parentage-03-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/german-parentage-03.sources data/staging/german-parentage-03.facts
```

Add `--report path/to/report.md` after the mode to produce a review artifact:

```powershell
cargo run --bin duchy-promote -- --dry-run --report data/staging/example-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
```

Apply a reviewed batch only after dry-run and review:

```powershell
cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
```

`--apply` rewrites the accepted fixture files with the validated merged content.
Use it only for reviewed historical batches, then commit the accepted fixture
diff separately from staging cleanup.
