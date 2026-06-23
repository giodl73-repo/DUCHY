# Pulse 178: Margraviate of Austria Bavaria Held Candidate

## Intent

Capture a reviewed immediate-parent lead without importing an invalid active
parentage fact. The Margraviate of Austria source supports a Bavaria relation,
but the current DUCHY rank model cannot materialize it as parentage.

## Finding

- Margraviate of Austria is currently ranked `Duchy`.
- Duchy of Bavaria is also ranked `Duchy`.
- The validator rejects `Duchy` -> `Duchy` active parentage.
- The attempted replacement of direct HRE parentage with Duchy of Bavaria was
  backed out before regeneration.

## Artifact

- `data/staging/margraviate-austria-bavaria-held-candidate.md`

## Current State

- accepted fixtures unchanged by this pulse
- Margraviate of Austria -> Holy Roman Empire remains active for `976..1156`
- no new accepted source record
- no new accepted fact

## Validation

- `cargo test --quiet`
