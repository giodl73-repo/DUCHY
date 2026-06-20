# Pulse 46: CK3 Europe County Discovery Queue

## Intent

Use the CK3 wiki county list to seed a larger Europe-focused discovery queue
for future source research without treating game data as historical evidence.

## Changes

- Parse the saved CK3 wiki county MHTML into a tabular staging export.
- Filter 1,069 county rows to Europe-facing CK3 empire buckets and select 500
  pending candidates.
- Generate manifest, report, duplicate URL report, and ten 50-row review shards.

## Review Boundary

This pulse does not promote historical facts. CK3 county rows are search leads
only. Every candidate must resolve to an independent reviewed historical source
before any title identity, parentage, holder, control, or territory claim can
enter accepted fixtures.

## Validation

- `cargo run --quiet --bin duchy-import -- manifest data/staging/ck3-counties-500.manifest`
- `cargo run --quiet --bin duchy-import -- manifest-report data/staging/ck3-counties-500.manifest data/staging/ck3-counties-500-report.md`
- `cargo run --quiet --bin duchy-import -- duplicate-url-report data/staging/ck3-counties-500.manifest data/staging/ck3-counties-500-duplicate-urls.md`
- `cargo run --quiet --bin duchy-import -- shard-manifest data/staging/ck3-counties-500.manifest data/staging/ck3-counties-500-shards 50`
