# V3/V5 Semantic Diff and Independent Equation Reconstruction

Evidence class: `Static + independently executed Python; no Rust oracle`

Status: `HOLD — equations and branches agree, exact historical byte producer is unresolved`

## Scope and method

The frozen V3/V5 JSON was compared recursively with output produced by the
byte-identical release/current calculators in isolated checkouts. Comparison
classified every leaf by JSON path and scalar type. The disputed families were
then reconstructed from their committed input operands using the equations and
operation order in `SC-VEGETATION-001`, without consulting Rust output.

The durable exhaustive audit is
[`complete-field-diff.tsv.md`](complete-field-diff.tsv.md). It records every
JSON path, frozen value, regenerated value, numeric delta, and branch
classification. The summary and authority conclusion therefore do not depend
on temporary execution files.

| Identity | Frozen SHA-256 | Regenerated SHA-256 | Differing leaves |
|---|---|---|---:|
| V3 | `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109` | `7e64d63729b538ff5721ded768eb62be4be195a7903464a2ac7a3ab2083bff00` | 167 |
| V5 | `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d` | `327b349cac6dcb4793c61f2d211f20c0140bd27cbc45f180b0f49816accc1eb2` | 526 |

All 693 differences are float-to-float. There are no differing keys, array
cardinalities, strings, booleans, integers, nullability, orderings, active-cap
sets, or branch labels. V3 has 20 and V5 has 23 sign changes, all in tiny
near-zero residual/closure evidence; neither identity has a zero/nonzero
transition. CPython 3.12.14 at the historically logged `.venv/bin/python` path
and CPython 3.14.4 produce the same divergent SHAs, so Python major/minor alone
does not explain the historical bytes.

## Field-level inventory

### V3

| JSON family | Count | Disputed field classes |
|---|---:|---|
| `accepted_uncapped_stage_a` | 73 | four solution scalars; `q2` and two positive `q3`; gas/energy states; six residual values and normalized forms; four closures; nine residual-history entries; request amounts; step, pivot, and matrix evidence |
| `alternate_warm_start.result` | 77 | the same physical/numerical classes plus canopy-air humidity and wet-potential evidence |
| `singular_jacobian.diagnostics` | 1 | matrix infinity norm only |
| ten named hydraulic poisons | 16 | accepted/rejected scalar expectations derived from the disputed potential solution |

Representative frozen-to-regenerated pairs are:

- `root_node_potential_mm`: `-4108.077521446589` to
  `-4108.0775214465875`;
- `beta_hyd_sun`: `0.3869356429939499` to `0.3869356429939492`;
- `q2`: `7.528263922531399e-06` to `7.528263922532101e-06`;
- `soil-1 q3`: `5.519758610300215e-06` to
  `5.519758610300213e-06`;
- `q1_sum_minus_q2`: `4.548566926755593e-19` to
  `-2.473336205982557e-19`;
- singular matrix norm: `5.2749636302073436e-05` to
  `5.274963596101292e-05`.

### V5

| JSON family | Count | Disputed field classes |
|---|---:|---|
| `fully_authorized_value_reduction` | 147 | inherited uncapped reference plus capped result, conversions, owner closure, histories, residuals, solution, and gas/energy detail |
| `accepted_constrained_all_cap` | 132 | five solution scalars; cap/law/final fluxes and conversions; gas/energy states; residuals, history, step, pivot, and matrix evidence |
| `alternate_warm_start` | 129 | four start scalars and 125 result fields of the same classes |
| `executed_failures` | 49 | singular, iteration-limit, backtracking, and operand diagnostic numerics |
| `capped_failures` | 49 | the corresponding independently exposed failure records |
| `executed_coupled_exact_tie_jacobian` | 16 | perturbation step and law/cap values around the tie |
| three named capped-pass poisons | 4 | scalar-ratio, sequential-clamp, and no-energy-resolve expectations |

Representative frozen-to-regenerated pairs are:

- constrained `beta_hyd`: `0.3607131351882922` to
  `0.36071313518831233`;
- constrained soil-1 cap rate: `3.0358672356651185e-06` to
  `3.0358672356651172e-06`;
- constrained soil-1 law flux changes while its branch remains
  `authorization_active_or_tie`;
- backtracking failure `step_norm`: `3925.853296952497` to
  `3925.8545901008915`;
- exact-tie branch strings and ordered active-cap identities do not change.

## Independent reconstruction by disputed equation family

### V3 constitutive hydraulics and continuity

For each trial vector
`[psi_sun,psi_shade,psi_stem,psi_root,beta_sun,beta_shade]`, reconstruction
used:

```text
q1c = (k1_max/z1) * A_c * v(psi_stem) * (psi_stem-psi_c)
q2  = (k2_max/height) * v(psi_root) * SAI
      * (psi_root-psi_stem-1000*height)
kr_i = (k3_max/z3_i) * v(psi_soil_i)
ks_i = Ksoil_i/dxroot_i
k3_i = kr_i*ks_i/(kr_i+ks_i)
RAI_i = (LAI+SAI)*root_fraction_i*root_to_leaf_area
q3_i = k3_i*RAI_i*(psi_soil_i-psi_root+gravity_i)
```

Dry, frozen, inaccessible, and zero-root branches reconstruct exact zero.
Using the frozen accepted operands gives positive layer fluxes
`5.519758610300215e-06` and `2.008505312231633e-06`; their `math.fsum` closes
against `q2=7.528263922531399e-06` to `-4.497744949920335e-19` in the frozen
serialization. The regenerated solution changes only terminal binary64 values,
not the equations or zero-layer branches.

The six reconstructed residual identities are the separate sun/shade
gas-minus-`q1`, gas-minus-`Emax*v(psi_leaf)`, `q1sun+q1shade-q2`, and
`sum(q3)-q2` equations. The accepted solution and alternate-start solution both
satisfy the shared threshold
`1e-12 + 1e-9*max(1e-12,Emax_sun,Emax_shade,abs(q1sun),abs(q1shade),abs(q2),abs(q3_i))`.
The frozen and regenerated near-zero sign changes therefore do not denote a
different physical direction or branch.

### V3 nonlinear algorithm, diagnostics, and derived poisons

Reconstruction used centered perturbations
`sqrt(binary64_epsilon)*max(abs(x_j),unit_scale_j)`, with `1000 mm` for each
potential and `1` for each beta; pivoted LU; strict decrease of normalized
infinity norm; at most 20 halvings and 50 steps; and accepted potential step
at most `1e-7 mm`. Matrix norms, pivots, histories, terminal steps, and poison
scalars are trajectory-derived rather than separate constitutive families.
Their changes follow from the few-ULP terminal/trajectory change. The singular
case retains iteration zero, pivot zero, typed `singular_jacobian`, and no
candidate/last iterate; only its recomputed matrix norm differs.

### V5 cap conversion, complementarity, and owner debit

Every disputed conversion was reconstructed in the required order:

```text
A_tile_i   = A_W_i/f_t
cap_rate_i = A_W_i/(f_t*dt)
q_i        = min(q_law_i,cap_rate_i) after q_law_i is evaluated
F_W_i      = f_t*q_i*dt
```

For the frozen constrained case, soil-1 has
`q_law=7.496646485507026e-06` and
`cap_rate=q=3.0358672356651185e-06`; soil-2 has
`q_law=2.6900533981312626e-06` and
`cap_rate=q=2.008505312231633e-06`. Both are cap-active. Dry, frozen, and
zero-root layers have exact-zero law, cap, and final flux and select the
active-or-tie branch by exact `cap_rate<=q_law`. The regenerated values preserve
all these inequalities and exact-zero branches.

Root continuity reconstructs as `q2-sum(q_i)`. Only that residual uses
`scale_W_cap`, the maximum of the inherited V3 operands and every absolute
`q_law`, `cap_rate`, and final `q`; the other five retain V3 scales. The
fully-authorized family independently reduces to the V3 physical solution,
while the constrained and alternate-start families converge to the same
cap-active physical state within the canonical thresholds.

### V5 generalized Jacobian and exact tie

At the frozen exact-tie operand,
`q_law=cap_rate=5.519758610300214e-06`, so the selected derivative is exact
zero. With root perturbation `6.121512534389777e-05 mm`, the frozen-branch pair
keeps `q=cap_rate` on both sides even though reconstructed law values are
`5.519758688370145e-06` and `5.519758532230283e-06`. Independent trial
reselection makes the latter trial a constitutive-law branch. This proves the
required distinction between freezing a branch within one centered derivative
pair and reselecting at a new trial. All categorical results are identical in
frozen and regenerated fixtures; only the numeric operands differ.

### V5 failures and poisons

The disputed failure numerics cover singular Jacobian, iteration limit,
backtracking limit, invalid authorization, and negative-law redistribution.
Reconstruction confirms the same failure identity, precedence, iteration and
backtracking counts, ordered active caps, nullability, and rollback in both
outputs. The larger backtracking `step_norm` difference is rejected-trajectory
evidence, not an accepted equation or branch change. Poison scalar changes are
derived from the same disputed accepted/failure operands; every poison remains
numerically distinct and its acceptance/rejection meaning is unchanged.

## Authority disposition

The science contract unambiguously authorizes the equations, operand bases,
operation order, active-set rule, and nonlinear acceptance rules above. It does
not distinguish the two within-threshold binary64 terminal trajectories. Exact
fixture identity is nevertheless a separate authority obligation, so numerical
equivalence cannot reconcile the byte mismatch.

1. Preserve V3 through V8 historical identities while provenance remains
   unresolved.
2. If a historical matching generator/operation implementation is recovered,
   review a calculator correction that reproduces the frozen historical bytes.
3. If it is not recovered, admit a prospective successor identity with an
   explicitly pinned oracle runtime/serialization contract.
4. Do not rewrite historical fixtures from current output, weaken exact
   regeneration, or use Rust to choose expected bytes.

This is a science `PASS` for equation and branch semantics and an authority
`HOLD` for exact-byte selection.
