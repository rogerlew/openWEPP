# GAP-SNOWFREEZE-002 Step 2 Disposition

Evidence mode: Static + Ran.

`GAP-SNOWFREEZE-002` remains open and is narrowed. This package performed
diagnostic attribution only; it did not change frost physics, snow physics, or
contract authority.

## Result

Frost timing does **not** fully agree at the two Step 1-unblocked Sleepers
sites. Both sites have timing residuals beyond `+/-14 days` whose direction is
not explained by the Step 1 snow residual direction.

| Site | Timing Result | Magnitude Result | Step 3 Pointer |
| --- | --- | --- | --- |
| `site1_sleepers_south_field_vt` | `4` candidate frost-model timing cells, all thaw-late; `7` forcing-attributable failures; `85/96` pass | `FORCING-LIMITED-MIXED-SIGN`; not verdict-bearing | Residue-lifecycle handoff; compare against legacy-envelope outlier flag |
| `site2_sleepers_w9_hardwood_vt` | `14` candidate frost-model timing cells, early-onset and thaw-late; `8` forcing-attributable failures; `49/75` pass | `FORCING-LIMITED-SIGN-INCOHERENT`; not verdict-bearing | Residue-lifecycle handoff / static-vs-dynamic `resdep`; compare against legacy-envelope outlier flag |

## Sign-Coherence Finding

Step 1 snow residuals are modeled-over-observed at both Sleepers sites. Deeper
modeled snow can explain shallower, later-onset, earlier-thaw, or
shorter-duration frost. It cannot explain early-onset, late-thaw, longer-
duration, or deeper-frost signs.

The candidate defects are timing failures with the unexplained signs:

- South Field: late-thaw cells in WY `1986`, `1987`, `2007`, and `2015`.
- W9 Hardwood: early-onset cells in WY `1994`, `1996`, `1997`, `1998`, and
  `2013`; late-thaw cells in WY `1994`, `1995`, `1996`, `1997`, `2004`,
  `2006`, `2009`, `2010`, and `2011`.

## Forward Route

Step 3 should investigate the residue-lifecycle handoff first. The legacy
envelope `13/43` outlier record remains comparator-flag context under ADR-0017,
not a target. The absent `Qwet` evaporative term remains a named candidate in
the strategy, but this Step 2 sign-coherence pass does not make it the primary
pointer.

Step 4 note: Step 1's `>0.25` systematic-timing-fraction cutoff is a
diagnostic-script-local constant. Only `TOLERANCE_DAYS = 14` is inherited here.
If `INV-SNOWFREEZE-048/050` ratification inherits that cutoff, adjudicate it
deliberately.
