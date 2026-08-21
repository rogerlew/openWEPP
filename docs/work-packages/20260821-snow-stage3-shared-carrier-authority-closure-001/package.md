# Snow Stage 3 Shared Carrier Authority Closure

Status: `queued / Child 2C authority scaffold`

Date: `2026-08-21`

Package ID: `20260821-snow-stage3-shared-carrier-authority-closure-001`

Plan class: `Critical contract-first snow, vegetation, and land-surface-energy authority`

## Objective

Correct the Child 2C handoff and author the two coupled authority surfaces
needed before any production Rust is written:

1. one shared snow--canopy turbulent carrier for forest-covered V11 and Stage
   3 ground snow; and
2. deterministic event-boundary coalescing that cannot create a positive
   snow-free land-surface-energy segment below the active adopter's admitted
   physical support domain.

The package must freeze canonical equations, ownership, support aggregation,
event chronology, tolerances, tie-breaking, typed rejection, restart identity,
and conservation obligations. It must not admit compensated or sub-ULP LSE
storage arithmetic. The released Child 2B support-admissibility receipt remains
the authority consumed by Child 2C.

## Authority boundary

Child 2B is terminally released at the verified `origin/main` release
`1d0239f4aab78966537c465bdfd4d1efc69f5ef1`, with
`SC-VEGETATION-001@25`, `SC-VEGETATIONTRANSACTION-001@14`, and
`SC-LANDSURFACEENERGY-001@6` active. V11 full-support V10 compatibility,
segmented support, seven-owner custody, coupled-time chronology, and Restart V3
are protected inputs to this package.

The active covered-forest V11/LSE adopter admits positive physical support of
`>= 600,000,000 ns`. Coupled-time may represent a one-nanosecond structural
clock interval, but this package must not turn that fact into a constitutive
solver promise. A positive snow-free LSE segment below the active adopter's
support domain is rejected before Newton and leaves all owners unchanged.

## Shared carrier authority surface

For forest-covered snow, the canonical authority must make the topology
explicit:

```text
sealed reference atmosphere
          |
          v
   shared canopy-air node
       /             \
 V11 canopy       Stage 3 ground snow
 surfaces             surface
```

The carrier must jointly solve or iterate canopy sensible heat, canopy vapor
exchange, snow sensible heat, snow sublimation/deposition, shared canopy-air
temperature, shared canopy-air humidity, and canopy--snow--sky longwave. The
contract must bind equations and transfer geometry to authoritative sources;
this package may not invent a fixed multiplier, proxy, or fitted substitute.

Ownership is fixed as follows:

| Operand/process | Owner |
| --- | --- |
| Reference wind, temperature, humidity, pressure | Sealed half-hour forcing |
| Canopy structure and leaf/stem surfaces | V11 configuration/state |
| Shared canopy-air node | Carrier transaction |
| Ground snow temperature, SWE, liquid, cold content | Stage 3 |
| Snow roughness, emissivity, albedo | Stage 3 configuration/state |
| Canopy longwave properties | V11/LSE configuration |
| Segment support | Coupled-time slab receipt |
| Parent commit | Coupled-time complete-owner transaction |

The authority must explicitly reject raw 10 m wind used directly as subcanopy
wind, a fixed forest attenuation multiplier, independent canopy-air nodes,
duplicate vapor or sensible flux, post-melt snow flux, and canopy-intercepted
snow. Canopy-intercepted snow remains outside this campaign.

## Event-boundary coalescing authority surface

For parent support `[a,b)`, proposed terminal event `t*`, pre-event minimum
support `dt_min_pre`, and post-event minimum support `dt_min_post`, an accepted
event tick must satisfy both neighboring support conditions:

```text
t_event - a == 0 or t_event - a >= dt_min_pre
b - t_event == 0 or b - t_event >= dt_min_post
```

The authority must enumerate admissible integer boundary candidates, keep
`proposed_event_tick` and `accepted_event_tick` in the event receipt, and select
the nearest admissible candidate only when the independently admitted
event-time, snow-mass, liquid-mass, and energy tolerances all pass. The default
tie order to review is:

```text
smallest absolute displacement
-> lower combined normalized mass/energy error
-> earlier tick
```

The exact order is not accepted until dual science review confirms it.

Chronology is normative:

```text
solve terminal event proposal
-> enumerate admissible boundary candidates
-> select deterministically
-> recompute terminal snow state and ledgers at selected tick
-> accept zero-duration custody transition
-> execute successor regime only when support is nonzero and admissible
```

If neither neighboring admissible boundary satisfies the tolerances, the
parent or terminal-event solve fails and retries under its declared rollback
policy. The implementation may not drop a remainder, freeze snow-free state,
scale a longer LSE result, execute below-domain LSE, apply snow flux after the
accepted event, or apply snow-free flux before it.

## Active-adopter support aggregation

The canonical active-participant rule is:

```text
common_minimum_support = max(minimum support of every active physical participant)
```

Pre-event participants may include Stage 3, V11, and the shared carrier.
Post-event participants may include V11, snow-free LSE, surface liquid,
hydrology, soil thermal, and BGC. A one-nanosecond clock interval remains
representable when no active physical owner admits a one-nanosecond state
advance; only the physical successor segment is rejected.

## Implementation intent and protected scope

Intent is `contract authority and contract-derived verification`. This package
does not implement production Rust, change selectors/defaults, activate Stage
3, retire CoE, qualify a seasonal consumer, or perform empirical calibration.
After the authority package passes, a bounded implementation package may add
the default-off shared carrier and actual V11 snow-covered segment.

Protected boundaries:

- V10 physical behavior, full-support compatibility, DirectV10 Restart V1,
  coupled-time Restart V2, V11 Restart V3, and released Child 2B receipts.
- No compensated/sub-ULP LSE storage arithmetic, hidden physical duration
  floor, tolerance relaxation, or canonicalize-and-proceed behavior.
- No canopy-intercepted snow, fitted wind attenuation, production cutover,
  public-output acceptance, or CoE parity target.

## Included scope

- Correct the Child 2B worker handoff language.
- Amend the canonical `SC-*` contracts needed to bind the shared carrier,
  event-boundary coalescing, active-participant support aggregation, receipt
  fields, and typed failure/rollback semantics.
- Add independently authored reference vectors and contract-derived tests for
  carrier ownership, candidate enumeration, tie-breaking, support aggregation,
  conservation tolerances, wrong-regime flux rejection, and retry/rollback.
- Record operand lineage, source authority, validity domains, units, and
  authoritative-versus-diagnostic status.
- Complete dual science review, dual verification, finding disposition, and a
  worker handoff to the subsequent default-off implementation package.

## Excluded scope

- Production Rust implementation or runtime selector/default changes.
- Calibration, observational efficacy, seasonal qualification, deployment,
  release, or CoE retirement.
- Canopy-intercepted snow and Richards/Lane D authority.
- Any treatment that makes sub-ULP storage increments physically admissible.

## Dependencies

- Child 2B release `1d0239f4aab78966537c465bdfd4d1efc69f5ef1` and its
  `LseSupportAdmissibilityReceiptV1`.
- `SC-COUPLEDTIME-001`, `SC-LANDSURFACEENERGY-001`, `SC-SNOWENERGY-001`,
  `SC-VEGETATION-001`, and `SC-VEGETATIONTRANSACTION-001`.
- The active campaign coordinator
  `20260819-snow-stage3-production-cutover-campaign-001`.
- Existing Stage 3 wind-source/exposure and snow-energy authority packages.
- Pinned legacy provenance only where a canonical equation requires a source
  mapping; no legacy comparator result is authority by itself.

## Intended write set

- `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md`
  only if coupled-time event/support authority requires an amendment.
- `SC-LANDSURFACEENERGY-001.md`, `SC-SNOWENERGY-001.md`,
  `SC-VEGETATION-001.md`, and `SC-VEGETATIONTRANSACTION-001.md` only for the
  admitted shared-carrier/coalescing authority.
- `docs/specifications/science-contracts/index.md` for lifecycle updates.
- Contract-derived tests and independently authored vectors under `tests/` and
  this package's artifacts.
- This package tree, `docs/work-packages/README.md`, `docs/ROADMAP.md`, and the
  Child 2 campaign handoff files.

No production Rust is in the write set. Any expansion requires a prospective
scope, authority, and gate amendment before edits.

## Contract-first phase plan

1. Intake exact release identity, required reading, owner/consumer map,
   operand lineage, and protected write set.
2. Amend canonical contracts and schemas; author reference vectors and derived
   tests. No production Rust edits precede the contract gate.
3. Run the pre-implementation contract/profile/schema gates and reconcile the
   exact authority diff.
4. Obtain two independent science reviews and two independent verifications;
   disposition every finding explicitly.
5. Publish the authority checkpoint and a bounded handoff for the later
   default-off implementation package.

## Exit criteria

- Canonical contracts bind the shared carrier topology, equations/source
  provenance, wind/exposure authority, support aggregation, event receipt,
  candidate enumeration, tie-breaking, tolerances, retry, rollback, and
  wrong-regime rejection.
- The stale handoff line is replaced exactly and no artifact admits sub-ULP
  LSE storage arithmetic.
- Contract-derived positive and poison vectors cover unequal supports, exact
  minimum support, one-nanosecond structural intervals, both neighbor-side
  admissibility checks, no-candidate retry, deterministic tie-breaking,
  proposal/accepted ticks, and owner preservation on rejection.
- An operand-lineage table and independent conservation reconstruction cover
  snow, liquid, vapor, energy, and event time without aliasing diagnostics.
- Dual science review and dual verification pass with no undispositioned
  findings; gate evidence is current and labeled `Static:` or `Ran:`.
- The final disposition authorizes only the later default-off implementation
  package. It does not activate or qualify production behavior.

## Security and data impact

Local repository contract, test, and documentation changes only. No network,
credentials, deployment, release, public data, or production selector changes
are authorized. Typed guards, rollback, immutable receipts, and exact-owner
custody remain security/correctness boundaries.

## Calibration readiness

This package is `science implementation / contract authority`, not calibration
or independent validation. `artifacts/calibration-readiness-matrix.md` must
record `NOT_APPLICABLE` for empirical calibration and `NOT_ASSESSED` or
`NOT_APPLICABLE` for identifiability with rationale; no execution grid may be
presented as a physical bound.

## Review and subagent authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent science-authority reviewers and two independent science
verification agents for the shared-carrier and event-coalescing scope. Expected
outputs are compact findings, exact command/count summaries, and artifact
paths. Reviewers and verifiers are read-only except their named package
artifacts. No implementation worker or comparator runner is authorized by this
authority scaffold.

## Line-count governance

Record touched Rust/test source line counts even though production Rust is
excluded. Repository policy is WARN at 2,000 lines and block at 3,000 unless a
generated/fixture exception includes owner and sunset.

## Progress

- [x] (2026-08-21) Confirmed `origin/main` release `1d0239f4a` and Child 2B
  authority.
- [x] (2026-08-21) Corrected the stale Child 2C handoff language.
- [x] (2026-08-21) Scaffolded the contract-first package and required gates.
- [ ] Amend canonical authority and derive vectors/tests.
- [ ] Complete dual science review and verification.
- [ ] Publish authority checkpoint and implementation handoff.

## Release boundary

`QUEUED / Child 2C authority only / default-off implementation not yet authorized`

## Outcomes and retrospective

The package is intentionally queued. The immediate handoff correction is
complete, and the next admissible work is contract-first authority authoring.
No production behavior claim is made by this scaffold.
