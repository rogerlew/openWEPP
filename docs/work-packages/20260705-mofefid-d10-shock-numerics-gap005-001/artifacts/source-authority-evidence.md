# Source Authority Evidence

Status: executed-hold
Evidence mode: Static

Source audit summary, with copyrighted sources summarized only:

| Source | D10 read | D10 use |
|---|---|---|
| Papanicolaou 2018 (R-63) | Local Markdown + PDF text around eqs. (8)-(14), Case 4, and Figure 4 context. | Defines the enhanced-WEPP framework and D-val surface; limiter branch text remains insufficient to authorize a source correction by itself. |
| Iwagaki 1955 (R-74) | PDF text for Case 4 flume geometry, slopes, lateral supplies, and Manning `n`. | Primary Case-4 setup authority; exposes the unresolved Manning-`n` to D-val `k_o` mapping. |
| Garcia-Navarro 1992 (R-81) | PDF text for TVD open-channel shock-capture method. | Primary TVD-family authority; not a complete reduced Papanicolaou KWE/OFE handoff prescription. |
| Mingham 2001 (R-82) | PDF text for TVD-MacCormack shock handling and CFL posture. | Primary TVD-family authority; not enough to choose a production correction in this code. |
| Current Rust solver/cascade | Read-only inspection of `kinematic_wave.rs`, `cascade.rs`, `dval.rs`, and `dval_case.rs`. | Confirms the defect lies in the Lane D numerical-method/handoff family, but a source-backed correction rule is not present. |

Rejected source-authority interpretations:

- The standard TVD limiter family cannot be substituted directly for the R-63
  extracted limiter without a contract reconciliation; a local branch flip
  regressed Case 4 and focused tests.
- Iwagaki `n=0.009` cannot be silently converted into a D-val `k_o` default in
  D10; that would cross into D11 friction operand authority.
- H2637 shadow diagnostics cannot replace Case-4 source acceptance because the
  shadow path is diagnostics-only and non-default.

Conclusion: source primaries are acquired, but source authority for production
correction remains incomplete.
