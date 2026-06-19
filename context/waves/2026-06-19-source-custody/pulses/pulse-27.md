# Pulse 27: Duplicate URL Report

## Intent

Catch repeated candidate source pointers before source-custody review spends
time on accidental duplicate intake rows.

## Changes

- Add `duchy-import duplicate-url-report <manifest> <output.md>`.
- Validate the source manifest before report generation.
- Group candidate rows by `source_url`.
- Emit a Markdown report with zero-duplicate evidence or duplicate URL groups.

## Review Boundary

Duplicate URL reports are hygiene artifacts. They do not reject candidates,
promote source records, accept facts, or change reviewed fixture history.

## Validation

- `cargo test --quiet`
- `cargo run --bin duchy-import -- duplicate-url-report data/staging/example.manifest <temp report>`
- inspect generated report
- `cargo fmt --check`
- `git diff --check`
