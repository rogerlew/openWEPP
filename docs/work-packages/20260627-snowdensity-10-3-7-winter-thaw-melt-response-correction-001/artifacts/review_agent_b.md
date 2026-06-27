# Review B

Evidence mode: Static.

Scope reviewed:

- Selector/parser boundary.
- Albedo isolation.
- Event-window improvement evidence.
- Conservation/routing evidence.
- Coupled WAT evidence.
- Package exit criteria.

Findings:

- No blocking QA findings.
- `CoeWinterThawStateLossV1` does not seed or require Brock albedo state.
- Snowbench exposes the selector only through the existing diagnostic `--model`
  surface; no parser/runfile/user activation was added.
- Focused test proves the candidate routes positive low-density thaw melt to
  SWE state loss while default `legacy_coe` retains the legacy gate behavior.
- The closure gate is met because both under-ablation count and aggregate
  depth-loss deficit improve, active-ledger conservation passes, and coupled WAT
  snow-control improves without paired-surface worsening.

Residual risk:

- Candidate thaw-ablation windows are not exactly like-for-like in count
  (`219` legacy vs `218` candidate) because modeled snow-depth trajectory changes
  one event classification. The aggregate improvement remains material and the
  package closure rule is still satisfied.
- Coupled snow-control remains failed (`978/1415`), so this is not a default
  activation or frost-unblock package.
