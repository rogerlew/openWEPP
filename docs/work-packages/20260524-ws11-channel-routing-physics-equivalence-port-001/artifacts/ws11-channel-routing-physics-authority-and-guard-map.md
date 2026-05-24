# WS11 Channel-Routing Physics Authority and Guard Map

Status: `completed`
Evidence mode: `Static`

## Static
- Canonical authority surfaces
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
  - `contract_version: 10`
  - WS11 canonical routing authority with explicit `ipeak` branch semantics,
    routed closure constraints, pinned baseline provenance anchors, and
    deauthorization of gain-factor surrogate routing.
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
  - `contract_version: 11`
  - WS11 consumer-coupling authority requiring channel payload consumers to
    preserve `ipeak` provenance and reject surrogate substitutions.
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `contract_version: 14`
  - WS11 system-integration authority for explicit `ipeak` execution branches
    and deterministic guarded boundary publication.
- WS11 surrogate deauthorization
  - WS11 explicitly deauthorizes the pre-WS11 channel routing surrogate:
    `(1 + ctlslp) / (1 + chnn)`.
  - Canonical parity authority is branch-selected channel routing from
    `ipeak` (`1` Rational, `2` CREAMS, `3` kinematic wave, `>=4`
    Muskingum-Cunge), not a single gain factor.
- Runtime authority mapping
  - Branch selector authority:
    - `SC-ROUTE-001` `INV-ROUTE-006`
    - `SC-SYSTEM-001` `INV-SYSTEM-006`
    - runtime meaning: exactly one explicit `ipeak` branch per evaluation.
  - Threshold/routed closure authority:
    - `SC-ROUTE-001` `INV-ROUTE-007`
    - `SC-SYSTEM-001` `INV-SYSTEM-005`
    - runtime meaning: `ipeak <= 2` zero-threshold behavior vs `ipeak >= 3`
      routed-flow closure when local runoff is absent.
  - Consumer-coupling authority:
    - `SC-HYDRAULICS-001` WS11 addendum
    - runtime meaning: downstream consumers preserve emitted route provenance
      and do not reconstruct surrogate peak paths.
- Legacy provenance anchors (pinned baseline)
  - `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
  - baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Guard map
  - Missing required WS11 channel/routing symbol:
    `WKERNEL-WS10-CHANNEL-E-001`
  - Non-finite WS11 channel/routing symbol or intermediate:
    `WKERNEL-WS10-CHANNEL-E-002`
  - Domain/closure/branch WS11 routing violation:
    `WKERNEL-WS10-CHANNEL-E-003`
- Cross-contract registry updates
  - `docs/specifications/science-contracts/index.md` entries for
    `SC-ROUTE-001`, `SC-HYDRAULICS-001`, and `SC-SYSTEM-001` now encode WS11
    authority posture and preserve WS12 context.
- Open governance note
  - `SC-ROUTE-001` keeps `GAP-ROUTE-005` open (non-promotable posture) and
    records `GAP-ROUTE-006` as `promotable-with-risk` lineage documentation
    follow-up.

## Ran
- Not run (authority/guard map artifact from static contract text only).
