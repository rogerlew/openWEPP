# Provider-parent controls run QA — 04

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static manifest/source inspection and inspection of the supplied Ran body04 build/list receipt. This reviewer did not execute the selected controls.

## Findings

No blocking QA finding for the exact source04 eight-control nonphysical run.

The selected control set remains the eight `m1_provider_` tests: payload/source oracle, both-cycle expansion, mutation refusals, receipt refusals, canonical encoding, real-caller refusal preservation, independent seven-owner bytes, and wrong-cycle/out-of-sequence refusal. The repaired mutation test now guarantees a changed override bit before admission refusal. Static inspection finds no support advancement, physical solver, or execution call in the provider/control paths.

## Exact binding

- Run support manifest: `provider-parent-controls-run-support-04.json`, SHA-256 `9216c314ff3ec2f0ff68eb29d89c4930f357f8019238d9272bcfb11940c2d844`.
- Detached source snapshot: 759 entries, SHA-256 `a717cc043b64603c19238642f5db0c1aefc09577ca2d0907dcede15eeaedfdd2`.
- Selected executable: `/tmp/openwepp-cold-canopy-m1-target/debug/deps/openwepp_hillslope_orchestrator-0ad665558c5e4d7c`, SHA-256 `086205641a824e71c3acdf798ee881cb4dec5e3dd4faa82a42018fccc1698fbe`; direct rehash agrees with the manifest.
- The 623-pin manifest binds the body04 source, successful build/list receipt, list output, executable, unchanged support links, Nix environment, target directory, and jobs setting. The inspected body04 receipt records detached-cwd build/list success in `38.65` seconds for the same source cut.
- The exact 180-second nonphysical invocation is:

  ```text
  nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(m1_provider_)' --no-fail-fast --success-output immediate --failure-output immediate
  ```

## Non-blocking follow-up

The repaired controls have not yet run. A passing result establishes only these eight admission/initialization controls. fmt, Clippy with warnings denied, broader tests, deny, physical parent work, and final acceptance remain separate missing evidence.

## QA disposition

**PASS — release the exact manifest-bound source04 eight-control nonphysical run.**
