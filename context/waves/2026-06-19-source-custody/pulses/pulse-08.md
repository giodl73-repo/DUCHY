# Pulse 08: Contested-History Review Packet

## Intent

Represent contested source-backed claims as reviewable alternatives instead of
letting the importer choose one claim and silently overwrite the other.

## Implementation

- Add `ContestedFactGroup`.
- Add `contested_fact_groups`.
- Keep `ConfidenceLabel::Contested` accepted only when a `conflict_group` is
  present.
- Block title materialization when any fact for that title is contested.

## Behavior

Contested facts can pass the source fact gate if they identify a conflict group.
They cannot produce a normal `Title` record until the conflict is resolved or a
future contested answer surface is implemented.

## Boundaries

The CLI demo uses synthetic contested rank alternatives to exercise the review
path. It is not a historical claim about Q158445 and does not add another real
title fact.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
