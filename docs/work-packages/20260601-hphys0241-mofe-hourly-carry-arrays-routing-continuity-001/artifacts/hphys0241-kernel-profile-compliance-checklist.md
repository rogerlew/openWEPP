# HPHYS0241 Kernel Profile Compliance Checklist

Status: complete
Evidence mode: static + ran

Static checklist:

- Contract-first sequencing: complete. Canonical `SC-*` amendments and
  contract-derived tests were recorded before production-code edits.
- Canonical contract authority: complete. New runtime behavior traces to
  `SC-WATBAL-001`, `SC-RUNOFFPART-001`, `SC-SYSTEM-001`, and `SC-ROUTE-001`.
- Baseline provenance: complete for carry-array symbols and copy-forward
  behavior, using pinned `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Typed guards: complete. Missing, malformed, non-finite, negative, wrong
  cardinality, inactive metadata, and aggregate anti-shadow cases hard-fail.
- No silent defaults for required dependencies: complete for active multi-OFE
  carry arrays. Area-ratio provenance is required when non-zero upstream carry
  is consumed.
- No proxy physics: complete. Positive saturation carry requiring unresolved
  hourly cadence lineage hard-fails and is handed off to HPHYS0242.
- Security impact: low; local flat-file code/docs/tests only.

Ran checklist:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with existing duplicate/unmatched-license warnings.
- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract`: pass.
