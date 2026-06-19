# Import Policy

## Allowed Now

- Fictional seed fixtures labeled `SourceClass::Seed`.
- Hand-authored test cases that make no historical claim.
- Metadata-only source records that point to candidate sources without copying
  source text, map geometry, or database rows.

## Blocked Until Review

- Bulk import of real European title, dynasty, territorial, or boundary data.
- Copying article prose, database text, map geometry, or commercial-game data.
- Publishing source-backed answers without source class and confidence fields.
- Mixing CC BY-SA or other share-alike material into MIT fixtures without an
  explicit artifact/package boundary.

## Future Import Gate

A future import must provide:

1. source record,
2. license posture,
3. fact extraction method,
4. confidence label,
5. citation pointer,
6. conflict handling,
7. review decision,
8. validation command.

## Artifact Boundary

Source-backed data should be packaged separately from Rust source code when the
license or attribution posture differs from MIT. The core crate may define
schemas and validators; source packages carry source-specific obligations.
