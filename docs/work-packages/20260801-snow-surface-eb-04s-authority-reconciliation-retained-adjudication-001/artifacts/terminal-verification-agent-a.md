# Terminal Verification A

Evidence mode: `Static + Ran` (read-only terminal verification; no model
subprocess and no regenerated scientific output).

Verifier: independent terminal verifier A

Decision: `PASS_WITH_NOTES`.

All accepted terminal-review findings are closed for the current retained
adjudication. The regenerated result remains
`CLOSE_NONPROMOTION_EMPIRICAL_RULE`; no simulation rerun, EB-04R rewrite,
coefficient change, observation change, or empirical-rule change occurred.

## Finding Closure

### `TA-H1` / `TB-H1` — CLOSED WITH NOTE

`tools/adjudicate_retained.py` now performs all frozen identity checks before
the first `score_cell` call:

- current EB-04R tool and prospective protocol;
- EB-04 tool, protocol, and report;
- EB-04E report;
- all eight decision dependencies;
- complete `crates` and `tests` trees;
- EB-04R freeze receipt;
- all 12 fixture and observation hashes plus role, filter, stratum, climate,
  and lane metadata;
- B/L/S/LS target selectors; and
- all non-target selectors.

Each comparison is against `execution-attempt.json`. A mismatch raises before
physical acceptance or observation access. The regenerated report records nine
composite frozen identity checks and
`frozen_population_and_selector_bindings: PASS`; gate and summary evidence
expose the same pre-observation PASS. All 48 provenance records and 288 retained
file identities continue to pass.

Note: a future frozen-identity mismatch exits fail-closed before scoring rather
than writing a replacement report with the literal outcome
`HOLD_PHYSICAL_OR_PROVENANCE_GATE`. Such an execution must be dispositioned as
HOLD and must not reuse the prior retained report. For the current exact state,
all bindings pass and the empirical criterion remains admissible.

### `TA-M1` / `TB-M1` — CLOSED

The population gate now explicitly requires EB-04R's package-specific daily
vapor-aggregation residual to be `<=1e-12 kg m^-2` before observations are
loaded. The independently retained maximum is
`7.993605777301127e-15 kg m^-2`, so the stricter gate passes. The canonical
version-6 `1e-9 kg m^-2` vapor-aggregation predicate and the corrected
`1e-6 kg m^-2` vapor-to-sublimation transfer tolerance remain separately
described; no predicate was generalized or substituted.

### `TA-L1` / `TB-L1` — CLOSED

`artifacts/required-reading-map.md` now lists exactly the same four Phase A
authority inputs as the manifest and freeze receipt. The erroneous EB-04E
`package.md` whitelist row is gone.

### `TB-M2` — CLOSED

The authority tool now has a terminal `--verify-seal` mode that verifies the
frozen receipt hash, result-blind status, current version-6 contract hash, and
dual-verified Phase B authorization without pretending the prospective
version-5 self-check is rerunnable after amendment. Ran:

```text
EB-04S frozen authority and terminal seal verification: PASS
```

The gate table truthfully distinguishes the prospective Phase A self-check
from terminal seal verification.

## Independent Terminal Checks

Ran the retained-only adjudicator self-check:

```text
EB-04S retained-only adjudicator self-check: PASS
```

Independently recalculated and matched these current identities:

| Surface | SHA-256 | Result |
|---|---|---|
| Authority freeze | `20c227029ccc876209cd81cdc830c9c68811307ee055d300836a769aa388798f` | PASS |
| Authority seal | `e60c3d0509f8fd8d512df843c205c615bc5cab26de5372837b409d94079bee0f` | PASS |
| Version-6 contract | `364a2bad34235c105cc4b47be50e12ca34e0b9e27b2aa2fd0c6842681670ab72` | PASS |
| EB-04R attempt | `6cac934d17882e4696608dcd4b4f2da3a42ed91ce19f66a580718d73a626d211` | PASS |
| EB-04R package tree | `e57b527be9da5c0c9c936453b192040595b9f08bf24cdbbea1e6e860c90dbe58` | PASS |
| Retained-output tree | `d0ac593105afba10a92e6e530f76de7ae4120e4a8e82e87235ffd4080045f2f3` | PASS |

The report records `model_rerun: false` and zero EB-04S simulation
subprocesses. Its before/after EB-04R and retained-output tree hashes remain
equal.

## Outcome Stability

- B remains score/failures `177/16`.
- LS remains score/failures `180/16`.
- All physical/provenance and protected criteria pass except the prospectively
  required robust-failure decrease: `16 < 16` is false.
- The independent decision reconstruction agrees with the inherited reducer.
- Outcome remains `CLOSE_NONPROMOTION_EMPIRICAL_RULE`.
- Stop-loss remains invoked; another factorial/calibration round remains
  unauthorized; mechanisms remain default-off; warm-maritime conifer transfer
  remains withheld.
- EB-04R remains an unchanged historical HOLD.

## Final Verification

`PASS_WITH_NOTES`. All TA and TB findings are closed for the exact retained
evidence and current hashes. The note concerns only how a future identity-drift
failure should be materialized; it does not weaken current result admissibility
or authorize further science execution.
