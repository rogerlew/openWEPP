# usersum Authoring and Style Guide

Status: Active
Scope: all end-user documentation under `usersum/`
Owner: maintainers (Claude Code maintains this guide)
Last updated: 2026-07-15

`usersum/` is openWEPP's end-user documentation, vendored into wepppy's
in-app documentation engine (manifest-driven, role-gated, rendered in a
GitBook-style shell with full-text search — see
`wepppy/wepppy/weppcloud/routes/usersum/specification.md`). Its readers are
modelers running openWEPP through wepppy or the CLIs, and scientific
reviewers evaluating model output. They are not contributors: they do not
know the repository's internal vocabulary, and they cannot follow a relative
link into `docs/`.

This guide exists because end-user documentation fails differently from
contributor documentation. A contributor doc fails by being incomplete; an
end-user doc fails by being *shaped wrong* — technically accurate content
arranged as an inventory of facts with no argument, hedged and
over-formatted until it reads as generated filler. The rules below are the
codified lessons of rewriting a document that had failed exactly that way
(§7).

## 1. Pick the document's shape before writing

usersum has three document types, and they have different correct shapes.
Most style failures come from writing one type in the shape of another.

**Reference pages** (`openwepp-cli-*.md`, `cli-run-index.md`) are
inventories, and should be. Flags, inputs, outputs, exit behavior — tables
and terse lists are the right form. The skeleton is the existing convention:
`Purpose` → `CLI interface` (a fenced invocation) → `Required inputs` →
`Output` → `Practical notes`. Do not narrate a reference page.

**Narratives** (model descriptions and scientific model-evaluation reports —
e.g. `snow-frost-modeling-and-validation.md`) are arguments, and must not be
inventories. A narrative answers a question a reader arrived with, and every
section advances that answer. Structure one as a scientific paper in
miniature: the problem that frames everything first, then the material
organized by intellectual outcome, then consolidated results, then what the
reader may and may not conclude. Do not structure one as a feature list.

An assurance report is the full scientific-paper case: title, key findings,
plain-language summary, abstract, introduction, formulation, data and methods,
results, discussion, limitations, conclusions, open research, and references.
Its lifecycle metadata is visible near the end, not used as the headline.
It links a version-bound public research-object surface containing every safe
project-owned claim-bearing dataset, table/figure source, procedure, software
identity, and reproduction object required to inspect its results.

**Index / routing pages** (`README.md`, `documentation-agent.md`) are maps:
one line per destination, no content of their own.

## 2. Audience and register

- Open every document with the italic audience line:
  `*Audience: <who>, <deciding/doing what>.*`
- Then write to that audience's floor. If the audience is hydrologists, do
  not define SWE, albedo, or KGE beyond a citation — defining a reader's own
  vocabulary to them is the fastest way to lose their trust. If the audience
  is GIS analysts, do define frost tubes and ice-bulb temperature.
- Terms below the audience's floor get one clause, in place, at first use —
  never a glossary preamble. A document that opens with "A note on
  terminology:" has already picked the wrong audience or the wrong floor.
- Translate internal vocabulary at the boundary. Work-package and
  specification jargon — "evaluation surface", "activation boundary",
  "conservation/activation specification boundary", "snow-neutral",
  R-stage phase names — either gets rendered in plain language or does not
  appear. The test: would this phrase mean anything to a reader who has
  never opened this repository?

## 3. Narrative structure

For narrative documents only; reference pages are exempt by design.

- **The framing problem comes first.** If a single idea governs how
  everything else must be read (for the snow/frost doc: forcing uncertainty
  decides which disagreements are even attributable to the model), it is
  Section 1, stated once, completely. Everything after *uses* it by name and
  never re-explains it.
- **Each fact lives in exactly one home.** State it once, where the argument
  needs it; elsewhere, reference it. If a results table wants to appear in
  two sections, the structure is wrong — consolidate the results.
- **Organize material by intellectual outcome, not by feature.** "What was
  kept / what was replaced / what matched anyway / what was tested and
  rejected" tells a story a feature list cannot. Rejected alternatives are
  presented as outcomes of the method — evidence the evaluation has teeth —
  not as confessions appended for completeness.
- **End with interpretation, not limitations.** The closing section answers
  "what may I conclude from this output, and what may I not" — practical
  guidance — rather than a defensive list of caveats.

## 4. Style rules

Each rule names the failure it prevents.

- **Embody virtue; never announce it.** No "(read this first)", "honest
  scope", "an honest account includes…", "importantly", "it is worth
  noting". If the document is honest, the reader will notice. Announcing it
  reads as performance — the single strongest generated-text tell.
- **Bold is for load-bearing emphasis, used rarely.** A handful of bolded
  phrases per document, at genuine decision points. Bolding keywords
  mid-sentence ("was **not adopted as the default**") is formatting doing
  the job the sentence should do; when everything is emphasized, nothing is.
- **Prose first; bullets only for the genuinely enumerable.** Site lists,
  tolerances, flags: bullets. Reasoning, comparisons, causality: sentences.
  A page that is mostly bullets has usually skipped the thinking that
  connects them.
- **No numbered-principles scaffolding.** "Five principles govern everything
  that follows" is ceremony. If the principles matter, they surface where
  the argument needs them.
- **Numbers travel with their context.** Every metric carries its units, its
  denominator, and its lineage. If a count improves in stages, chain it in
  one place ("1147 → 761 from liquid retention alone, → 498 with the full
  adopted set") rather than dropping unconnected values in different
  sections. If denominators vary across a table, say why in a clause.
- **One idea, one explanation.** If you find yourself re-explaining a
  distinction (forcing-robust vs forcing-limited, say) in a third section,
  delete two and strengthen the first.
- **Tables appear once.** A table pasted verbatim in two sections is the
  structural bug of §3 made visible.
- **Cut the hedging tic, keep the real hedges.** "Provisional, pending
  external review" stated once is a real, useful hedge. The same
  qualification re-attached to every number it touches is noise.

## 5. Claims and evidence

The repository's truthfulness discipline extends to end-user docs in
reader-facing form:

- Keep software verification, empirical evidence, and application fitness
  separate. Verification may pass or fail against a named requirement;
  empirical evidence is described through the study's quantitative results,
  domain, uncertainty, and conclusion. Do not turn either into a page-level
  aggregate grade or call a model, release, process, or result simply
  "validated."
- A usersum narrative explains why the model behaves as it does. Claim-bearing
  evaluation method and results belong in an approved scientific report and its
  technical supplement; do not duplicate a numeric results table across those
  records. The narrative and report cross-link so readers can move from “why and
  how” to “what this evaluation showed.”
- Assurance drafts and review candidates stay outside public `usersum`. The
  public catalog lists only independently reviewed, approved reports and may
  truthfully contain zero reports. An absent study belongs in a gap record or
  narrative, not a status-first public report.
- Application fitness belongs to the named user or institution. Public text may
  explain applicability and limitations but must not issue an unnamed site's
  fitness verdict.
- Never present capability as behavior. "Implemented but not a default" is a
  distinct state and must be labeled every time it appears; a reader
  configuring a run needs to know what the binary *does*, not what the
  codebase *contains*.
- Command examples must reflect actual release-binary behavior — run them
  against the current binary before landing, per the documentation-agent
  scope. A stale flag in a CLI page is a correctness bug, not a style issue.
- Comparisons must be like-for-like, and the document should say what was
  compared (which build, which sites, which cells were scoreable). This is
  the user-facing edge of the repo's comparator discipline.

## 6. Mechanics

- **Files:** kebab-case names; one document, one topic.
- **Titles:** `# <Topic>` plain; CLI pages backtick the binary name.
- **Wrapping:** hard-wrap near 80 columns for narrative prose; code fences
  and tables run as long as they need.
- **Links: usersum must be self-contained.** Link freely within `usersum/`;
  never link into `docs/`, `crates/`, or other repo paths — vendored copies
  render inside wepppy, where those targets do not exist. If contributor
  context is essential, name the document ("the array-native runtime
  specification in the openWEPP repository") without a hyperlink.
- **Equations:** fenced code blocks in documented form, with the caveat
  pattern when code coefficients differ: "The constants above are the
  documented form; the production code carries the same physics with its own
  internally consistent coefficients and sign conventions."
- **References:** author–date in text, APA-style list under `## References`,
  ordered alphabetically. Cite the published source, not the work package
  that implemented it.
- **Versioning (narrative documents):** science/authority narratives carry
  `*Version X.Y — YYYY-MM-DD*` on the line under the title, and close with
  a `## Revision Log` section as the document's final section (after
  `## References`): a `Version | Date | Changes` table, oldest row first,
  one row per landed revision with a one-sentence summary of what changed
  and why (an operator review, a model change, a validation update). Bump
  the minor version for any content change; the header line always matches
  the newest log row. Scientific reviewers cite these documents, so they
  need to know which version they read. Reference and index pages are
  exempt — they version with the binaries and the tree.
- **Headings and terms are the search surface.** The vendored engine is
  full-text-indexed; put the words a user would search for (the binary name,
  the process name, the observable) in headings and first sentences.

## 7. The canonical example

`usersum/snow-frost-modeling-and-validation.md` was rewritten against these
rules on 2026-07-01. The before/after pair is the fastest way to calibrate:

```bash
git show 33a4c8b1:usersum/snow-frost-modeling-and-validation.md   # before
git show ffbd66ae:usersum/snow-frost-modeling-and-validation.md   # after
```

Same claims, same numbers, same references. What changed: a glossary
defining SWE to hydrologists, deleted; "(read this first)" and "honest
scope", deleted; ~60 bolded phrases reduced to a handful; one framing
problem stated once instead of three times; a results table appearing once
instead of twice; internal jargon translated; and the findings moved from
line 367 to the sections that motivate them.

## 8. Pre-landing checklist

Before landing a usersum document (human or agent):

- [ ] Document type identified; shape matches type (§1).
- [ ] Audience line present; no term defined below the audience's floor; no
      glossary preamble (§2).
- [ ] Narrative docs: framing problem first; no idea explained twice; no
      table duplicated; results consolidated; closes with interpretation (§3).
- [ ] No virtue announcements; bold count is a handful; bullets only where
      content is enumerable (§4).
- [ ] Every number has units and denominator; staged changes chained once (§4).
- [ ] Verification, empirical characterization, and application fitness are
      separate; no unqualified "validated" label; capability vs default
      behavior labeled (§5).
- [ ] Claim-bearing evaluation results have one canonical home in an approved
      scientific report, with narrative cross-links instead of duplication
      (§5).
- [ ] CLI examples verified against the current release binary (§5).
- [ ] No links outside `usersum/` (§6).
- [ ] Narrative docs: version line under the title matches the newest
      `## Revision Log` row, and the log has a row for this change (§6).
- [ ] Read one section aloud. If it sounds like it is reassuring you rather
      than telling you something, rewrite it.
