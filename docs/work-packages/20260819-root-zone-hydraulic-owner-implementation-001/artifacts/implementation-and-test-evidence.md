# Implementation And Test Evidence

Evidence class: `Ran + Static`.

Reviewed implementation commit:
`3ea08d81d966ccbf163ee64377aa741308e2665a`.

The V10 scheduler now constructs opaque per-root hydraulic receipts from the
current hydrology frame and immutable root-zone configuration on every
interval. Brooks--Corey retention and conductivity use pinned `libm 0.2.16`;
layer top, signed gravity, root-tissue path, accessibility and frozen posture
are owner-derived. Positive rooted layers require receipts. Zero-root layers
are inactive and cannot consume caller physics.

Ground VIS/NIR optics are projected from LSE tile configuration and upward
longwave from the current LSE tile temperature. Unowned external runon rejects
before candidate mutation. Atmospheric and precipitation values remain sealed
provider authority. Caller hydraulic template values are non-authoritative and
ignored; poisoned optics and longwave are overwritten by exact tile-qualified
owner projections.

Restart V1 wire bytes and phase union are unchanged. The immutable root-zone
configuration is supplied through `ExpectedRestartStaticContext`; restored
continuation templates are checked against reprojected live owners. The
released reference implementation and production restart fixtures use the
same static context and retained exact-byte continuation path.

Ran on the exact implementation tree:

- orchestrator all-features Nextest: 722/722 PASS;
- persisted restart all-features Nextest: 30/30 PASS;
- released restart reference: 28/28 PASS;
- focused V10 owner suite: 9/9 PASS;
- focused authority/integration targets: 31/31 PASS;
- authority anti-evasion: PASS;
- AUTH11 obligation guards: 3/3 PASS;
- affected warnings-denied Clippy: PASS;
- exact-head workspace Nextest: 3,087/3,087 PASS, 33 canonically skipped,
  including environment-corrected reruns of the three initially affected
  harness cases;
- workspace doctests: PASS;
- cargo-deny: advisories/bans/licenses/sources PASS (one unmatched allowance
  warning only);
- rustfmt and diff hygiene: PASS.

The exact-current comparator ran seven benchmark surfaces and thirteen Child-4
selectors: 20/20 PASS. Its logs and manifest are retained in the Child-4
package under `artifacts/comparator-heavy/20260820T003717Z-child4-root-owner-final/`.

The scheduler attachment remains explicit and default-off. No runner,
selector, default, publication, output or cutover path changed.
