# CAL-07 Terminal Verification A

Verifier: `/root/cal07_terminal_verify_a`

Evidence class: `Ran + Static`

Disposition: `PASS FOR HOLD CLOSURE`

This is a pass for the fail-closed `HOLD`, not a pass of CAL-07's
result-bearing analysis or roadmap Order 7.

## Independent Checks

### Frozen-input VPD reconstruction

Ran an independent standard-library Python calculation directly over
`inputs/forcing.csv`, using
`1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))` and
`es(T) = 0.6108 * exp(17.27 * T / (T + 237.3))`.

Result: `PASS`.

- 3,332 forcing rows: 1,666 per site.
- Every reconstructed value was finite.
- Exactly three values were negative, all for `SH-EN-ALERCE`:
  - 2022-07-22: `-58.860502313193393 Pa`
  - 2022-09-15: `-70.492437340680112 Pa`
  - 2025-09-09: `-1.0022421439928042 Pa`
- The independently reconstructed dates and values agree with
  `negative-vpd-days.csv`; the 2025 event is inside a prespecified scoring
  year.

No clipping, deletion, substitution, or assumed correction is scientifically
authorized by the frozen package.

### Hold validator and publication boundary

Ran:

```text
.venv/bin/python docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/tools/validate_hold.py
```

Result: `PASS`:

```text
CAL-07 HOLD validation PASS: 3 negative VPD days; no partial canopy result
```

The validator also reproduced all source-manifest sizes and SHA-256 identities,
the complete 37-member custody inventory, 934 admitted Beza camera days, 925
admitted Alerce camera days, all four SVG/sidecar pairs, SVG accessibility
metadata, and required sidecar sections.

I also invoked the Rust executor independently with a temporary output target:

```text
cargo run --quiet --manifest-path docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/tools/executor/Cargo.toml -- docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/inputs/ensemble.csv docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/inputs/forcing.csv <temporary-directory>/daily.csv
```

Result: expected exit `1` with
`invalid VPD for SH-EN-ALERCE 2022-07-22`; the temporary daily output did not
exist. Static inspection confirms the executor buffers output and writes it
only after every member/site/day succeeds.

The package artifacts contain none of:

- `daily-kernel-output.csv`
- `gate-results.csv`
- `ensemble-daily.csv`
- `shape-scores.csv`
- `transition-residuals.csv`
- `verdict-matrix.csv`

Result: `PASS` for no partial result publication.

### Deterministic diagnostics and figures

Ran `tools/diagnose_forcing.py` and `tools/plot_hold.py`, then compared
SHA-256 identities with the pre-rerun identities. Every identity was
byte-for-byte unchanged:

| Artifact | SHA-256 |
| --- | --- |
| `forcing-diagnostics.csv` | `a705f8c935f6aa5486f3a28ab63a85e5289e8a9776a474d9dedb8093528a1719` |
| `negative-vpd-days.csv` | `a31a5d078922580a920f469d2cfd0d3d1c911f1016c6f7b75a61b642d060eb17` |
| `observation-source-summary.csv` | `aeea1445e32b5f6b32d907fc161d9191e156640e726dc81b2d748208c73a18a6` |
| `cal07-forcing-vpd-compatibility.svg` | `255e80878bcfa9c657f6d30f05c9be08d8eea178a881eecd66125402cb63b376` |
| `cal07-hold-evidence-boundaries.svg` | `385d5224545d405b35ae2e0d995e84fb8e31714296dbd4f0aa729c39f762fb97` |
| `cal07-negative-vpd-operands.svg` | `6a9e5ae5ffd0b9b10e9e0bf17ab44348a5a332629cdcc5ff82588e79a0606f07` |
| `cal07-observational-lanes.svg` | `72c5ef1022cb835af307adfc5631445569528fe167204ba921a3a76a19549c11` |

Result: `PASS`.

### Disposition and roadmap consistency

Static inspection found consistent terminal language:

- `package.md`: `hold / forcing authority incompatible`
- `artifacts/final-disposition.md`:
  `HOLD / FORCING AUTHORITY INCOMPATIBLE / NO CANOPY RESULT`
- work-package catalog: same hold, three negative Alerce days, no partial
  result, and an explicit retry prerequisite
- canopy roadmap Order 7: not passed and remains open

The focused producer-phase and real-consumer tests did not run after the
executor failed. The disposition says so and does not use their absence as
positive evidence. Absolute amplitude, quantitative evergreen-floor
agreement, phase-transformed real-consumer chronology, and source-incomplete
downstream physics remain unevaluated. This is consistent with the
gate-non-deferral rule because the package is held rather than marked complete.

`git diff --check` produced no output.

Result: `PASS`.

## Verdict

`PASS FOR HOLD CLOSURE`. The forcing incompatibility is independently
reproduced, the executor fails before publication, deterministic diagnostics
are stable, and package/catalog/roadmap claims remain calibrated to the
evidence. CAL-07 has no result-bearing scientific verdict, and roadmap Order 7
must remain open until the named forcing-authority prerequisite is satisfied.
