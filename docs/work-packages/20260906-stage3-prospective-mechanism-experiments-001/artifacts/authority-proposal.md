# Prospective authority cut 1

Static: canonical authority and contract-binding tests only. Runtime behavior and measurements NOT RUN by this author. Status: READY FOR DUAL INDEPENDENT REVIEW, not implementation admission or production promotion.

Source base: `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`. Exact pre-edit reading/write map: `authority-required-reading.md` (270933 bytes, OK).

## Binding and protected scope

`EXP-STAGE3-20260906-F` maps to `SC-SNOWENERGY-001` INV-088/C-056; `EXP-STAGE3-20260906-R` maps to `SC-LANDSURFACEENERGY-001` INV-164/C-020. Each has an additive Binding Exposure Index row and a canonical non-versioned section. Contract versions/lifecycle registry are unchanged. Every former line and historical assertion remains intact: the tracked cut adds 204 lines and removes none.

These are prospective experimental qualification bindings authorized by the kickoff, not historical reinterpretation. F retains all typed guard, precedence, role/path, forced-two-call, closure and custody obligations. Its new baseline multiplicity is measured instead of inferred from historical 400/200. R retains the entire normative graph/crossability/custody/forced-complete matrix; its independent oracle reconstructs actual signed stencil probes, evaluator classification, starts, completions, errors, identities and dropped-record reconciliation. Nonzero completed real-primary replay is mandatory. No all-centered existential or hardcoded replacement histogram is imposed. Historical 64 MiB/fixed timing gates remain historically intact and are reported separately from new paired treatment decisions. No production promotion, hidden selector, changed solver/physics/tolerance/ownership policy or cross-evaluation result cache is authorized.

No new dimensional symbol, conversion, parameter, constitutive equation or calibration claim exists. Existing typed error mappings remain authoritative. A missing required scientific/typed proof prevents candidate admission; textual bindings prove no runtime connectivity or scientific correctness. Critical experiment correctness and scientific/output reconstruction remain parent-owned required execution before measurements.

## Immutable four-path cut

SHA-256 values identify the precise files submitted for independent review:

| Path | SHA-256 |
| --- | --- |
| `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `109be57ca5e5cac79262b9b22c5de18ec1d85eaac3a645ce85015bd4a2af78cf` |
| `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | `f31b98b1d6bad044db880746be413b488478b2fab3aa4b7e5429f3c3b4a83f87` |
| `tests/integration/land_surface_energy_balance_authority_contract.rs` | `aea74e997574cb620196ace0da204bfc29f8de3b0b276d4429d85c6448663e80` |

## Ran and not run

Ran from `/workdir/openWEPP`:

- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`: PASS, 52 consolidated rows.
- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`: PASS, 16 consolidated rows.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`: PASS, no findings.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`: PASS, no findings.
- `git diff --check`: PASS at author cut.

One initial binding-exposure invocation incorrectly passed two paths; exit 2 with usage requiring one contract. Corrected single-path invocations above passed. No semantic retry occurred. `rustfmt` is absent from the ambient PATH; parent should use the repository Nix environment. Cargo execution/formatting NOT RUN by this author to preserve serialized build/measurement scheduling.

Execute these focused contract tests before behavior edits:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract -E 'test(prospective_20260906_feed_forward_binds_science_and_separate_qualification) | test(revision_61_binds_one_typed_feed_forward_call_per_logical_terminal_group)'
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract -E 'test(prospective_20260906_replay_binds_stencil_oracle_and_scientific_separation) | test(version_thirty_one_binds_component_temperature_dependency_replay)'
nix develop -c cargo fmt --all -- --check
```

Run complete affected contract targets at the integrated checkpoint, preserving and separately classifying predecessor structural expected-red seams. These focused textual tests do not substitute for runtime expected-red, typed ownership, replay parity, full correctness or real-consumer tests. No external-authority suite posture/cohort/required-case binding changes occur in this author cut; parent reconciles anti-evasion applicability for the complete package.

Line-count disposition: snow integration file 2436 lines, WARN; LSE integration file 1833 lines, OK. Snow remains a pre-existing monolithic historical contract test host; the 42-line additive test avoids altering predecessor assertions during this experiment. Owner openWEPP maintainers/parent package; follow-on split intent is a separate test-organization change grouping terminal numerical authority revisions. Neither file reaches the 3000-line refactor gate. Canonical Markdown length does not trigger the Rust line gate.

Independent review must check qualification separation, binding conservation, unchanged historical assertions and protected science, not artifact presence alone. Dual verification and explicit finding disposition remain required before terminal package closure.
