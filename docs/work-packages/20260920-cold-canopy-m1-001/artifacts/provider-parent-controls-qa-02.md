# Provider-parent controls QA — 02

**Reviewer:** `/root/provider_qa` (same independent QA reviewer; fix check of
the review01 findings).  
**Evidence:** Static. No Rust compilation or test execution was performed.

## Findings

### BLOCKING — expected phase records violate the required binary64 payload representation

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:434-450,698-703`

`expected_payload` copies `continuous-fixtures-draft01.json`'s
`initial_phase_records` directly. Those source fields are JSON decimal numbers,
whereas SC-VEGETATION-001 v36 requires each
`mass_kg_m2_tile_ground` and `enthalpy_j_m2_tile_ground` payload member to be a
16-lowercase-hexadecimal binary64-bit string. The resulting equality assertion
would either reject a conforming provider payload or bless a provider that emits
the wrong decimal schema. Construct each expected phase record from source
binary64 `to_bits()` values and assert the exact strings.

### BLOCKING — adapter root pointer is semantically wrong and the claimed dependency manifest is not source authentication

`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:45-47,372-381,424-430,871-892`

The exposure, topology, and controller adapter locators use `output_pointer:
"/"`. Under the required RFC 6901 semantics, `/` selects an object member with
the empty key; it is not the whole document. The root pointer is the empty
string. These rows therefore cannot authenticate the stated complete
constructor-output objects.

Further, every adapter locator names `crates/openwepp-hillslope-orchestrator/Cargo.toml`
as its `dependency_manifest_path`. Cargo metadata does not enumerate the
retained input source paths and SHA-256 values that the contract requires the
adapter to verify. The tests mutate the locator label, but no actual
dependency-manifest object or source-hash inventory is constructed or checked.
That leaves a synthetic metadata label able to stand in for the required source
authentication. Provide a small frozen adapter dependency manifest with the
actual source paths/hashes and bind both its bytes and each listed dependency in
the independent oracle and planned admission API.

## Fix verification and non-blocking notes

Review01's other substantive gaps are corrected in this cut:

- `expected_payload` and direct fixture/endpoint reads provide an independent
  full-payload oracle; independent run, calendar, GSI, and parent-forcing
  framing functions are present.
- Both cycles, all 4,320 records per cycle, all 144 parent slices, calendar day
  mapping, phases, and per-parent forcing receipts are covered at lines 687-752.
- All seven closed join fields and adapter locator members receive value and
  omission poisons; source, implementation, projection, segment, record, and
  override coverage is substantially expanded at lines 755-1067.
- The parser vectors now include non-ASCII, escaped slash, alternate escape,
  nested duplicate, and non-hex binary64 forms; receipt poisons retain a
  passing sister case. Caller snapshot checks use the planned real-caller seam.

Static inspection confirms the stated file identities:

- controls SHA-256 `7e9f5da56cba985289da2d92abc35cb1e854a542bcb41a8d882ddfa4fbd8e1ef`
- `mod.rs` SHA-256 `240a2238531c817634ce58e20a2b15efa4d4222416e2c4ca534e448b24091aad`
- parent-supplied canonical recorder snapshot
  `17c71b369c3475c19cc173f1e6201796c7f1095313d43098dc17873a7f09e7d8`

The planned provider source file is intentionally absent; its `include_bytes!`
and imports therefore remain expected-red compilation failures until the
private body exists. This review does not treat that absence as a source-custody
defect. The controls02 static cut also preserves the earlier corrected
understanding that the recorder's canonical snapshot, not its linked `src`
support traversal, covers detached `crates/**` files.

## Verdict

**HOLD — do not release controls02 as the provider-body gate.** Correct the
phase bit encoding and replace the non-manifest adapter provenance labels with
an actual frozen dependency manifest and root-pointer semantics; then freeze
the revised cut for a bounded fix check. No physical or provider acceptance is
implied.
