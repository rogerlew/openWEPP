# Hydrology, Science and Ownership Review — `15a110ece`

Evidence class: `Static + Ran`

Exact reviewed commit:
`15a110ece02941d02424dd54e111e6446e044e42`.

Verdict: `HOLD`.

## Material Finding

`B-TERMINAL-15A-HIGH-001`: attachment still permits E003 to mask E002.
`configure_surface_liquid_shadow()` invokes a frame helper that performs
production-lane numeric validation before state structural/digest identity.
Lane-area NaN can therefore preempt a later state-key or stale-digest E002.

Required correction is to split frame identity from lane numeric domain,
finish configuration/frame/state structural, cross-input and declared-digest
E002 validation first, and add lane-NaN × state-key/digest cross-poisons with
complete hashes and unchanged attachment bytes.

All other requested hydrology/custody surfaces passed. Ran evidence: LSE 28/28;
integration 66/66; custody authority 10/10; surface-liquid subset 86/86;
affected strict Clippy; formatting and diff hygiene.
