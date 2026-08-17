# Review Agent B Terminal Closure 5 — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `cf6acd2f5aaf0b131cba5de77a83b186f29702a7`

Verdict: `HOLD / chronological independent-closure authority is incomplete / no new authority package indicated`.

This review preserves all earlier review and HOLD artifacts. It confirms that
the production ingress calculation has restored the canonical common
subinterval mixture, then audits the independent operand reconstruction against
the complete admitted timed-parcel domain rather than only the full-interval
fixtures.

## Material findings

### B-TERMINAL-CLOSURE5-CRITICAL-001 — independent closure replaces chronological subinterval mixtures with one whole-OFE mixture

The production path now correctly partitions every source support into exact
chronological boundaries and computes one `h_mix = Q_b / X_b` for each active
subinterval (`surface_liquid_ingress.rs:1206-1267` and `:1333-1358`). It assigns
that same temperature and enthalpy basis to every attributed infiltration,
retention and runoff child in that subinterval. This closes the prior
source-specific-temperature regression.

The independent validator does not reconstruct that algorithm. It first
aggregates each source over the whole interval by
`(source_parcel_id,basis_ofe_id)` (`surface_liquid_closure.rs:936-970`), then
sums all sources over the whole OFE (`:1058-1075`), computes one OFE-wide
`h_mix` (`:1076-1098`), and rewrites every source's expected enthalpy with that
single value (`:1099-1114`). Source support is not used anywhere in this
projection.

That is not equivalent to the canonical Section 6 rule:

```text
for each exact chronological subinterval b:
    X_b = sum_p(x_p,b)
    Q_b = sum_p(q_p,b)
    h_mix,b = Q_b / X_b
```

For example, two open tiles in one OFE may legitimately supply precipitation
on disjoint supports at unequal temperatures. Production assigns the first
subinterval its first source temperature and the second subinterval its second
source temperature. The independent projection instead expects both source
rows at the mass-weighted whole-OFE temperature. The correctly produced public
candidate therefore fails E010. With partly overlapping supports, the same
projection cannot prove which source mass was mixed under each accepted
`h_mix,b`, so it can also compare against the wrong post-mix source operands.

This is a terminal material defect even though the constitutive producer is
now correct: timed raw precipitation and routed carry are explicitly inside the
admitted domain, and the public candidate cannot be accepted without this
independent validation.

Required correction:

1. Independently partition frozen source operands at every exact support
   boundary for each current basis OFE.
2. Reconstruct `x_p,b`, `q_p,b`, `X_b`, `Q_b`, and `h_mix,b` for each
   subinterval, including the exact `X_b=0 => h_mix,b=0` branch.
3. Accumulate each source/basis expected post-mix mass and enthalpy from those
   subinterval rows, while separately proving raw `Q_b` equals the complete
   post-mix output energy.
4. Use the contract's final-child mass and enthalpy remainder arithmetic so
   `Q_excess,b = Q_b - Q_infiltration,b` is constructed rather than inferred
   only through repeated `mass * h_mix` products.
5. Add positive and poison vectors with disjoint and partially overlapping
   unequal-temperature sources, including unequal source durations and routed
   downstream carry.

### B-TERMINAL-CLOSURE5-HIGH-002 — frozen source identity discards admitted precipitation support and imposes noncanonical input order

`capture_amount()` copies mass and enthalpy from the immutable input but writes
every source operand as `[0, INTERVAL_S)` at
`surface_liquid_closure.rs:763-800`. This contradicts the production source,
which preserves `amount.start_s` and `amount.end_s` at
`surface_liquid_ingress.rs:1129-1130`, and the contract requirement that raw
precipitation and routed parcels retain exact support.

The supposedly independent frozen-identity reconstruction repeats the same
hard-coded full interval at `surface_liquid_closure.rs:851-894`. It derives
local identities from configuration rather than the immutable ingress input,
then compares vectors in exact order at `:896-903`. The ingress validator,
however, accepts the complete tile set in caller order and the producer sorts
timed parcels canonically before execution. Thus:

- a valid partial-support raw-precipitation source is frozen with false support;
- support mutation cannot be detected against the actual immutable input; and
- a semantically identical reordering of valid tile ingress can be rejected
  solely because configuration order differs from caller order.

Required correction: reconstruct complete local frozen identities from the
actual validated immutable ingress records, preserving exact start/end bits;
add resource-derived overflow identities; sort both expected and actual
identities by the admitted canonical parcel ordering before cardinality/set
comparison. Retain all exact-zero members. Add partial-support, reordered-input,
support-rekey, deletion, duplicate and zero-source controls.

## Confirmed closure from the latest remediation

- Production uses one common attributed temperature per active subinterval;
  source kind no longer selects infiltration, retained-water or runoff
  temperature.
- Raw source mass and enthalpy remain frozen separately from post-mix receipt
  operands. Exact-zero source rows participate in frozen identity/cardinality
  validation, including deletion, duplicate, re-key and kind-swap controls.
- OFE aggregate arithmetic failures now retain owner/OFE identity and typed
  absence for tile, surface and source rather than fabricating the first tile.
- Per-key actual/expected maps use the complete union, and routed rows retain
  current basis-OFE identity. The remaining defect is the temporal meaning of
  the expected values, not a return to origin-only routing identity.
- Surface-water D/A/F remains exact and typed; only finalized withdrawal debits
  the hydrology-owned store. Condensation remains a signed credit before
  capacity overflow.
- WB14 continuation, ingress routing, production-soil and soil-thermal
  receivers, retained-LSE receipts, restart lineage and rollback architecture
  remain present. No additional production selector, default, scheduler,
  publication or runtime activation path was introduced.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 47/47 selected; 507 skipped by filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

These gates cover simultaneous full-interval mixed sources, exact-zero frozen
identity and aggregate context. They contain no disjoint-support or
partly-overlapping unequal-temperature vector and therefore do not exercise the
chronological independent reconstruction defect.

## Approval statement

`NO-GO`: exact commit `cf6acd2f5` restores the canonical constitutive
well-mixed subinterval calculation and closes the zero-source and typed-context
findings, but its independent validator reconstructs a different whole-OFE
mixing algorithm and its frozen operands erase admitted raw-precipitation
support. Correct the independent temporal projection and exact source-support
freeze, add nondegenerate timed-source vectors, and rerun closure, receiver,
rollback and restart gates before another exact-byte terminal review. The
existing SC-SURFACELIQUID-001 authority is sufficient; no new contract or model
identity is required.
