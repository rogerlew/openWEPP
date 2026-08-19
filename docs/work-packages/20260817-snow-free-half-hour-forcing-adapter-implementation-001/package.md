# Implement Snow-Free Half-Hour Forcing Adapter

Status: `executing / physics PASS / stateful GSI, cursor and persisted restart integration active`

Date: `2026-08-17`

Package ID: `20260817-snow-free-half-hour-forcing-adapter-implementation-001`

Plan class: `Critical forcing-provider implementation`

## Objective

Implement a concrete, digest-bound repository provider producing 48 exact
half-hour V10/LSE-V2 receipts from actual climate/runtime inputs under
`OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1`, default-off only.

## Dependency and protected boundary

Production edits are forbidden until the authority package is COMPLETE. The
provider may refactor neutral SIMIMPL28 hourly mechanics without changing
winter-trigger behavior or existing output bytes. Callers may not inject
completed closure-eligible interval physics. Live hydrology, thermal,
surface-liquid, vegetation, and BGC state remain owner-read at each interval.

The authority dependency completed in local commit `8ffc49aed`; implementation
started only afterward. The V10/LSE-V2 consumer dependency now passes both
complete zero-radiation and realistic positive-radiation 48-interval days at
`8abc81f6f2e40b99ecccc4a975cb1fa7f0f915d6`.

## Intended write set

The climate/runtime-input boundary, meteorology primitives, Child-4 provider
adapter, contract-derived integration tests, package evidence, and necessary
manifests. No selector, default, production mutation/output, deployment,
calibration, PR, remote branch, or push.

## Exit criteria

Exact parent-hour parity, 24-to-48 support, radiation/precipitation closure,
breakpoint overlap, four-way shortwave and atmospheric longwave vectors,
pressure/humidity guards, digest poisons, explicit CO2, unsupported domains,
real-provider consumer evidence, fresh Rust/science review, and dual terminal
verification all pass. Close only as the requested default-off native provider.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only forcing-science and Rust reviewers and two read-only terminal
verifiers. Expected outputs are compact package-local artifacts.
