# Pulse 212 - Relation Model Work Package

Date: 2026-06-23

## Scope

- Convert the repeated rank-skip review blockers into a formal VTRACE work
  package.
- Define which relation classes should be modeled outside strict parentage.
- Stop treating reviewed shared-parent bridge false positives as direct import
  candidates.

## Results

- Reviewed high-priority rank-skip keys analyzed: 40.
- Importable replacement facts from those keys: 0.
- Proposed work package added: `WP-007`.
- Proposed requirement added: `REQ-015`.
- Proposed interface added: `IF-015`.

## Decision

The next major milestone should be relation-model implementation, not more
manual rank-skip imports. DUCHY needs typed relation facts for imperial-state,
confederation-member, federal-state-member, composite-crown, split-fief/control,
vassalage/suzerainty, subdivision/appanage, and rank-transition semantics.

## Validation

```powershell
cargo test --quiet
git diff --check
```

## Notes

- Accepted fixture facts remain unchanged.
- Relation facts must be query trace evidence beside parentage, not hidden
  parentage replacements.
