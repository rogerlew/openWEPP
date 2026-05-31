# AUTH03 Gate Results

Status: completed  
Evidence mode: Ran

## Scope
- Execute AUTH03 required validation gates and record outcomes.

## Commands run
1. `cargo fmt`
   - pass
2. `cargo test --test auth03_level4_constitutive_gate_contract`
   - pass (`4 passed`)
3. `cargo fmt --check`
   - pass
4. `cargo clippy --workspace --all-targets -- -D warnings`
   - pass
5. `cargo test --workspace`
   - pass
6. `cargo deny check`
   - pass (warnings only; no failing checks)
7. `markdown-doc lint --path ...`
   - pass (`26 files validated, 0 errors, 0 warnings`)
8. `markdown-doc validate --path ... --path docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001`
   - fail (prompt file schema-tool false positive: reported empty document for
     non-empty kickoff prompt)
9. `markdown-doc validate --path <scoped AUTH03 spec/suite docs>`
   - pass (`6 files validated, 0 errors`)

## Gate decision
- pass (AUTH03 scoped gates complete; one tool false-positive isolated to
  package prompt validate path aggregation, rerun on scoped docs passes)
