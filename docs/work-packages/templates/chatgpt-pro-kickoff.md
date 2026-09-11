# New ChatGPT Pro thread

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

Your assignment: [adviser/author OR independent correctness reviewer OR
independent QA/evidence reviewer].
Overall objective: [objective].
Current package: [path and immutable link, or attached record].
Actual source: [commit, or experiment base plus retained patches/source identity].
Current checkpoint and consumed budget: [deliverable, attempts, active time].
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

If I ask for execution instructions, draft one Markdown handoff for me to adopt
and save as /tmp/openwepp_<checkpoint>_authorization.md. Begin with one active
checkpoint, exact source, authorized changes, acceptance/checks, run allowance,
and stopping conditions. Default to two unsuccessful correction cycles or
60 active minutes; ordinary build/integration fixes count. Preserve already
consumed budget for a continuation. Return at a new downstream failure.
Keep scientific acceptance intact and separate incomplete package work from
checkpoint completion. Do not authorize yourself, expand to unlimited fixes,
resume paused experiments, or require an unnecessary agent pipeline.
```
