# Pulse 06: Source-Backed Title Query

## Intent

Make the first real imported title queryable through DUCHY's existing answer
envelope instead of leaving it as a standalone materialized record.

## Implementation

- Add `TitleTimeline::title_path_query_for_title_in_year`.
- Add source-class-specific title-path trace codes.
- Add `source_backed_timeline_from_facts`.
- Add `first_real_timeline`.
- Exercise Q158445 through a `SourceClass::SourceBacked` title-path query.

## First Query

- Subject: `title-q158445`
- Year: 1815
- Status: `Answered`
- Source class: `SourceBacked`
- Trace code: `source_backed_title_path`
- Answer path: Grand Duchy of Mecklenburg-Schwerin

## Boundaries

The answer is intentionally a one-step title path. DUCHY has not yet imported a
reviewed parentage claim, so no kingdom, empire, holder, area, or map assertion
is attached to the real title.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
