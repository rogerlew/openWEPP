# Provider-parent body QA — 01

**Reviewer:** `/root/provider_qa` (independent QA).  
**Evidence:** Static source and supplied compiler-log inspection. No Rust command,
test, or physical execution was run by this reviewer.

## Findings

No blocking QA finding in the assigned private-provider/body and selected-build
custody scope.

## Static verification

The frozen source receipt lists the reviewed provider,
provider-controls, and module bytes as respectively
`59b53d031612a47c103c59cec106a6c5fce4f1d845745b0287fd0d394e3371a2`,
`eec2e7c65a19e52d5a4eb7ee421a9943737fcfb5453861ccec386ff8a6448e42`, and
`21566e54078f1a6aa7271ecd4168da82d26b8c98472d53492ead698de1855263`; direct
rehash agrees. The supplied canonical detached snapshot is
`5cfa36252496195644931d63d359bb253bf2ff1783fb1f5af41fe2b01bbabc25`.

The provider is private under `#[cfg(test)]`. Its adapter rejects altered
manifest bytes and verifies the closed nine-entry dependency manifest before
canonical payload construction. Admission fail-closes with `VEG-E-144` before
record/receipt exposure. The controls retain actual source-adapter input,
full-payload/receipt checks, poisoned manifest control, and caller-snapshot
refusals. The real caller seam constructs from cycle phase records and retained
endpoint fixture, reads seven owner bytes including `snow`, checks the complete
seven-owner digest, and installs the coupled parent clock only after provider,
receipt, owner, and phase checks. This is static implementation evidence only;
it is not proof of a completed parent transition.

The supplied `provider-parent-body-compile-01.stderr` reports compiler warnings
but no error. It is evidence of the reported test-inclusive compiler PASS, not
a test result and not a strict-lint clearance.

## Selected nonphysical build clearance

**PASS** for `provider-parent-body-build-support-01.json`, SHA-256
`802139b9e96db46869a932218b44216669e4b65ba716451a1c5c682327d06f09`.
It binds the detached source root and stated snapshot, canonical authority/input
roots, material environment, all seven existing support links, and adapter
dependency manifest SHA-256
`2f0942ff787e493624ef86a5a639a563d8757df9015d9fc8ad09ded57db4a654`.
The exact selected argv is nonphysical and bounded to 180 seconds:
`nix develop /workdir/openWEPP --command env
CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo
nextest list -p openwepp-hillslope-orchestrator --lib --message-format json`.
The canonical recorder snapshot covers the detached `crates/**` provider body;
the linked workspace-root source and external support pins are separately bound.
The recorder must still enforce its fresh deadline/reserve and output-collision
checks at dispatch.

## Non-blocking follow-up

The compiler warnings in the supplied log must be handled by the later required
strict lint gate if they are introduced or selected by that gate. The actual
binary/list result, provider-control execution, complete-parent staging/commit,
real receiver progression, physical runs, balance, restart, and performance
acceptance are outside this clearance and remain unproved.

## Verdict

**QA PASS — release the selected nonphysical test-binary/list build.** The
private body is suitable for the next source-bound expected-red control run;
this verdict is not M1 or complete-parent acceptance.
