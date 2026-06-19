# DUCHY Source-Custody Package

## Scope

This package gates any move from fictional seed fixtures to real European title,
territorial, lineage, or historical boundary data.

It does not import data. It defines what must be true before DUCHY can import or
publish source-backed rows.

## Package Files

| File | Purpose |
|---|---|
| `SOURCE_INVENTORY.md` | Candidate source classes and rights posture. |
| `IMPORT_POLICY.md` | Allowed and forbidden import behavior. |
| `CONFIDENCE_MODEL.md` | Confidence, uncertainty, conflict, and citation rules. |
| `SOURCE_SCHEMA.md` | Required source metadata fields for future fixtures. |
| `REVIEW_GATE.md` | Source Custody review checklist and acceptance gate. |

## Current Decision

DUCHY may keep using seed fixtures for model and query validation. DUCHY may not
claim real European historical coverage until a source-backed fixture passes the
review gate in this package.
