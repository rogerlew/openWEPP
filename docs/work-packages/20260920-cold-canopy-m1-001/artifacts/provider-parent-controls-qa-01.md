# Provider-parent controls QA — 01

**Reviewer:** `/root/provider_qa` (independent QA)  
**Scope:** frozen contract-derived expected-red provider controls only; no provider
body, source-adapter implementation, parent integration, or command launch.  
**Evidence:** Static. No Rust compilation or test execution was performed.

## Findings

### BLOCKING — canonical payload and two receipt domains have no independent known answer

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:109-148,238-269`

The independent calculations cover the run identity and only the first GSI
support receipt. They do not calculate and compare a fixed canonical payload
byte/hash known answer, calendar receipt, or parent-forcing receipt. The test
instead obtains the payload from `FixedSequenceSourceAdapter` and admits it,
then uses that same production-derived payload as the preimage for its checks.
That cannot detect a shared wrong payload construction, omitted/incorrect
projection member, or a calendar/forcing framing error. SC-VEGETATION-001 v36
requires the canonical-payload known answer and all four provider identity/
receipt domains.

### BLOCKING — schedule, cycle, day, phase, and projection coverage is incomplete

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:160-196,238-269,273-306`

The fixture oracle confirms both cycles have contiguous 60-second-divisible
segments and 4,320 records, but it seals only cycle zero's first 60-second
breakpoint. The passing path checks cycle zero parent zero and records 0/29;
it does not independently check cycle one, all retained breakpoint positions,
day mappings, distinct cycle initial-phase bytes, the final record/parent, or
each full projection. A parser accepting a shifted whole-minute internal
breakpoint or a wrong second-cycle day/phase can therefore satisfy these
controls. V36 requires both full projections and every breakpoint, day, cycle,
and phase identity.

### BLOCKING — mutation inventory does not cover the full closed source and implementation bindings

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:277-358`

The cases mutate source zero's SHA only and implementation path only. They do
not independently poison both mandatory source names/paths/order and source
one's SHA, implementation version or SHA, or all required CalendarV1 and
ProjectionV1 closed fields. For source joins, `locator_kind` is poisoned only
at index zero; the loop omits it for the other six joins. The current-GSI
adapter object's required path/hash/manifest/accessor/output-hash/pointer
members are also not individually poisoned. This falls short of the v36
source/path/pointer/kind/value/implementation mutation obligation.

### BLOCKING — semantic canonical-encoding negatives are too narrow

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:337-357`

The byte-level cases cover duplicate `provider_id`, leading whitespace, one
leading-zero integer, and CRLF. They do not exercise the required non-ASCII,
illegal escape spelling/case, escaped slash, nonlexicographic key order,
unknown/duplicate nested key, signed/other malformed integer, or non-hex
binary64 encoding. Without these cases a permissive parser can fail a malformed
encoding before reaching the intended semantic-negative boundary, leaving the
contract's canonical parser obligations unproved.

## Non-blocking follow-up

Keep the useful pieces: direct hashes of both frozen source artifacts
(`fixture_oracle`), independent segment-contiguity/count checks, direct
run/GSI framing code, real planned admission seam, audit assertions for no
staging/publication on rejection, and missing/foreign/reordered/stale GSI
receipt poisons with a passing sister case. The controls are correctly wired
under `#[cfg(test)]` in
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:95-99`.

The module body is deliberately absent, so compile failure at the named API is
expected and no execution conclusion follows from this review.

## Verdict

**HOLD — do not release these controls as the v36 provider-test gate.** Add the
missing independent known answers and mutation/coverage cases above, then
freeze the revised source and perform a new bounded QA review before provider
implementation or result-bearing execution.

## Compile-manifest clearance — correction

**Retraction:** the preceding manifest finding was incorrect. I read
`(source / 'src').rglob('*.rs')` as if it traversed every crate source tree.
It does not. It reaches only the linked workspace-root `src/lib.rs`, which is
listed and pinned by the manifest. The canonical `SNAPSHOT.snapshot` function
separately enumerates every file under `source/crates` and the detached root
manifests; hence the provider controls and `mod.rs` are covered by the declared
758-entry snapshot `0bf36ce5bbbc96b6417883c9f7422989a448a6e83686ead17efcff23b9a68cb3`.
The recorder comment at `run_recorded.py:109-110` correctly describes its
per-file `src` enumeration as external support, not detached crate coverage.

**PASS — manifest/custody clearance for the one nonphysical compile command.**
Static inspection confirms manifest SHA-256
`d7266243eafc4ecf0614adbbaa5ae7988fe54f10c1cdc5f35a0c681d05f62e71`,
the detached source root and stated snapshot, authoritative/input roots,
unchanged support links with selected target content, the workspace-root source
pin, and the exact nonphysical argv/timeout policy. The command is `nix develop
/workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target
CARGO_BUILD_JOBS=2 cargo check -p openwepp-hillslope-orchestrator --tests`,
with a 180-second bound, detached source root, and no binary requirement. It
remains subject to the recorder's fresh time/reserve and no-collision checks at
dispatch. This clearance does not change the substantive provider-controls
**HOLD** above, nor does it make a compile result provider or M1 acceptance.

## Identity

Reviewed detached files:

- `m1_fixed_sequence_provider_controls.rs` SHA-256
  `fe4b26f2872defa38fbbac2211124451317069fc32403d268e572c45b5ab7045`
- `mod.rs` SHA-256
  `240a2238531c817634ce58e20a2b15efa4d4222416e2c4ca534e448b24091aad`

The parent reports the recorder-algorithm source snapshot as
`0bf36ce5bbbc96b6417883c9f7422989a448a6e83686ead17efcff23b9a68cb3`.
This review independently rechecked the two scoped file hashes; it did not
reconstruct the full recorder snapshot.
