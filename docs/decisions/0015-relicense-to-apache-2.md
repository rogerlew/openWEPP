# ADR-0015: Relicense openWEPP to Apache-2.0 (supersedes ADR-0001)

**Status:** Accepted
**Date:** 2026-05-28
**Deciders:** Roger Lew, Claude Code
**Supersedes:** ADR-0001

## Context

ADR-0001 selected CC0-1.0 for openWEPP, citing alignment with the
wepp-palimpsest CC0 posture and preservation of the USDA public-domain
provenance for legacy WEPP material.

Three problems with CC0-1.0 for openWEPP have since been recognized:

1. **No explicit patent grant.** Apache-2.0 §3 grants an irrevocable patent
   license from contributors covering claims they own that read on the
   contribution. CC0-1.0 §4(a) explicitly *excludes* patent and trademark
   rights from the dedication. For a scientific model with embedded
   numerical methods, missing patent coverage is a real downstream risk
   that CC0 silently leaves on the floor.
2. **Not OSI-approved.** CC0-1.0 is not on the OSI-approved list. Some
   institutional and corporate procurement policies reject anything that
   is not on that list, regardless of how permissive it actually is.
   Adopting CC0 unnecessarily restricts downstream institutional adoption.
3. **CC was not designed for software.** Creative Commons itself
   recommends against using CC licenses for software. CC license text does
   not address source-vs-binary distribution, contribution mechanics, or
   trademark separation in the way software-purpose licenses (Apache-2.0,
   MIT, BSD) do.

Apache-2.0 retains every practical permission CC0-1.0 grants for users
(commercial use, modification, redistribution, sublicensing under
compatible terms), adds the explicit patent grant, is OSI-approved, and
is the dominant license in the Rust ecosystem alongside MIT.

The USDA legacy WEPP provenance argument from ADR-0001 is unaffected by
this change: federal-employee authorship in legacy WEPP is in the US
public domain under 17 USC §105 independent of openWEPP's license choice.
That status is now recorded in `NOTICE` per Apache-2.0 attribution
conventions.

openWEPP is at this point a single-contributor codebase (Roger Lew, 100%
copyright on first-party material). Unilateral relicensing is therefore
legally clean and requires no contributor-consent process. The repository
is still private on GitHub; no prior CC0-1.0 distribution has occurred
that would constrain the change.

## Decision

openWEPP is licensed Apache-2.0, retroactively from the first commit. SPDX
identifier `Apache-2.0` is used in per-crate `Cargo.toml` metadata and in
per-file headers per the legacy-source-attribution-and-contributors-policy
governance document.

A `NOTICE` file at the repository root records the USDA WEPP derivation
and points to `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
for the pinned baseline anchor and to the governance policy for per-file
authorship continuity.

Dependency license posture is unchanged: `cargo deny` denies viral
copyleft (GPL / AGPL / LGPL) by allow-list exclusion. MPL-2.0 (weak
per-file copyleft) remains excluded by default; specific MPL-2.0
dependencies may be added with documented justification once a real need
arises. CC0-1.0 remains in the dependency allow list for upstream
compatibility (some crates ship CC0-1.0 data assets).

The prior CC0-1.0 disposition recorded in ADR-0001 remains as historical
record. Distribution events that occurred under CC0-1.0 (if any future
discovery surfaces them) remain effective under CC0-1.0 with respect to
the artifacts then distributed; the present authoritative grant for the
codebase is Apache-2.0.

## Consequences

- Explicit patent grant from any future contributor flows downstream.
  This is the substantive new protection.
- OSI-approved status removes a procurement-checklist friction for
  institutional adopters.
- Apache-2.0 `NOTICE` mechanics require contributors who incorporate
  upstream NOTICE-bearing material to preserve those notices. This is a
  minor operational overhead absent under CC0.
- Per-file SPDX headers in legacy-source-attribution policy update from
  `CC0-1.0` to `Apache-2.0`; the policy's `Origin-Class`,
  `Original-Author(s)`, and `Contributors` fields are unchanged.
- The wepp-palimpsest CC0 posture from ADR-0001 is no longer a parity
  constraint for openWEPP. openWEPP and wepp-palimpsest may diverge on
  first-party license while remaining compatible at the use-and-modify
  level (CC0 inbound to Apache-2.0 outbound is unambiguous; the reverse
  is not, but openWEPP does not flow back into wepp-palimpsest).
- ADR-0001 is marked `Superseded by 0015` in the ADR index.
