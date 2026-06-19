# Pulse 05: Source-Backed Title Materialization

## Intent

Turn reviewed source-backed fact sets into usable DUCHY title records while
keeping historical scope narrow and auditable.

## Implementation

- Add `ClaimKind::Rank`.
- Add `source_backed_titles_from_facts`.
- Add `first_real_titles`.
- Require each materialized title to have reviewed name, rank, and existence
  facts.
- Leave `de_jure_parent` empty until parentage facts pass a separate source
  review.

## First Materialized Title

- ID: `title-q158445`
- Name: Grand Duchy of Mecklenburg-Schwerin
- Rank: Duchy
- Existence span: 1815 through 1918
- Source: `src-wikidata-q158445`

## Explicit Non-Imports

- No territorial geometry.
- No parent kingdom or empire relation.
- No county membership.
- No holder genealogy.
- No control timeline.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
