# Research Basis

Status: complete for ASSURE-02 architecture authoring

## Research Question

What documentation form and reproducibility contract will let hydrologists,
soil scientists, researchers, and practitioners understand and audit an
openWEPP model-evaluation claim without making internal governance machinery the
scientific product?

## Sources And Architectural Use

| Source | Relevant guidance | ASSURE-02 use |
| --- | --- | --- |
| [EPA Guidance on the Development, Evaluation, and Application of Environmental Models](https://www.epa.gov/sites/default/files/2015-04/documents/cred_guidance_0309.pdf) (2009) | Model evaluation is ongoing and application-specific; it considers scientific basis, data, correspondence with observations, sensitivity, uncertainty, peer review, and application niche. Transparency requires clear, complete documentation at an audience-appropriate level. | Treat environmental validation as claim-specific evidence, not a terminal universal verdict. Require purpose, domain, data quality, uncertainty, limitations, and review in the public report. |
| [Oreskes, Shrader-Frechette, and Belitz](https://doi.org/10.1126/science.263.5147.641) (1994) | Complete verification or validation of open natural systems is logically precluded; observational agreement is partial confirmation. Computer-code correctness remains a separable question. | Preserve the verification/corroboration asymmetry and prohibit an unqualified “validated model” headline. |
| [Bennett et al.](https://doi.org/10.1016/j.envsoft.2012.09.011) (2013) | Performance characterization begins with aim, scale, scope, and data; it combines graphical, quantitative, and qualitative evidence and distinguishes performance evidence from user adoption. | Require claim envelope, data characterization, more than one informative view where empirical performance is assessed, and a separate application decision. |
| [Grimm et al.](https://doi.org/10.1016/j.ecolmodel.2014.01.018) (2014) | TRACE links rationale, design, testing, and intended use across the model life cycle and treats documentation as part of good modeling practice. | Keep “why,” “how,” and “what” connected through the report, supplement, model narrative, and contracts rather than publishing a detached scorecard. |
| [Moriasi et al.](https://www.ars.usda.gov/research/publications/publication/?seqNo115=197994) (2007) | Watershed-model evaluation should use defensible statistical and graphical techniques selected for the quantity and project context. | Require authors to justify metrics and diagnostic graphics; do not impose one universal metric or threshold across quantities. |
| [Wilkinson et al.](https://doi.org/10.1038/sdata.2016.18) (2016) | Research objects should be findable, accessible, interoperable, and reusable for people and machines; the principles precede implementation choices. | Require stable identities, metadata, access, and reuse information while deferring a technology-heavy evidence graph until the manuscript demonstrates the needed fields. |
| [AGU manuscript requirements](https://www.agu.org/publications/authors/journals/text-graphics-requirements) | Earth-science manuscripts use recognizable title, abstract, plain-language, text, open-research, reference, table, and figure surfaces. Supporting information is subordinate; interpretation belongs in the main text. | Base the public report on conventional scientific structure. Keep detailed traceability and methods in a linked supplement without exiling the argument or limitations from the report. |
| [AGU data and software guidance](https://www.agu.org/publications/authors/journals/data-software-for-authors) | Data and software need direct availability statements, formal citations, versions, persistent identifiers where available, and preserved inputs behind tables and figures. | Require an open-research statement, content-identified dependencies, table/figure source data, software realization, and reproduction instructions. |
| [NASA-STD-7009B](https://standards.nasa.gov/sites/default/files/standards/NASA/B/1/NASA-STD-7009B-Final-3-5-2024.pdf) (2024) | Credibility assessment is tied to intended use, referents, limits, uncertainty, and program-defined acceptance authority. | Retain useful intended-use, referent, limit, and credibility concepts, but do not import NASA's program-acceptance hierarchy as openWEPP's public validation verdict. |
| [USGS Fundamental Science Practices for scientific data](https://www.usgs.gov/survey-manual/5028-fundamental-science-practices-review-approval-and-release-usgs-scientific-data) (2026) | Public scientific data, including model inputs and outputs, require review, approval, metadata, preservation, and open machine-readable release where possible. | Separate staging from public release, require named human approval, preserve model inputs/results, and treat preliminary material as nonpublic. |

## Synthesis

The sources converge on a smaller architecture than v1:

1. The scientific argument belongs in a conventional public manuscript.
2. The report must state its question, quantity, scale, domain, referent,
   method, results, uncertainty, limitations, and conclusion in domain language.
3. Detailed reproducibility and traceability belong in a reviewed supporting
   record, linked from but subordinate to the report.
4. Machine records make identities and dependencies checkable; they do not
   choose metrics, interpret results, or issue a scientific conclusion.
5. Drafts and candidates remain in staging. Public inclusion follows
   independent scientific and publication review plus named human approval.
6. Application fitness is a new assessment by the decision owner, not a label
   inherited from a model report.

This synthesis rejects two extremes: a status-first governance page that lacks
a scientific argument, and an attractive narrative whose numbers cannot be
traced or reproduced.
