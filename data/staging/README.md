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

Example dry run:

```powershell
cargo run --bin duchy-promote -- --dry-run fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
```

Apply a reviewed batch only after dry-run and review:

```powershell
cargo run --bin duchy-promote -- --apply fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
```

`--apply` rewrites the accepted fixture files with the validated merged content.
Use it only for reviewed historical batches, then commit the accepted fixture
diff separately from staging cleanup.
