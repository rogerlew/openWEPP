# 2. Principles

The decisions in this chapter are the ones most likely to surprise a developer
arriving from a legacy modeling codebase. None of them is a style preference;
each is a response to a specific, recorded failure mode. The ADR links are the
normative versions.

## 2.1 The legacy binary is not the oracle

The instinctive way to rewrite a trusted model is characterization testing:
pin the old output, match it bit for bit, refactor underneath. openWEPP
explicitly rejects that as its acceptance model, for a blunt reason: **the
legacy binary is not trustworthy enough to be an oracle.** Thirty-plus years
of legacy WEPP development left known defects, routines disabled to work
around bugs, dead code paths (whole energy terms compiled but zeroed), and
behavior that exists only because two bugs cancel. Matching all of that
faithfully would enshrine the defects; matching it selectively requires an
authority that says *which* behavior is correct — and once you have that
authority, it, not the binary, is the oracle.

That authority is the **science contract** (`SC-<DOMAIN>-<NNN>` documents in
`docs/specifications/science-contracts/`): governing equations from the WEPP
technical documentation and literature, plus named invariants (`INV-*`) with
explicit tolerances. Kernels are accepted against contracts, and every
implementation — ported, rewritten, or agent-authored — passes the same gate
([ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md)).

The legacy binary still earns its keep, demoted to a **comparator flag**: a
divergence between openWEPP and legacy output is a *signal to investigate*,
routed by confidence tier (a single-OFE daily water-balance delta is a strong
signal; an hourly or watershed-level delta is only an investigation trigger).
It is never an acceptance gate
([ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)).
The project learned this the hard way: a comparator arc once spent sixteen
work packages chasing "openWEPP defects" that turned out to be
unit-mismatched comparison cut-points (snow *depth* compared against snow
*water-equivalent*, a suspiciously round 10× delta). The standing rule from
that arc: prove the two sides of a comparison are like-for-like before calling
a divergence a defect.

Related: **parity is semantic, not bitwise**
([ADR-0003](../decisions/0003-parity-semantic-not-bit.md)). Where openWEPP is
compared against anything, acceptance is agreement within a contract-named
tolerance, not bit identity. (Bit identity does appear in one different role —
protecting *openWEPP's own* outputs against unintended change during
refactors; see "protected outputs" in chapter 5.)

## 2.2 Architecture first, physics second

openWEPP builds the state model, module boundaries, orchestration, and
conservation gates *before* filling in process physics. This ordering looks
backwards from a science-first viewpoint, but it is what makes defects
localizable. In legacy WEPP, state crosses routines through `COMMON` blocks —
global memory any routine may read or write — so a wrong number surfaces far
from its cause, and the call order in the source is load-bearing in ways the
source never states. openWEPP inverts this:

- **Typed state.** Quantities carry identity and units in the type system
  (`openwepp-unit-boundary`, the units registry in `openwepp-sim-contract`).
  Mixing meters with millimeters, or a depth with a flux, fails to compile.
  WEPP's canonical variable names are kept as the authoritative vocabulary,
  with explicit aliases where an internal name differs — the contract text and
  the code speak the same language.
- **Explicit boundaries.** Each process phase declares inputs and outputs;
  ownership is enforced by the borrow checker. "What overwrote my variable?"
  is a compile error, not a debugging season.
- **Deterministic orchestration.** The phase order is an explicit, validated
  plan (chapter 3), identical on every run and machine. Numerics policy
  (summation order, RNG, float handling) is written down in
  [docs/numerics/](../numerics/).
- **No silent failure.** A `NaN`, a negative storage, an out-of-range flux
  becomes a typed guard error naming the field and the day it occurred —
  surfacing at the timestep it happened, not thousands of steps downstream.

## 2.3 Closure before magnitude

The roadmap ordering principle ([ROADMAP.md](../ROADMAP.md)): every rung of
model capability is accepted on **closure** — does the water/mass balance
conserve within tolerance, do the bounds hold — *before* anyone argues about
whether the magnitudes are physically right, and each rung adds **one
mechanism** on an already-closed foundation.

The reason is diagnostic, not aesthetic. Conservation is independent of
magnitude: a model can be beautifully calibrated and non-conserving, or
conserving with a magnitude bias. If you chase magnitude first, structural
errors (a leak in the balance) and physics errors (a wrong parameter) alias
into one another and every investigation confounds them. Closing the structure
first means that when a magnitude question is finally judged, a divergence can
no longer be a bookkeeping artifact — it is physics, judged against external
authority (published values, controlled experiments), not against the legacy
binary.

In day-to-day terms: closure checks run *inside* the engine on every run
(chapter 3's reconciliation and closure phases), and a conservation violation
is a hard, located error — `lane_index`, `day_index`, field — not a warning.

## 2.4 Non-clean-room, with provenance

openWEPP developers and agents **read the legacy Fortran source** for static
analysis and provenance mapping
([ADR-0010](../decisions/0010-non-clean-room-direct-port-policy.md)); the
pinned baseline is `wepp-forest_260430`
([ADR-0012](../decisions/0012-legacy-wepp-260430-baseline-anchor.md)). The
legal basis is that legacy WEPP is public domain (17 U.S.C. § 105); the
engineering basis is that reading the source is how you discover what the
model *actually does* — including the boundary conditions and workarounds no
paper ever recorded. What is ported is the governing equation and its
documented intent; what is *not* inherited is the legacy state model, control
flow, or defect surface.

## 2.5 Truthfulness about work performed

The project's review culture runs on evidence-class labeling, and it applies
to humans exactly as to agents. A claim states whether it is **static** (read
the source and reasoned) or **ran** (the command was executed, output in
hand). "I tested X" without having run X is treated as the serious failure it
is, because every downstream decision compounds on it. The same register
discipline applies to conclusions: a hypothesis presented as a finding is the
competence version of the same failure mode. See the Truthfulness sections of
[CLAUDE.md](../../CLAUDE.md) and [AGENTS.md](../../AGENTS.md); the work-package
convention (chapter 7) exists largely to make this auditable — every closed
package carries its evidence, including the dead ends.

## 2.6 Determinism is a feature, not an accident

Identical inputs must produce identical outputs on any machine, and run-to-run
diffs must mean something. This is why the codebase bans nondeterminism at
some surprising altitudes: iteration over unordered maps in anything
output-bearing, wall-clock or RNG access inside kernels, floating-point
reassociation in accumulations that cross a contract surface. The payoff is
operational: byte-identity of protected outputs is usable as a regression
gate for behavior-preserving refactors, and a divergence between two runs is
always a real difference, never noise.
