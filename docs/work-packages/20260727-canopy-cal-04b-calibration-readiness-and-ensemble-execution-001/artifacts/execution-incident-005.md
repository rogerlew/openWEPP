# Execution Incident 005

Status: `SCIENCE-DESIGN HOLD AT SYNTHETIC RECOVERY`

Evidence class: `Static`

The retained 2026-07-27 attempt passed preparation, both builds, the twelve-case
native production-consumer proof, and synthetic trace production. The first
synthetic reconstruction then failed before Hubbard population:

```text
Error: "hidden candidate lacks one crossing per year"
```

This is the required identifiability test for recovering a known hidden
threshold vector. The hidden case did not produce the required single crossing
in every synthetic year, so the recovery claim cannot be evaluated. This is a
science-design defect in the synthetic case or threshold parameterization, not
a tooling or execution-authority hold.

The surviving local evidence is:

| Object | SHA-256 | Retained observation |
| --- | --- | --- |
| `/home/workdir/cal04b-objects/synthetic_reconstruct.log` | `758f5304e1f9065c7576201fb89d9bebe9e9c146a35ccd56ee9c0cef3f7a3369` | exact primary error |
| `/home/workdir/cal04b-objects/synthetic_gsi.log` | `a43ea6916411b5da795e25d00ec75d41f75dbc1e87de70d4cf05bdb601592c9d` | producer PASS followed by reconstructor failure |

The generic `.err` file at the same root was subsequently overwritten by a
later failed retry and is not claimed as evidence for this incident. The two
logs above were read statically; Order 2 ran no CAL command.

Disposition: retain Harvard sealed, execute no candidate population, and repair
the synthetic-recovery design only under explicit CAL-04B science authority.
The next CAL attempt must use a fresh execution root and first prove the hidden
case produces the required crossings and is recovered by both reconstructors.
