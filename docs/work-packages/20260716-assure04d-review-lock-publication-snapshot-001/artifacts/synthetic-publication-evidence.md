# ASSURE-04D Synthetic Publication Evidence

Status: PASS — test-only mechanics evidence

Evidence class: Ran

The focused positive publication contract executed entirely in disposable
`/tmp` source, staging, usersum-shaped, and snapshot roots. After successful
publication, the test-only public generation and immutable artifacts were
copied byte-for-byte into `synthetic-publication/`. These retained files are
non-operational package evidence; they are not a public report, scientific
approval, release snapshot, or vendored surface.

| Identity | Value |
| --- | --- |
| Trust domain | `test_only` |
| Test marker | `TEST ONLY — NOT SCIENTIFICALLY APPROVED` |
| Report | `linear-groundwater-reservoir-recurrence` |
| Release commit | `ec396c458a5015c504011a75814ff13e274544a1` |
| Release configuration | `openwepp-release-default-v1` |
| Public tree SHA-256 | `0547ffeb3e3c843ed727a3791d2c3443057b2be2181f674cc31379ba98259aef` |
| Snapshot ID | `e5348b835da39192a1d5c257cb44fdff5fef0a2edb11b44c20b273440e4ea647` |
| Receipt ID | `5b10bd8f50dbf0283d0519e75c4078c82ed3b7e04e6db75f206e226da7a120b1` |
| Manifest payload entries | 34 |

The snapshot ID equals the SHA-256 of retained `manifest.json`; the receipt ID
equals the SHA-256 of its retained JSON. Public report, catalog, README,
snapshot manifest, and receipt all retain the explicit test marker. The same
test proved identical retry, serialized simultaneous publishers, no-replace
conflict detection, unchanged public bytes after conflict, and rejection by
the actual release-candidate preflight before release-directory creation.

Command:

```text
OPENWEPP_ASSURE04D_RETAIN_ROOT=/tmp/openwepp-assure04d-retained \
  cargo nextest run --test assurance_v2_publication_contract \
  synthetic_approved_fixture_publishes_idempotently_and_release_rejects_it \
  --profile quick
```

Renewed result after the Phase 5 Clippy HOLD remediation: PASS, 1 run, 24
skipped; nextest run `261aad56-3785-48ae-96c1-1432a4fd8bbc`. The regenerated
public/snapshot/receipt tree compared byte-for-byte equal to this retained
tree.

## Exact-byte retention

The generated public README and its immutable snapshot copy end with the same
intentional terminal blank line. A package-local `.gitattributes` rule disables
only Git's `blank-at-eof` check for those two paths so their content-addressed
bytes remain unchanged; other whitespace checks remain active.
