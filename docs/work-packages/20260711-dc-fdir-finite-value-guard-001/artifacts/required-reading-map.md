# Required-reading map

Status: active
Evidence mode: Static

| Path/group | Tier | Rationale |
| --- | --- | --- |
| Root/work-package/ExecPlan guidance and package files | Core | Autonomous package and gate governance |
| DC, contract authoring/profile/index, ADR-0012 | Conditional, applicable | Defect closure and contract amendment |
| Target contract/spec/source/test and pinned legacy files | On-demand, activated before their phase | Mechanism authority and implementation |
| ADR-0021 and CQR standards/prior hold | On-demand, activated for coverage phase | Science-tier and CRAP closure |

Applicable instruction chain was resolved with `tools/agents/find-agents --for`
for every declared write path: root plus the nearest work-package, science-
contract, crate, test, and fixture `AGENTS.md` files.

Local required-reading bytes: `554345`.
Disposition: `WARN` (`>400000`, below `800000`). The package must combine DC,
canonical contract, parser implementation, and science-tier/CQR closure in one
non-deferring envelope. The contract/spec and parser/test files therefore cannot
be deferred past their respective phases; the catalog and queue documents are
large but mandatory governance/context surfaces. No unrelated SC corpus or
historical package corpus is required.
