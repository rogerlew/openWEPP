# Characterization

Ran before decomposition: a detached worktree at scaffold commit `81311ba2`
received a public `parse_soil` characterization covering every canonical datver
family (`97.5`, `2006.2`, `7777`, `7778`, `9002`, `9003`, and `9005`). It
asserts successful public parse, parsed datver identity, and single-OFE shape
with representative base, extended, and Rosetta layer rows plus policy rows.

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t03-predecomp-target \
  cargo nextest run -p openwepp-input-contract --profile quick \
  -E 'test(cqr23_predecomposition_parse_soil_all_datver_families)'
```

Exit `0`: `1` passed, `5` skipped. The detached worktree was removed after the
run; no production change was present in that oracle.

The production target carries the same public family coverage and additionally
characterizes strict/compatibility datver aliases, all `SOL-E-*` labels and
formatted errors, exact token/quoted-token parsing, policy/header order and
arity, layer ordering, per-OFE/footer restrictive-layer closure, and trailing
record rejection. Assertions consume `parse_soil` or the extracted private
parser seams directly; they do not compare only producer-side intermediate data.
