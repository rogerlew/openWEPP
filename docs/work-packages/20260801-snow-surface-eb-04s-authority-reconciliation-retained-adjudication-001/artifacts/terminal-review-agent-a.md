# Terminal Science And Governance Review A

Evidence mode: `Static + Ran` (read-only hash and retained-report checks; no
model subprocess).

Reviewer: independent terminal reviewer A

Disposition recommendation: `GO_WITH_AMENDMENTS`.

The result-blind dimensional decision is sound, the authority reviews and seal
precede retained analysis, EB-04R and its retained output remain hash-stable,
and the reported retained population passes the authority-corrected
vapor-to-sublimation bound. The empirical result also reconstructs to
`177/16 -> 180/16`, so nonpromotion and the frozen stop-loss are directionally
correct. Two acceptance-evidence defects must be corrected before package
closure; neither requires or authorizes a model rerun.

## Findings

### High — empirical inputs and reducers are not bound to the frozen attempt before scoring

`tools/adjudicate_retained.py:89-100` verifies only attempt status, result count,
and cell keys before physical analysis. It does not verify the attempt-bound
tool/protocol hashes, decision-dependency hashes, current observation hashes,
fixture hashes, roles, filters, strata, climates, selector maps, or non-target
environment. Observations are then loaded at lines 109-114. The inherited
EB-04R validator contains these checks, but EB-04S does not call it; its
subprocess guard would also prevent using that validator unchanged.

This leaves acceptance criterion 7 (`package.md:115-116`) and the
`no_forcing_or_input_mutation` empirical criterion vulnerable to a changed
observation or reducer even though the retained runtime tree itself is sealed.
The package-tree hash at lines 147-162 covers EB-04R's package, not the external
observation files and scoring dependencies recorded in the attempt.

Required amendment: before any observation loader is called, independently
compare the current package-local protocol/tool, all decision dependencies,
the complete population manifest (including observation and fixture hashes and
roles/filters), cell selector map, and non-target environment with
`execution-attempt.json`. Record a machine-readable pre-observation PASS in the
retained report, then rerun analysis only. Do not invoke the model and do not
change EB-04R.

Independent terminal checks found that the current EB-04R tool and protocol
hashes, all 12 observation hashes, and the three external scoring-module hashes
examined match the attempt. This indicates an evidence-path defect rather than
observed scientific drift, but the package consumer must prove the complete
frozen set itself before closure.

Impact: closure-blocking scientific/governance evidence gap; in-scope to amend.

### Medium — the retained consumer does not enforce EB-04R's stricter vapor-aggregation gate

EB-04R prospectively froze daily vapor aggregation at `1e-12 kg m^-2`
(`EB-04R/artifacts/prospective-decision-protocol.md:31-36`). EB-04S imports the
EB-04E audit consumer, whose threshold is `1e-9 kg m^-2`, and the version-6
seal and conservation narrative retain that canonical EB-04E value
(`tools/adjudicate_retained.py:69-75` and
`artifacts/conservation-evidence.md:12-13`). The authority reconciliation
changed only the vapor-to-sublimation transfer bound; it did not prospectively
amend EB-04R's distinct aggregation gate.

Required amendment: add and report an explicit EB-04R-specific check that the
maximum daily vapor-aggregation residual is `<=1e-12 kg m^-2`, while retaining
the canonical `1e-9 kg m^-2` predicate as separate authority. Include this
stricter check in the population gate before observation access.

Ran: the retained population maximum is
`7.993605777301127e-15 kg m^-2`, so all 48 cells pass the stricter frozen gate.
This amendment should not change the empirical result.

Impact: package-gate legitimacy and precise frozen-protocol preservation;
in-scope to amend without rerun.

### Low — the required-reading map lists a fifth Phase A input

The authority manifest and frozen receipt correctly bind exactly four Phase A
inputs. However, `artifacts/required-reading-map.md:16` labels the EB-04E
`package.md` itself as a `Phase A whitelist` input, contradicting that firewall.
The authority tool and both reviews report only the four sealed inputs, so no
result-bearing leakage is evidenced.

Required amendment: relabel or remove that row so the reading map agrees with
the exact four-file authority manifest.

Impact: result-blind governance clarity; not independently outcome-changing.

## Verified Evidence

- Authority freeze SHA-256:
  `20c227029ccc876209cd81cdc830c9c68811307ee055d300836a769aa388798f`.
- Dual-verified authority seal SHA-256:
  `e60c3d0509f8fd8d512df843c205c615bc5cab26de5372837b409d94079bee0f`.
- Sealed version-6 contract SHA-256:
  `364a2bad34235c105cc4b47be50e12ca34e0b9e27b2aa2fd0c6842681670ab72`.
- EB-04R attempt SHA-256:
  `6cac934d17882e4696608dcd4b4f2da3a42ed91ce19f66a580718d73a626d211`.
- EB-04R package tree independently recomputed and matched:
  `e57b527be9da5c0c9c936453b192040595b9f08bf24cdbbea1e6e860c90dbe58`.
- Retained-output tree independently recomputed and matched:
  `d0ac593105afba10a92e6e530f76de7ae4120e4a8e82e87235ffd4080045f2f3`.
- Retained report contains 48/48 provenance and physical passes; 288 file
  identities are represented; maximum vapor-to-sublimation residual is
  `8.109983287707401e-8 kg m^-2`, below the reconciled `1e-6 kg m^-2` bound.
- No EB-04R history rewrite or model rerun was observed. File timestamps place
  the authority freeze, dual reviews, finding disposition, dual verification,
  and final seal before retained adjudication.
- The unchanged eight-part empirical reduction reports one failed criterion:
  robust failures do not decrease (`16 -> 16`). The ordinal score increase
  (`177 -> 180`) cannot override that failure. `CLOSE_NONPROMOTION_EMPIRICAL_RULE`,
  stop-loss invocation, no additional round, default-off mechanisms, and the
  withheld warm-maritime conifer transfer claim are truthful if the two
  closure-blocking evidence amendments above pass.

## Recommendation

`GO_WITH_AMENDMENTS`. Add complete frozen empirical-input/dependency validation,
enforce and report the EB-04R-specific `1e-12 kg m^-2` vapor-aggregation gate,
correct the reading-map label, and regenerate retained-analysis evidence. If
those checks pass with the already observed values, terminal disposition may
remain `CLOSE_NONPROMOTION_EMPIRICAL_RULE`; otherwise the package must `HOLD`.
