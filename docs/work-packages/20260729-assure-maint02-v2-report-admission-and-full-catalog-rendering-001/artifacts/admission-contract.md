# New-Report Admission Contract

Status: frozen before production edits

Evidence class: Static

## Interface

```text
openwepp-assurance amend admit-report \
  --report <id> --path <manifest> \
  [--if-generation <generation-id>] (--check | --apply)
```

The report ID must use the V2 logical-ID grammar. The only admitted manifest
shape is `assurance/v2/reports/<id>/report.yaml`, expressed as one confined
root-relative path.

## Preconditions

- The current generated identity verifies completely.
- The expected generation, when supplied, equals the current generation.
- The ID and manifest path are absent from the current catalog.
- The manifest is a regular nonsymlink file, declares the same ID, and is a
  production-domain, nonfixture `DRAFT`.
- Catalog-bound version, title, owner, trust domain, and fixture state are
  obtained from the manifest, not duplicated CLI input.
- Every manifest-declared path is confined, regular, readable, and
  content-identified into the candidate generation.
- A report-local `review.lock.json` does not preexist.

## Transaction

The operation appends one stable catalog row, generates an initial empty-event
review lock, generates the successor identity lock, and writes one canonical
receipt. All candidate bytes are validated through the real V2 repository
before active-tree exchange.

`--check` performs the same candidate preparation and validation but leaves the
active generation and all tracked bytes unchanged. `--apply` uses the existing
locked candidate-tree exchange. Any pre-exchange failure discards the
candidate; any post-exchange failure restores the previous tree or leaves the
documented recoverable cleanup state.

The receipt operation is `admit-report`, impact class is `scientific-full`, and
affected reports contain only the admitted report. A repeated request for an
already cataloged exact ID/path is a deterministic no-op only after the current
generation and report binding validate. Conflicting duplicate IDs or paths
fail.

Generated digests, identity locks, review locks, and receipt IDs are never
accepted as CLI inputs and are never hand-authored.
