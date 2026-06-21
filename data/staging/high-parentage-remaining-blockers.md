# Remaining High-Priority Parentage Blockers

Reviewed on 2026-06-21 after high-priority parentage batches 01-10 and the
machine-readable blocker queue update.

## Summary

- Active high-priority parentage gaps remaining: 0.
- Reviewed blocker rows remaining: 2.
- These rows are no longer active parentage-import work because the currently
  accepted fixture catalog lacks a reviewed higher-rank parent that overlaps the
  title span, or the source-backed relation is not a de jure parentage claim
  under the current model.

## Blockers

| Title | Span | Blocker |
|---|---:|---|
| Principality of Albania (Q187035) | 1914-1925 | The accepted Italian protectorate/Italian Empire Albania parentage context begins outside this title's accepted span; World War I occupation/protectorate sources are partial and do not provide a whole-title parentage claim under an accepted higher-rank parent. |
| Principality of Montenegro (Q779011) | 1852-1910 | Sources support de facto independence and 1878 recognition rather than a clean Ottoman parentage span; importing Ottoman parentage would overstate the relation without contested/de facto status modeling. |

## Next Work

- Add contested/de facto/de jure relation semantics before importing Montenegro
  or partial Albania occupation/protectorate claims.
- Keep these rows visible as `blocked_parentage_review` in
  `data/staging/parentage-gap-blockers.tsv` and the generated gap queue until
  those modeling gaps are resolved.
