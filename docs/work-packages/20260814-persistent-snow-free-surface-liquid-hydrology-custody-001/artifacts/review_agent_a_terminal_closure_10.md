# Review Agent A — Terminal Closure 10 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `f375349e38c134eb6eb105d03a87d85841ba8c26`

Verdict: `HOLD / NO-GO`.

The review used a source archive generated from the exact reviewed Git object.
Concurrent work later modified the shared checkout; none of those later Rust
bytes is assessed or used as evidence here. This review adds documentation
only.

## Findings

### High — Ending-state replacements still report the missing expected identity instead of the actual offending row

The aggregate half of `A-TERMINAL-CLOSURE9-HIGH-001` is corrected:
owner/configuration and digest-wide E010 failures now carry transaction and
owner with typed absence for OFE/tile/surface/source. Missing rows also retain
the exact expected missing member, and pure additions, duplicates and reorders
retain an available actual member.

The replacement case remains incorrect. In
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:2326-2346`,
`first_membership_aware_mismatch()` searches for a missing expected identity
before it searches for an actual identity outside the expected set or the
first positional actual mismatch. For an equal-length replacement such as
actual `forged,middle,lower` versus expected `upper,middle,lower`, the helper
therefore returns `upper`, not the available actual offender `forged`. The same
precedence applies to persistent store keys and WB14 continuation OFE IDs at
`:2375-2385` and `:2414-2423`.

The new tests encode rather than catch that drift:

- the wrong store identity copies the middle key over the upper row but expects
  the reported store to remain `upper` at
  `surface_liquid_ingress_tests.rs:2276-2292`; and
- the wrong continuation changes the actual OFE to `forged` but expects
  `upper` at `surface_liquid_ingress_tests.rs:2322-2338`.

That contradicts the accepted closure-9 correction, which required the actual
replacement/reordered row when present, and it contradicts the package claim
that replacements carry the exact replacement identity in
`artifacts/gate-results.md:684-686`. It also leaves the requested first/middle/
last addition, deletion and replacement matrix incomplete; the tests cover
one middle deletion, one trailing duplicate, one first-pair reorder and one
first-row replacement and do not assert the complete phase, context and
rollback payload.

The candidate still rejects E010, so no wrong store or continuation is
accepted. This remains high severity because exact contextual identity is a
canonical public failure payload under `SC-SURFACELIQUID-001:477-481` and is
required for rollback localization. Select the expected missing member only
for a true deletion; otherwise prefer the actual positional/extra/replacement
row. Add exact first/middle/last structural poisons and assert code, phase,
transaction, owner, row context, typed absences and beginning/attempted hashes
at the complete public error boundary.

### Medium — The affected-file inventory still lacks required WARN/split dispositions for three 2,000-line Rust modules

The closure-9 ingress-test governance finding is corrected:
`surface_liquid_ingress_tests.rs` is now accurately counted at 2,759 lines,
marked WARN, given a cohesion rationale and assigned a future fixture/vector
split. The newly enlarged 2,014-line ingress module is likewise WARN with a
future dependency-neutral identity split.

The complete affected-file inventory is not yet compliant with
`crates/AGENTS.md:57-60`, which requires every Rust file at or above 2,000 lines
to be WARN with both decomposition rationale and follow-on split intent:

- `direct_runtime/runoff.rs` is 2,852 lines, but
  `artifacts/line-count-governance.md:9` does not mark it WARN and names only a
  completed WB14 extraction, not a follow-on split;
- `direct_runtime/surface_liquid_owner.rs` is 2,347 lines and its row at `:11`
  gives a cohesion rationale and completed test extraction but no follow-on
  split intent; and
- `vegetation_real_hydrology_shadow.rs` is 2,157 lines and its row at `:13`
  gives a change-scope rationale but no follow-on split intent.

No affected Rust file reaches the mandatory 3,000-line refactor threshold.
This is a bounded documentation correction, but the artifact's statement that
all WARN files satisfy `crates/AGENTS.md` is currently unsupported.

## Closure-9 Remediation Re-Audit

The other closure-9 obligations are corrected and no numerical change is
hidden in their centralization:

- One five-field key now defines start, end, origin store, kind and source-ID
  order in `surface_liquid_ingress.rs:611-630`. Production and the frozen and
  projected closure adapters all use it. Production allocation and the
  receipt-free expected allocation remain separate implementations.
- One typed constructor now defines local and condensation source IDs at
  `surface_liquid_ingress.rs:581-609`. Production creation, frozen capture and
  identity reconstruction consume it; physical partition, retention and
  routing arithmetic are not shared.
- The mixed-kind, unequal-area, downstream-overlap fixture at
  `surface_liquid_ingress_tests.rs:345-797` freezes every emitted source,
  basis, routed kind, disposition, typed recipient, support, mass, mixture
  temperature and enthalpy bit. Its exact rows contain nonzero infiltration,
  retained surface water, routed runoff and outlet runoff, and bind final-
  remainder ownership.
- The same fixture freezes both ending stores and both WB14 continuation
  records, including cumulative supply/infiltration, cadence and transaction
  lineage, and proves caller-ingress-order invariance.

The bit-frozen receipt temperatures bind the chronological mixture specific
enthalpy, and the bit-frozen receipt enthalpies bind Q and its per-source
attribution. The vector therefore protects the now-shared ordering definition
without coupling the two physical allocation implementations.

## Full Custody Endpoint And Historical Finding Re-Audit

- Expected partition construction still has zero receipt access. Actual
  receipts populate only the actual comparison map; expected WB14 partition,
  retention, routing, store and continuation values derive from frozen raw
  sources, beginning state and configuration.
- The independent replay retains and bitwise joins every ending store and WB14
  continuation before recomputing the complete state digest. A coordinated
  retained-operand/ending-store forgery remains rejected E010.
- Parcel identity still binds owner, source parcel, origin/current store,
  complete typed recipient, basis OFE, routed kind, support bits and
  disposition. Routed descendants become `UpstreamRunon` and apply the
  destination area conversion once.
- Raw Q remains bit-joined to mass times frozen specific enthalpy. The
  chronological projector retains `h_mix,b = supply_enthalpy / supply_mass`
  and assigns infiltration, retention and runoff enthalpy from that mixture.
- Arithmetic/domain E003 preflight still precedes immutable producer E009,
  which precedes independent E010. Both cumulative-infiltration bounds are
  checked before the zero-supply shortcut.
- Exact D/A/F custody, finalized-use-only debit, signed condensation,
  pre-ingress capacity, one stateful WB14 call per OFE, receiving soil-liquid/
  soil-thermal/retained-LSE identities, strict restart lineage and snow/frost
  rejection remain intact.
- Candidate construction and validation remain clone-only. Public error
  completion retains beginning and attempted owner hashes; no partial mutation,
  selector activation, publication or serialization change is introduced by
  this remediation.

No new arithmetic, clamp/guard-precedence, unit-conversion, constitutive,
serialization, receiver, rollback or production-selection defect was found in
the exact bytes beyond the contextual replacement defect above. The remaining
production/projector algorithm duplication is intentional and justified by
the independent anti-tautology requirement; canonical identity, constants and
ordering are centralized.

## Exact-Commit Validation

Ran from a source archive generated from
`f375349e38c134eb6eb105d03a87d85841ba8c26`:

```text
CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  -p openwepp-hillslope-orchestrator --profile quick
PASS: 562/562; 3 slow retained routing-oracle tests; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo clippy \
  -p openwepp-hillslope-orchestrator --all-targets --all-features -- \
  -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check \
  fd8633865df289620aa5b9cf8c4e1bd206432f30...\
  f375349e38c134eb6eb105d03a87d85841ba8c26
PASS
```

These commands prove the focused authority/consumer surface, the complete
owning-crate quick suite and strict source quality for the exact reviewed
bytes. They do not prove the missing exact replacement context or the omitted
complete structural-context matrix. Full-workspace, doctest, dependency-policy
and release gates were not rerun by this reviewer.

## Residual Risk And Missing Tests

Science-state risk is low because every identified malformed ending state is
rejected. Operational diagnostic risk remains material: an actual producer
replacement defect can be attributed to the missing expected owner member
instead of the malformed emitted identity. The exact first/middle/last
structural-context and complete public rollback-payload matrix remains missing.

The line-count defect is evidence-only and has no runtime effect. The package
must add concrete follow-on split intents for the three long modules named
above before claiming complete line-governance compliance.

## Approval Statement

`NO-GO`: exact commit `f375349e38c134eb6eb105d03a87d85841ba8c26`
corrects aggregate typed absence, canonical parcel identity/order, the frozen
mixed-route numerical oracle and the closure-9 ingress-test WARN. It does not
correct actual replacement-row attribution, and its affected-file line-count
artifact remains incomplete. Correct those bounded findings and rerun the
focused endpoint/context, routing-oracle and strict source-quality gates before
terminal approval.
