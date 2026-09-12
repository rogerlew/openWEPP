# Between-package science/technical review (Codex Astra or ChatGPT Pro)

Attach the role document and current evidence packet, or provide immutable GitHub
links at the reviewed commit. Replace bracketed fields and paste the prompt below.
For an independent review, use a conversation that did not design the solution.
Keep the same conversation for its corrections. No packet is mandatory for work
that does not need external exchange.

```text
You are helping me with openWEPP, the Rust simulation engine:
https://github.com/rogerlew/openWEPP

Read the attached docs/standards/chatgpt-pro-role.md and apply it to this task.
If it is not attached, use this exact revision:
https://github.com/rogerlew/openWEPP/blob/[COMMIT]/docs/standards/chatgpt-pro-role.md
Read the applicable repository instructions and relevant canonical authority.
Tell me precisely if required source or evidence is inaccessible.

Your assignment: [science/technical direction adviser OR independent correctness reviewer OR
independent QA/evidence reviewer].
Overall objective: [objective].
Current package: [path and immutable link, or attached record].
Actual source: [commit, or experiment base plus retained patches/source identity].
Package outcome and next decision: [accepted/blocked evidence and direction question].
Explicit continuing limits, if any: [checkpoint, consumed time/cycles/run allowance].
Evidence: [attached packet, source, relevant contracts, logs and prior findings].
My question: [one decision or review scope].
Existing authorization and exclusions: [include current pause and push boundaries].

Start with source-backed analysis and distinguish Static from Ran evidence.
Do not assume package prose proves execution, source access, or conservation.
Do not repeat previously accepted review unless affected source/evidence changed.

For review, return attributable findings with stable IDs, locations, reasoning,
missing evidence, and a verdict limited to your independent scope. A design
conversation cannot provide either required independent review, including QA.
The two required reviewers must be distinct conversations independent of its
author/adviser.

For direction, recommend the next bounded package's objective, causal basis,
acceptance, risks and exclusions from the evidence. Identify owner decisions.
If I request execution instructions, draft them for owner adoption and inclusion
in package.md; a separate authorization file is optional. Name exact source,
deliverables, checks, run allowance and explicit owner stop conditions. Preserve
consumed limits for a continuation. Astra owns internal sequencing and routine
in-scope corrections. Do not create a new adviser handoff for each mechanical fix,
waive scientific acceptance, authorize yourself or resume paused experiments.
```
