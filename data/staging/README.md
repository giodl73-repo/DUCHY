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
cargo run --bin duchy-import -- shard-manifest data/staging/example.manifest data/staging/shards 2
```

Generated source stubs are blocked by default and must be reviewed before they
can support fact promotion.
Rejected reports preserve rejected candidates and notes for audit.
Active manifests keep only pending and reviewed candidates after promoted or
rejected rows have been archived.
Archive manifests keep promoted and rejected candidates as parseable audit
records outside the working queue.
Manifest shards split larger queues into fixed-size, parseable review batches.

Example dry run:

```powershell
cargo run --bin duchy-promote -- --dry-run fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
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
