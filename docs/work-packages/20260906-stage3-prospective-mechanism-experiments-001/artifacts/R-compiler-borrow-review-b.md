# R compiler-negative custody QA review B

Static: source-only review, no compiler/test invocation. Root/work-package
AGENTS apply; instruction discovery completed before this artifact edit.

## Findings first

- RCB-1 / P1, `reproduction/check_replay_borrow_guards.py:119` and
  `verify_source_kit.py:18,66`: verified reconstruction excludes docs/, yet the
  positive `cargo check --tests` compilation embeds documentary fixture bytes.
  Direct LSE tests require four documentary inputs; production vegetation V9
  embeds two further docs required by the runner dependency. The existing
  source-kit identity cannot certify those inputs or reconstruct this compile.
  Positive compilation should correctly fail rather than fabricate custody
  evidence, but the harness is not ready to execute until a frozen documentary
  compiler-input adjunct is preserved and identity-checked before both phases.
  No unbound live-doc copy/fallback is acceptable. Parent owns source-kit
  correction; test worker owns consuming the declared verified adjunct API.

The source scanned for this finding is runner SHA-256
`8b3f9e03ae446f7ee1d82c8f49b376a621c28e6ccb1d179099c734b88ca8660c`;
negative snippet
`86ec7d5d90a19bbcc33b3693530e31a2c551c5ae91447bee3b59207297993941`.
My initial message's graph-canonical-SC include example was incorrect: the
actual graph test has no document include macro. The LSE fixture omission is
independently sufficient for the finding. Likewise the V7 definition include
is test-only; V9 vectors/runtime descriptor are the unconditional production
documentary operands.

## Gate quality otherwise

The actual private-type snippet exercises second consumed signed-proof use,
mutation while its base is borrowed, and dropping that base before the proof.
Expected E0382/E0502/E0505 codes must have primary spans wholly inside their
own exact function ranges in the injected actual test source. Private-symbol,
unrelated or unmatched errors reject. A successful structured positive Cargo
completion is required before any scratch-only injection; negative completion
must be false/nonzero and all three named errors present. This is meaningful
compiler custody evidence, not a textual symbol check or synthetic API clone.

Fresh reconstruction, exclusive output directory, exact appended bytes,
source/snippet preservation and post-compile manifest verification protect
custody. Existing source rows check bytes/hash/mode/symlink and deletion. Keep
the same checks for the documentary adjunct after each compile, and retain
its identity in source-verification/result evidence. Do not overwrite initial
provisional source kits or negative gate logs. Wrapper output is not accepted
as a Rust diagnostic. Scope remains compile-time ownership, not runtime
parity, full regression or physics qualification.

Non-blocking follow-up: preserve the exact runner script SHA with its command
and result, not only original/injected source and snippet hashes. Compiler
error-code expectations are intentionally fail-closed across toolchain changes;
an unexpected diagnostic requires inspection, never broad matching.

QA: HOLD RCB-1 source-input reconstruction; otherwise GO static negative-gate
design. No compiler-negative PASS claimed.

## RCB-1 concrete source-kit repair review

Static source review plus lightweight read-only JSON/tar/base-blob checks;
no reconstruction build or Rust execution. Instruction discovery again returned
root and work-package AGENTS. Corrective source identities:

- `freeze_source.py`:
  `34290b901f1c22892233eaaa03b49ba33e301ffb08a54eb6635f9a9854005c10`.
- `verify_source_kit.py`:
  `e2efe0369106fdbb47475f39b8518965bb021c3c3be80f06b242fcbe782d13dd`.
- A-source-03:
  `0c12a724516f973eb13f9de72afc4d767922a4440f897ecea545966936493038`.
- F-source-03:
  `b455bc706775e39b6ad7d112dbde75349ecaf9374107097609df33f1daea22fd`.

The parent now accepts the exact sorted15-path compiler documentary catalogue
and incorporates each path into the ordinary content identity, tracked patch
and untracked custody as applicable. Metadata separately lists the bound
catalogue. Verification rejects missing catalogue rows/out-of-scope manifest
paths, reconstructs exact documentary paths from the base rather than the whole
docs tree, and compares their bytes/hash/mode/symlink just like Rust inputs.
This is a compiler-input closure extension, not relabeling physical model
vectors as ignorable documentary outputs.

Read-only checks verified A's10146 and F's10150 rows; exactly15 docs in each,
all present, matching the independent per-arm inventory AND exact base blobs.
Both manifest digests and patch hashes match metadata. Actual untracked archive
members (A0, F4) are all identity-bound rows; no out-of-scope file is present.
Original limited kits remain historical and must not claim binary reconstruction.
The15 compiler inputs do not purport to cover every runtime filesystem read by
the entire workspace's tests; parent retains separate authority/test input
custody for that claim.

The compiler-negative runner's existing `unchanged_manifest` visits every
ordinary row, so it now checks these documentary inputs after positive and
negative compilation without a new live-doc fallback. The final R source kit
must have the same exact catalogue before invocation. Positive structured
compilation remains prerequisite to injection, and its future actual result
cannot be inferred from static reconstruction checks.

RCB-1 is resolved statically by this fix. QA GO exact repaired source-kit design
and reviewed A/F cuts; final R kit, actual compiler-negative execution and
runtime/scientific gates remain pending.

Non-blocking hardening: additionally require every untracked archive member to
be an included present manifest row, not merely a member of the metadata's
untracked list. The current producer and reviewed actual kits already satisfy
this; an explicit check would prevent a future out-of-scope documentary extra
from riding alongside an unchanged source identity.
