# Independent terminal verification A

Static: root/work-package governance, verifier procedure, testing strategy
sections 9-10 and 18, package/resume authorization, frozen protocol, authoritative
worker handoff, source/build manifest, treatment results, architecture handoff,
gate results, both terminal reviews and focused re-reviews, finding disposition,
final disposition, compact results, source manifests, timing freezes, and the six
durable result streams. Ran: read-only hashing, JSON reduction, package analyzer,
collector unit suite, Git identity/status, and source/result identity searches. No
simulation, build, full-workspace gate, or comparative measurement was rerun.

Role/session: terminal verifier A; configured/requested effort: inherited;
effective: UNOBSERVED. Reviewed identity: uncommitted stable substantive HOLD cut
at repository HEAD `e7c1850ddb84c33809228a4f6b557e10c4b0d985`; controlling publication hashes
include package `82206cf8...c9a608`, worker handoff `a7ecfe27...1f906`, final
disposition `5dc4fb6c...6e499`, treatment results `318bc250...0c36`, architecture
handoff `56c4e8ba...c4d0`, and source/build manifest `f2790c46...f1e32`.
Assigned scope: authority/instruction/claim legitimacy, exact identities, durable
evidence hashes, F/R statistics, collector suite, scale non-deferral, and
architecture handoff. Any later substantive change to these inputs ends this
verification's freshness.

## Independent checks

- Ran from `/workdir/openWEPP`: `tools/agents/find-agents --for
  docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/verification_agent_a.md`,
  exit 0. The applicable chain is root plus `docs/work-packages/AGENTS.md`.
- Ran from `artifacts/reproduction`:
  `/workdir/openWEPP/.venv/bin/python -m unittest test_collectors.py`, exit 0,
  PASS 8/8. This independently accepts the current collector's identity,
  statistics, lifecycle, stencil, and poison controls.
- Ran `sha256sum` on the six retained streams, two timing freezes, three source
  manifests, and authority composition manifest, exit 0. Stream hashes are:
  A05/F06 timing `313d3ceb...e83e91c`, A/F memory
  `208eb2f6...664b2c`, A/F teardown `c3341d4d...6e2d4ced`, A04/R05 timing
  `9aee044a...884a582`, A/R memory `5e5e7465...de4453d0`, and A/R teardown
  `fa43fad4...e946e129`. Timing-freeze hashes are
  `bfc7b0ee...6ca11f` and `dd6d640a...436e2e4`. Source-manifest hashes
  `aa1abb4b...fc1b`, `d9fa2541...ecd4`, and `275da2b1...9ecc` and
  composition hash `0a03c8e5...04b42` match the source/build manifest.
- Ran a read-only Python inventory/reduction over all six streams, exit 0: row
  counts/valid counts are timing 28/28 per comparison, memory 12/12 per
  comparison, and teardown 6/6 per comparison. Each timing stream contains four
  warmup processes and 12 measured pairs. The first ad-hoc reduction included
  warmups and therefore was not used for claims. The corrected package-analyzer
  invocation (`analyze_series.summarize` over parsed rows) exited 0 and reproduced
  F median improvement 22.4384%, seconds saved 1.1207305 s, 95% intervals
  22.1009-22.5831% and 1.101593-1.1280925 s, CPU ratio 0.776073; and R median
  regression 5.7241%, seconds saved -0.2840685 s, corresponding intervals
  5.4495-6.0343% regression and -0.2982505--0.2707690 s, CPU ratio 1.056720.
  An initial analyzer call passed a `Path` rather than parsed rows and exited 1
  with `TypeError`; it produced no evidence or changes and was corrected once.
- Static + Ran identity check: final F timing binds A05 source
  `e9899b0e...ff2b4`/binary `43b64b3b...28a` and F06 source
  `290a346d...5f99`/binary `71cee13d...d45`. R timing binds A04 source
  `98ee7baf...4ae`/binary `22cdcd94...8b5f` and R05 source
  `28c3310d...5027`/binary `5a7619bb...9a8`. A/F lifecycle streams retain the
  preceding A04/F cut (`22cdcd94...8b5f` and `fc4cace8...16db`), not the final
  scale-assertion binaries. Reuse is bounded by the recorded dependency proof
  that the A05/F06 change only corrects OFE>1 assertion validation; no lifecycle
  or one-OFE treatment claim is silently represented as a same-binary rerun.
- Static non-deferral check: F passed the one-OFE competitive predicate, making
  authentic 10/19-OFE scale evidence mandatory. The attempted 10-OFE fixture
  produced provider=carrier=0 and D=5330 in both arms, so it does not enter the
  reviewed native path; 19 OFE is NOT RUN. Final disposition correctly classifies
  this as an unmet blocker, names the next authorized Stage3 prototype package as
  owner and an authentic contract-reviewed multi-OFE fixture as trigger, withholds
  standalone/scaling claims, and remains `EXECUTED HOLD`, not COMPLETE.
- Static architecture check: the handoff selects a contract-first component-
  temperature derivative/residual block at `covered_jacobian_probe_residuals`,
  requires new LSE numerical-method authority, preserves domains, active sets,
  residual/error/conservation/custody/restart/publication invariants, uses R's
  lawful finite-difference oracle, and sets explicit 10x block-work/30% end-to-end
  success and correctness-first kill criteria. It does not authorize successor
  implementation or infer scaling from the one-OFE F result.
- Static gate check: optimized workspace correctness, broad Clippy, release
  golden, and post-return memory allowance remain recorded FAIL; scale remains
  BLOCKED/NOT RUN as applicable. These failures are not converted into passes and
  production remains HOLD. Review A's corrected-cut verdict passes the bounded
  HOLD disposition while withholding COMPLETE; review B's second focused review
  has no remaining blocker to verification of that same bounded disposition.

## Findings and disposition

No new blocker to the bounded `EXECUTED HOLD` claim. Identity precision note,
accepted for this HOLD cut: `source-and-build-manifest.json` names the final
A05/F06/R05 binaries but does not itself enumerate the A04 baseline or preceding
F lifecycle binary and does not list the eight durable result/freeze hashes.
The authoritative handoff plus retained freezes/rows and this independent hash
record make the bounded evidence cut reproducible, but a future COMPLETE,
publication, or stronger unified-manifest claim should consolidate those
members rather than imply every campaign used the final timing executables.

Uncertainty: effective agent runtime settings are unobserved; expensive workflows
were not rerun; local hashes are custody evidence, not remote trust; the repository
is dirty and the package raw/source artifacts are currently untracked. This
verification accepts only the exact local content-addressed HOLD cut described
above and creates no production, release, scale, standalone-F, or long-run-memory
qualification.

Verdict: **PASS for a stable, bounded `EXECUTED HOLD` disposition; explicitly
not PASS for COMPLETE.** F is supported only as a one-OFE useful architecture
building block, R is rejected as a standalone runtime treatment, the mandatory
F scale obligation remains unmet with named follow-on ownership, inherited gate
failures remain visible, and the selected successor direction is an
authorization-ready handoff rather than an implementation authorization.
