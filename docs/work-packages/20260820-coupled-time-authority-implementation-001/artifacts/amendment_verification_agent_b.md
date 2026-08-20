# SC-COUPLEDTIME-001 amendment verification B

Verification date: 2026-08-20

Scope: exact current-worktree authority surfaces for SC-COUPLEDTIME-001 version
2 and `OPENWEPP_COUPLED_TIME_RESTART_V2`. Production Rust was not edited.

## Exact verification input

- Git HEAD: `42f88d644cf4f3c86bae0b9ae066505684699704`
- HEAD tree: `741778f5c08e245ff7325be9885b1025a6a4f142`
- Authority-surface manifest: sorted SHA-256 records for the contract, contract
  index, and the 50 files then present under this package's `artifacts/`.
- Authority-surface manifest SHA-256:
  `dc7a2f25de2e1b08d7b66c8b0630cb1394559ee981e42523bd01a66fbf5e5b77`
- Binary Git diff SHA-256 over the contract, index, and artifact directory:
  `2f0da88b936f26946d68673ada43e665d7fd1bdbd1d004d0bea2cab281954c06`

The manifest and diff hashes were computed before creating this verification
record, so the record does not self-alter its declared input digest.

## Gate results

| Gate | Result |
|---|---|
| Complete independent reference oracle | PASS, 108/108; result SHA-256 `4540951f70f9de0846669f8f955e7eeca425dd831108997f50009d6ec002df95` |
| V2 semantic/canonical poison population | PASS, 47/47; result SHA-256 `797f45478e94461c1e1740e20b16d010dd006ac2705d19b3b1b69721447ff1af` |
| Independent merged A+B -> B-to-C -> A+C chronology | PASS; output SHA-256 `a0e32b83623ebcbdaa79a14e5d760d984980b0b722423fa636b15361c8b6d586`; chronology digest `6b131695fda7f600344dc7c706f63e8c1cf86ef41ab72afd5583b8b76ff25971` |
| Independent restored parent/publication finalization KAT | PASS; result SHA-256 `3730e0d85eb945e652155f79a90761d0d42c6c1d33471eae9b3127dd8a29c115` |
| `git diff --check` | PASS |

The restored finalization identities were parent
`90627286f5cc4b6e341f0162323606013f0c0d8f58b2dd17615459befd6cfda3`
and publication
`5faa32af248f6d4badbb0d6b65cf075d18b25f3eaedd23a2d49e53f6ff574602`.

## Wire and compatibility verification

- Released `restart-schema.json` remains
  `OPENWEPP_COUPLED_TIME_RESTART_V1`, version 1.
- Its current SHA-256 is
  `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d`,
  exactly matching the bytes at authority checkpoint `30e82ab16`.
- The additive authenticated-chronology wire is separately named
  `OPENWEPP_COUPLED_TIME_RESTART_V2`, version 2, in
  `restart-schema-v2.json`, SHA-256
  `10fb181d1544df7f6a4bb07618c3891ed8de5916bf643a8be2e7722de8b92296`.
- The legacy `slab-receipt` identity KAT remains accepted; V2 slab/event receipt
  domains are separately named and bind parent, support, ordinal, owner, clock,
  segment, constraint, and ledger operands.

Technical verification confirms exact duration-bit reconstruction, parent
interval/transaction reconstruction, initial owner/clock anchors, active
segment reconstruction, next ordinal and last-step joins, deterministic merged
slab/event chronology, terminal owner custody, and independently reconstructed
parent/publication identities.

## Finding-disposition audit

Both final independent review appendices report PASS and explicitly close all
technical findings. However the canonical `amendment_disposition.md` has not
been updated after the final correction. It still says production work remains
paused until re-review and does not enumerate or disposition the subsequently
raised findings `V2-A-001`, `V2-A-002`, or review-B `RB1` through `RB3`.
Reviewer closure text is verification evidence; it is not a substitute for the
authority owner's mandatory finding disposition record.

## Verdict

**FAIL — governance-only release blocker.** All executable authority gates and
wire/versioning checks pass, and no technical finding remains open. The exact
authority checkpoint must not be created until `amendment_disposition.md` is
updated to disposition the second-round findings and remove its stale paused
state. After that artifact-only correction, rerun `git diff --check`, recompute
the exact authority manifest/diff hashes, and repeat or append this independent
verification. Production implementation remains paused until dual verification
passes against that exact corrected surface.

---

## Final verification after disposition correction

Re-run on 2026-08-20 against the corrected canonical disposition.

### Exact final input

- Git HEAD and HEAD tree remain
  `42f88d644cf4f3c86bae0b9ae066505684699704` and
  `741778f5c08e245ff7325be9885b1025a6a4f142`.
- Non-self-referential authority manifest: the contract, index, and 50 package
  artifact files excluding this verification record.
- Final authority manifest SHA-256:
  `75b57185e9b49a208407765dc2e02ba3a93d714d33508be66babc98f656defdb`.
- Tracked binary Git diff SHA-256 over the same authority paths, excluding this
  record: `2f0da88b936f26946d68673ada43e665d7fd1bdbd1d004d0bea2cab281954c06`.
  The manifest hash is the complete content identity because Git diff does not
  include untracked amendment artifacts.

### Re-run results

| Gate | Final result |
|---|---|
| Complete reference oracle | PASS, 108/108; `4540951f70f9de0846669f8f955e7eeca425dd831108997f50009d6ec002df95` |
| V2 semantic poison population | PASS, 47/47; `797f45478e94461c1e1740e20b16d010dd006ac2705d19b3b1b69721447ff1af` |
| Independent merged chronology | PASS; `a0e32b83623ebcbdaa79a14e5d760d984980b0b722423fa636b15361c8b6d586` |
| Independent restored finalization | PASS; `3730e0d85eb945e652155f79a90761d0d42c6c1d33471eae9b3127dd8a29c115` |
| V1 bytes versus `30e82ab16` | PASS; both `71c6905d9913ad3a8baccdef3785256c32ea89cff52c757ca157e0438711a05d` |
| `git diff --check` | PASS |

### Disposition audit

`amendment_disposition.md` now explicitly accepts and records the corrections
for `RB1`, `RB2`, `RB3`, `V2-A-001`, and `V2-A-002`, states that no finding is
waived, records both final review PASS verdicts, and retains the correct boundary
that production implementation remains paused through dual verification and the
exact amended-authority checkpoint. All review findings are dispositioned.

### Final verdict

**PASS.** Verification B approves the exact amended authority surface identified
by manifest SHA-256
`75b57185e9b49a208407765dc2e02ba3a93d714d33508be66babc98f656defdb`
for the amended-authority checkpoint. This verifies the contract/wire surface;
it does not approve the paused production Rust implementation.
