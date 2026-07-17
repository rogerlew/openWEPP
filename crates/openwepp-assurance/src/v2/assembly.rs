use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Component, Path, PathBuf};

use serde::Serialize;

use super::{
    ContentSource, PrincipalRegistry, Report, ReportSource, RequiredNullable, ResearchObject,
    ResultObject, ResultValue, Table, V2Plan, V2PlanState, V2ReportPlan, ValueBinding,
    digest_input_set, parse_hydrated_yaml, parse_json, read_regular_confined,
    validate_catalog_binding, validate_display, validate_report, validate_report_structure,
};
use crate::{AssuranceError, Result, sha256_bytes};

use super::confined::ConfinedDirectory;
use super::identity::{IdentityLock, ReviewLock};

pub(super) const ASSEMBLY_TOOL_ID: &str = "openwepp-assurance-assembly:1";
const OUTPUT_PREFIX: &str = "usersum/assurance/reports";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2AssemblyResult {
    pub reports: Vec<V2AssemblySummary>,
    pub outputs: BTreeMap<PathBuf, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2AssemblySummary {
    pub id: String,
    pub version: String,
    pub source_root_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Operation {
    Build,
    Check,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn execute(
    repository_root: &Path,
    requested_staging_root: &Path,
    shared_inputs: &BTreeMap<PathBuf, String>,
    identity: &IdentityLock,
    principals: &PrincipalRegistry,
    sources: &[&ReportSource],
    plan: &V2Plan,
    operation: Operation,
) -> Result<V2AssemblyResult> {
    execute_with_post_install(
        repository_root,
        requested_staging_root,
        shared_inputs,
        identity,
        principals,
        sources,
        plan,
        operation,
        &mut || Ok(()),
    )
}

#[allow(clippy::too_many_arguments)]
fn execute_with_post_install(
    repository_root: &Path,
    requested_staging_root: &Path,
    shared_inputs: &BTreeMap<PathBuf, String>,
    identity: &IdentityLock,
    principals: &PrincipalRegistry,
    sources: &[&ReportSource],
    plan: &V2Plan,
    operation: Operation,
    post_install: &mut dyn FnMut() -> Result<()>,
) -> Result<V2AssemblyResult> {
    let staging = prepare_staging_root(repository_root, requested_staging_root, operation)?;
    require_current_plan(plan, sources)?;
    let mut rendered = Vec::new();
    for source in sources {
        let report_plan = plan
            .reports
            .iter()
            .find(|candidate| candidate.id == source.id)
            .ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "assembly plan omitted selected report '{}'",
                    source.id
                ))
            })?;
        rendered.push(render_report(
            repository_root,
            identity,
            principals,
            source,
            report_plan,
        )?);
    }
    rendered.sort_by(|left, right| left.summary.id.cmp(&right.summary.id));

    verify_assembly_inputs(repository_root, shared_inputs, identity, sources, plan)?;
    match operation {
        Operation::Build => {
            build_reports_transactionally(
                repository_root,
                &staging,
                shared_inputs,
                identity,
                sources,
                plan,
                &rendered,
                post_install,
            )?;
        }
        Operation::Check => {
            for report in &rendered {
                check_rendered_report(&staging, report, true)?;
            }
            verify_assembly_inputs(repository_root, shared_inputs, identity, sources, plan)?;
            staging.verify_identity()?;
        }
    }

    let mut outputs = BTreeMap::new();
    let mut summaries = Vec::new();
    for report in rendered {
        for (relative, bytes) in &report.files {
            outputs.insert(report.output_base.join(relative), sha256_bytes(bytes));
        }
        summaries.push(report.summary);
    }
    Ok(V2AssemblyResult {
        reports: summaries,
        outputs,
    })
}

fn verify_assembly_inputs(
    repository_root: &Path,
    shared_inputs: &BTreeMap<PathBuf, String>,
    identity: &IdentityLock,
    sources: &[&ReportSource],
    plan: &V2Plan,
) -> Result<()> {
    verify_shared_inputs(repository_root, shared_inputs)?;
    let repeated = super::planner::plan_sources(
        repository_root,
        shared_inputs,
        identity,
        plan.total_report_count,
        sources,
    )?;
    if repeated == *plan {
        Ok(())
    } else {
        Err(AssuranceError::Drift(
            "v2 assurance sources changed during assembly".to_owned(),
        ))
    }
}

fn require_current_plan(plan: &V2Plan, sources: &[&ReportSource]) -> Result<()> {
    if plan.reports.len() != sources.len() {
        return Err(AssuranceError::Invalid(
            "assembly plan/source selection count mismatch".to_owned(),
        ));
    }
    for report in &plan.reports {
        if report.state != V2PlanState::Current
            || report
                .nodes
                .iter()
                .any(|node| node.state != V2PlanState::Current)
        {
            return Err(AssuranceError::Invalid(format!(
                "report '{}' is not current and cannot be assembled",
                report.id
            )));
        }
        if report.source_root_sha256.is_none() {
            return Err(AssuranceError::Invalid(format!(
                "report '{}' has no current source-root identity",
                report.id
            )));
        }
    }
    Ok(())
}

struct RenderedReport {
    summary: V2AssemblySummary,
    output_base: PathBuf,
    report_root: PathBuf,
    files: BTreeMap<PathBuf, Vec<u8>>,
}

fn render_report(
    root: &Path,
    identity: &IdentityLock,
    principals: &PrincipalRegistry,
    source: &ReportSource,
    plan: &V2ReportPlan,
) -> Result<RenderedReport> {
    let manifest_bytes = identified_bytes(root, &source.manifest_path, &source.manifest_sha256)?;
    let report: Report = parse_hydrated_yaml(&source.manifest_path, &manifest_bytes, identity)?;
    validate_catalog_binding(source, &report)?;
    let mut report_inputs = BTreeMap::new();
    validate_report(root, &report, &mut report_inputs)?;
    let review_path = PathBuf::from(format!(
        "assurance/v2/reports/{}/review.lock.json",
        report.id
    ));
    let review_bytes = identified_bytes(root, &review_path, identity.digest_for(&review_path)?)?;
    let review_lock = ReviewLock::parse(&review_path, &review_bytes)?;
    let bindings = resolve_bindings(root, &report)?;
    let mut usage = Usage::default();
    let mut figures = BTreeMap::new();
    let manuscript = render_content(
        root,
        &report,
        &report.manuscript,
        principals,
        &review_lock,
        &bindings,
        &mut usage,
        &mut figures,
    )?;
    let supplement = render_content(
        root,
        &report,
        &report.supplement,
        principals,
        &review_lock,
        &bindings,
        &mut usage,
        &mut figures,
    )?;
    validate_usage(&report, &usage, &bindings)?;

    let version_root = PathBuf::from(&report.version);
    let mut files = BTreeMap::from([
        (version_root.join("index.md"), manuscript.into_bytes()),
        (version_root.join("supplement.md"), supplement.into_bytes()),
    ]);
    for (id, bytes) in figures {
        insert_output(
            &mut files,
            version_root.join("figures").join(format!("{id}.svg")),
            bytes,
        )?;
    }
    stage_research_objects(
        root,
        &report,
        principals,
        &review_lock,
        &usage,
        &version_root,
        &mut files,
    )?;

    let source_root_sha256 = plan.source_root_sha256.clone().ok_or_else(|| {
        AssuranceError::Invalid(format!("report '{}' lacks a source root", report.id))
    })?;
    let manifest = build_manifest(&report, &source_root_sha256, &version_root, &files)?;
    insert_output(
        &mut files,
        version_root.join("build-manifest.json"),
        manifest,
    )?;
    let output_base = PathBuf::from(OUTPUT_PREFIX).join(&report.id);
    Ok(RenderedReport {
        summary: V2AssemblySummary {
            id: report.id.clone(),
            version: report.version.clone(),
            source_root_sha256,
        },
        report_root: output_base.clone(),
        output_base,
        files,
    })
}

fn identified_bytes(root: &Path, path: &Path, expected: &str) -> Result<Vec<u8>> {
    let bytes = read_regular_confined(root, path)?;
    if sha256_bytes(&bytes) != expected {
        return Err(AssuranceError::Drift(format!(
            "identified assembly source changed: {}",
            path.display()
        )));
    }
    Ok(bytes)
}

#[derive(Clone)]
struct ResolvedValue {
    title: String,
    result_id: String,
    unit_id: String,
    unit_symbol: String,
    transform: String,
    value: f64,
    rendered: String,
}

struct ResolvedBindings {
    values: BTreeMap<String, ResolvedValue>,
    used_result_values: BTreeSet<(String, String)>,
}

fn resolve_bindings(root: &Path, report: &Report) -> Result<BTreeMap<String, ResolvedValue>> {
    let ids = validate_report_structure(report)?;
    let objects = load_result_objects(root, report, &ids)?;
    let resolved = resolve_value_bindings(report, &objects)?;
    require_all_result_values_bound(report, &objects, &resolved.used_result_values)?;
    validate_table_bindings(report, &resolved.values)?;
    validate_figure_bindings(report, &resolved.values)?;
    Ok(resolved.values)
}

fn load_result_objects(
    root: &Path,
    report: &Report,
    ids: &super::ReportIds,
) -> Result<BTreeMap<String, ResultObject>> {
    let mut objects = BTreeMap::new();
    for source in &report.results {
        let bytes = identified_bytes(root, &source.path, &source.sha256)?;
        let object: ResultObject = parse_json(&source.path, &bytes)?;
        super::validate_result_object(source, &object, ids)?;
        objects.insert(source.id.clone(), object);
    }
    Ok(objects)
}

fn resolve_value_bindings(
    report: &Report,
    objects: &BTreeMap<String, ResultObject>,
) -> Result<ResolvedBindings> {
    let units = report
        .units
        .iter()
        .map(|unit| (unit.id.as_str(), unit.symbol.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut used_values = BTreeSet::new();
    let mut resolved = BTreeMap::new();
    for binding in &report.value_bindings {
        let value = resolve_value_binding(binding, objects, &units)?;
        used_values.insert((binding.result_id.clone(), binding.value_id.clone()));
        if resolved.insert(binding.id.clone(), value).is_some() {
            return Err(AssuranceError::Invalid(format!(
                "duplicate value binding '{}'",
                binding.id
            )));
        }
    }
    Ok(ResolvedBindings {
        values: resolved,
        used_result_values: used_values,
    })
}

fn resolve_value_binding(
    binding: &ValueBinding,
    objects: &BTreeMap<String, ResultObject>,
    units: &BTreeMap<&str, &str>,
) -> Result<ResolvedValue> {
    let object = objects.get(&binding.result_id).ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "value binding '{}' refers to missing result '{}'",
            binding.id, binding.result_id
        ))
    })?;
    let value = unique_result_value(object, &binding.value_id, &binding.id)?;
    if value.unit_id != binding.unit_id {
        return Err(AssuranceError::Invalid(format!(
            "value binding '{}' unit '{}' does not match result unit '{}'",
            binding.id, binding.unit_id, value.unit_id
        )));
    }
    let unit_symbol = units.get(binding.unit_id.as_str()).ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "value binding '{}' has unknown unit '{}'",
            binding.id, binding.unit_id
        ))
    })?;
    let transformed = transform_value(binding, value.value)?;
    Ok(ResolvedValue {
        title: binding.title.clone(),
        result_id: binding.result_id.clone(),
        unit_id: binding.unit_id.clone(),
        unit_symbol: (*unit_symbol).to_owned(),
        transform: binding.transform.clone(),
        value: transformed,
        rendered: render_number(transformed, &binding.display)?,
    })
}

fn transform_value(binding: &ValueBinding, value: f64) -> Result<f64> {
    let transformed = match binding.transform.as_str() {
        "identity" => value,
        "absolute" => value.abs(),
        _ => {
            return Err(AssuranceError::Invalid(format!(
                "value binding '{}' has unsupported transform",
                binding.id
            )));
        }
    };
    if !transformed.is_finite() {
        return Err(AssuranceError::Invalid(format!(
            "value binding '{}' produced a nonfinite value",
            binding.id
        )));
    }
    Ok(transformed)
}

fn require_all_result_values_bound(
    report: &Report,
    objects: &BTreeMap<String, ResultObject>,
    used_values: &BTreeSet<(String, String)>,
) -> Result<()> {
    for source in &report.results {
        let object = objects.get(&source.id).ok_or_else(|| {
            AssuranceError::Invalid(format!("missing result object '{}'", source.id))
        })?;
        for value in &object.values {
            if !used_values.contains(&(source.id.clone(), value.id.clone())) {
                return Err(AssuranceError::Invalid(format!(
                    "orphaned result value '{}:{}' has no value binding",
                    source.id, value.id
                )));
            }
        }
    }
    Ok(())
}

fn unique_result_value<'a>(
    object: &'a ResultObject,
    value_id: &str,
    binding_id: &str,
) -> Result<&'a ResultValue> {
    let mut values = object.values.iter().filter(|value| value.id == value_id);
    let value = values.next().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "value binding '{binding_id}' refers to missing value '{value_id}'"
        ))
    })?;
    if values.next().is_some() {
        return Err(AssuranceError::Invalid(format!(
            "value binding '{binding_id}' refers to duplicate value '{value_id}'"
        )));
    }
    Ok(value)
}

fn render_number(value: f64, display: &str) -> Result<String> {
    let (kind, precision) = validate_display(display)?;
    match (kind, precision) {
        ("integer", None) if value.fract() == 0.0 => Ok(format!("{value:.0}")),
        ("integer", None) => Err(AssuranceError::Invalid(format!(
            "integer display cannot represent nonintegral value {value}"
        ))),
        ("fixed", Some(precision)) => Ok(format!("{value:.precision$}")),
        ("scientific", Some(precision)) => format_scientific(value, precision),
        _ => Err(AssuranceError::Invalid(format!(
            "unsupported display precision '{display}'"
        ))),
    }
}

fn format_scientific(value: f64, precision: usize) -> Result<String> {
    let raw = format!("{value:.precision$e}");
    let (mantissa, exponent) = raw.split_once('e').ok_or_else(|| {
        AssuranceError::Invalid("scientific formatter omitted exponent".to_owned())
    })?;
    let exponent = exponent.parse::<i32>().map_err(|_| {
        AssuranceError::Invalid("scientific formatter emitted invalid exponent".to_owned())
    })?;
    Ok(format!("{mantissa}e{exponent:+03}"))
}

fn validate_table_bindings(
    report: &Report,
    bindings: &BTreeMap<String, ResolvedValue>,
) -> Result<()> {
    for table in &report.tables {
        for row in &table.rows {
            for (column, binding_id) in table.columns.iter().zip(&row.value_binding_ids) {
                let binding = bindings.get(binding_id).ok_or_else(|| {
                    AssuranceError::Invalid(format!(
                        "table '{}' refers to missing value binding '{}'",
                        table.id, binding_id
                    ))
                })?;
                if let RequiredNullable::Value(unit_id) = &column.unit_id
                    && binding.unit_id != *unit_id
                {
                    return Err(AssuranceError::Invalid(format!(
                        "table '{}' column unit '{}' does not match binding '{}' unit '{}'",
                        table.id, unit_id, binding_id, binding.unit_id
                    )));
                }
            }
        }
    }
    Ok(())
}

fn validate_figure_bindings(
    report: &Report,
    bindings: &BTreeMap<String, ResolvedValue>,
) -> Result<()> {
    for figure in &report.figures {
        let mut result_ids = BTreeSet::new();
        let mut unit_id = None;
        for binding_id in &figure.value_binding_ids {
            let binding = bindings.get(binding_id).ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "figure '{}' refers to missing value binding '{}'",
                    figure.id, binding_id
                ))
            })?;
            if binding.transform != "absolute" || binding.value <= 0.0 {
                return Err(AssuranceError::Invalid(format!(
                    "figure '{}' requires positive absolute value bindings",
                    figure.id
                )));
            }
            if unit_id
                .as_ref()
                .is_some_and(|expected| expected != &binding.unit_id)
            {
                return Err(AssuranceError::Invalid(format!(
                    "figure '{}' mixes value-binding units",
                    figure.id
                )));
            }
            unit_id = Some(binding.unit_id.clone());
            result_ids.insert(binding.result_id.clone());
        }
        if result_ids != figure.result_ids.iter().cloned().collect() {
            return Err(AssuranceError::Invalid(format!(
                "figure '{}' result IDs do not match its value bindings",
                figure.id
            )));
        }
    }
    Ok(())
}

#[derive(Default)]
struct Usage {
    values: BTreeSet<String>,
    tables: BTreeSet<String>,
    figures: BTreeSet<String>,
    references: BTreeSet<String>,
    research_objects: BTreeSet<String>,
}

#[allow(clippy::too_many_arguments)]
fn render_content(
    root: &Path,
    report: &Report,
    content: &ContentSource,
    principals: &PrincipalRegistry,
    review_lock: &ReviewLock,
    bindings: &BTreeMap<String, ResolvedValue>,
    usage: &mut Usage,
    figure_outputs: &mut BTreeMap<String, Vec<u8>>,
) -> Result<String> {
    let bytes = identified_bytes(root, &content.path, &content.sha256)?;
    let template = std::str::from_utf8(&bytes).map_err(|error| {
        AssuranceError::Invalid(format!(
            "authored Markdown '{}' is not UTF-8: {error}",
            content.path.display()
        ))
    })?;
    let mut output = String::with_capacity(template.len() + 2048);
    let mut cursor = 0;
    while let Some(relative_start) = template[cursor..].find("{{") {
        let start = cursor + relative_start;
        let literal = &template[cursor..start];
        reject_unresolved_closer(literal)?;
        reject_authored_link(literal)?;
        reject_authored_quantity(literal, report)?;
        output.push_str(literal);
        let body_start = start + 2;
        let relative_end = template[body_start..].find("}}").ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "unterminated assembly directive in {}",
                content.path.display()
            ))
        })?;
        let end = body_start + relative_end;
        let body = &template[body_start..end];
        let directive = parse_directive(body)?;
        validate_block_position(template, start, end + 2, &directive)?;
        render_directive(
            root,
            report,
            content,
            principals,
            review_lock,
            bindings,
            usage,
            figure_outputs,
            directive,
            &mut output,
        )?;
        cursor = end + 2;
    }
    let literal = &template[cursor..];
    reject_unresolved_closer(literal)?;
    reject_authored_link(literal)?;
    reject_authored_quantity(literal, report)?;
    output.push_str(literal);
    if output.contains("{{") || output.contains("}}") {
        return Err(AssuranceError::Invalid(format!(
            "unresolved assembly directive in {}",
            content.path.display()
        )));
    }
    Ok(output)
}

fn reject_unresolved_closer(literal: &str) -> Result<()> {
    if literal.contains("}}") {
        return Err(AssuranceError::Invalid(
            "authored Markdown contains an unmatched directive closer".to_owned(),
        ));
    }
    Ok(())
}

fn reject_authored_link(literal: &str) -> Result<()> {
    let lowercase = literal.to_ascii_lowercase();
    if literal.contains("](")
        || literal.contains('@')
        || lowercase.contains("://")
        || lowercase.contains("www.")
    {
        return Err(AssuranceError::Invalid(
            "authored Markdown links and autolinks must use typed link directives".to_owned(),
        ));
    }
    Ok(())
}

fn reject_authored_quantity(literal: &str, report: &Report) -> Result<()> {
    for unit in &report.units {
        let mut cursor = 0;
        while let Some(offset) = literal[cursor..].find(&unit.symbol) {
            let unit_start = cursor + offset;
            let unit_end = unit_start + unit.symbol.len();
            let bounded_after = literal[unit_end..]
                .chars()
                .next()
                .is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_');
            if bounded_after && numeric_token_before(&literal[..unit_start]).is_some() {
                return Err(AssuranceError::Invalid(format!(
                    "authored numeric quantity with unit '{}' must use a typed quantity directive",
                    unit.symbol
                )));
            }
            cursor = unit_end;
        }
    }
    Ok(())
}

fn numeric_token_before(prefix: &str) -> Option<&str> {
    let prefix = prefix.trim_end();
    let start = prefix
        .char_indices()
        .rev()
        .find(|(_, character)| {
            !character.is_ascii_digit() && !matches!(character, '.' | ',' | '+' | '-' | 'e' | 'E')
        })
        .map_or(0, |(index, character)| index + character.len_utf8());
    let token = &prefix[start..];
    if token.is_empty() {
        return None;
    }
    let normalized = token.replace(',', "");
    normalized.parse::<f64>().ok().map(|_| token)
}

#[derive(Clone, Copy)]
enum Directive<'a> {
    Quantity(&'a str),
    Table(&'a str),
    Figure(&'a str),
    Reference(&'a str),
    ReportLink(&'a str),
    SupplementLink(&'a str),
    ResearchObjectLink { id: &'a str, label: &'a str },
    UsersumLink { path: &'a str, label: &'a str },
    Attribution,
    Lifecycle,
}

impl Directive<'_> {
    const fn is_block(&self) -> bool {
        matches!(
            self,
            Self::Table(_) | Self::Figure(_) | Self::Attribution | Self::Lifecycle
        )
    }
}

fn parse_directive(body: &str) -> Result<Directive<'_>> {
    if body.is_empty()
        || body.contains(['\n', '\r', '\t', '{', '}'])
        || body.bytes().any(|byte| byte.is_ascii_control())
    {
        return Err(AssuranceError::Invalid(format!(
            "malformed assembly directive '{{{{{body}}}}}'"
        )));
    }
    if let Some(id) = body.strip_prefix("quantity:") {
        validate_directive_id(id)?;
        return Ok(Directive::Quantity(id));
    }
    if let Some(id) = body.strip_prefix("table:") {
        validate_directive_id(id)?;
        return Ok(Directive::Table(id));
    }
    if let Some(id) = body.strip_prefix("figure:") {
        validate_directive_id(id)?;
        return Ok(Directive::Figure(id));
    }
    if let Some(id) = body.strip_prefix("reference:") {
        validate_directive_id(id)?;
        return Ok(Directive::Reference(id));
    }
    if let Some(label) = body.strip_prefix("link:report|") {
        validate_label(label)?;
        return Ok(Directive::ReportLink(label));
    }
    if let Some(label) = body.strip_prefix("link:supplement|") {
        validate_label(label)?;
        return Ok(Directive::SupplementLink(label));
    }
    if let Some(rest) = body.strip_prefix("link:research-object:") {
        let (id, label) = split_once_exact(rest)?;
        validate_directive_id(id)?;
        validate_label(label)?;
        return Ok(Directive::ResearchObjectLink { id, label });
    }
    if let Some(rest) = body.strip_prefix("link:usersum:") {
        let (path, label) = split_once_exact(rest)?;
        validate_usersum_path(Path::new(path))?;
        validate_label(label)?;
        return Ok(Directive::UsersumLink { path, label });
    }
    if body == "assurance:attribution" {
        return Ok(Directive::Attribution);
    }
    if body == "assurance:lifecycle" {
        return Ok(Directive::Lifecycle);
    }
    Err(AssuranceError::Invalid(format!(
        "unknown assembly directive '{{{{{body}}}}}'"
    )))
}

fn split_once_exact(value: &str) -> Result<(&str, &str)> {
    let (left, right) = value.split_once('|').ok_or_else(|| {
        AssuranceError::Invalid("link directive requires one label separator".to_owned())
    })?;
    if right.contains('|') {
        return Err(AssuranceError::Invalid(
            "link directive contains multiple label separators".to_owned(),
        ));
    }
    Ok((left, right))
}

fn validate_directive_id(id: &str) -> Result<()> {
    super::validate_id(id, "assembly directive")
}

fn validate_label(label: &str) -> Result<()> {
    if label.trim().is_empty()
        || label != label.trim()
        || label.contains(['[', ']', '(', ')', '{', '}', '<', '>', '\n', '\r'])
    {
        return Err(AssuranceError::Invalid(
            "assembly link label contains unsafe Markdown".to_owned(),
        ));
    }
    Ok(())
}

fn validate_block_position(
    template: &str,
    start: usize,
    after: usize,
    directive: &Directive<'_>,
) -> Result<()> {
    if !directive.is_block() {
        return Ok(());
    }
    let line_start = template[..start].rfind('\n').map_or(0, |index| index + 1);
    let line_end = template[after..]
        .find('\n')
        .map_or(template.len(), |index| after + index);
    if !template[line_start..start].trim().is_empty()
        || !template[after..line_end].trim().is_empty()
    {
        return Err(AssuranceError::Invalid(
            "table and figure directives must occupy their own line".to_owned(),
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn render_directive(
    root: &Path,
    report: &Report,
    content: &ContentSource,
    principals: &PrincipalRegistry,
    review_lock: &ReviewLock,
    bindings: &BTreeMap<String, ResolvedValue>,
    usage: &mut Usage,
    figure_outputs: &mut BTreeMap<String, Vec<u8>>,
    directive: Directive<'_>,
    output: &mut String,
) -> Result<()> {
    match directive {
        Directive::Quantity(id) => {
            require_declared(id, &content.value_binding_ids, "value binding")?;
            let value = bindings
                .get(id)
                .ok_or_else(|| AssuranceError::Invalid(format!("unknown value binding '{id}'")))?;
            usage.values.insert(id.to_owned());
            let _ = write!(output, "{} {}", value.rendered, value.unit_symbol);
        }
        Directive::Table(id) => {
            require_declared(id, &content.table_ids, "table")?;
            let table = find_table(report, id)?;
            usage.tables.insert(id.to_owned());
            output.push_str(&render_table(table, bindings, usage)?);
        }
        Directive::Figure(id) => {
            require_declared(id, &content.figure_ids, "figure")?;
            let figure = report
                .figures
                .iter()
                .find(|figure| figure.id == id)
                .ok_or_else(|| AssuranceError::Invalid(format!("unknown figure '{id}'")))?;
            usage.figures.insert(id.to_owned());
            let (markdown, svg) = render_figure(figure, bindings, usage)?;
            if figure_outputs.insert(id.to_owned(), svg).is_some() {
                return Err(AssuranceError::Invalid(format!(
                    "figure '{id}' is rendered more than once"
                )));
            }
            output.push_str(&markdown);
        }
        Directive::Reference(id) => {
            require_declared(id, &content.reference_ids, "reference")?;
            let reference = report
                .references
                .iter()
                .find(|reference| reference.id == id)
                .ok_or_else(|| AssuranceError::Invalid(format!("unknown reference '{id}'")))?;
            usage.references.insert(id.to_owned());
            output.push_str(&render_reference(reference)?);
        }
        Directive::ReportLink(label) => {
            let _ = write!(output, "[{label}](index.md)");
        }
        Directive::SupplementLink(label) => {
            let _ = write!(output, "[{label}](supplement.md)");
        }
        Directive::ResearchObjectLink { id, label } => {
            require_declared(id, &content.research_object_ids, "research object")?;
            let object = find_public_research_object(report, id)?;
            let basename = research_object_basename(object)?;
            usage.research_objects.insert(id.to_owned());
            let _ = write!(output, "[{label}](research-objects/{basename})");
        }
        Directive::UsersumLink { path, label } => {
            let relative = Path::new(path);
            validate_usersum_source(root, relative)?;
            let _ = write!(output, "[{label}](../../../../{path})");
        }
        Directive::Attribution => output.push_str(&render_attribution(report, principals)?),
        Directive::Lifecycle => output.push_str(&render_lifecycle(review_lock)),
    }
    Ok(())
}

fn require_declared(id: &str, declared: &[String], kind: &str) -> Result<()> {
    if declared.iter().any(|candidate| candidate == id) {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "{kind} '{id}' is not declared for this authored content"
        )))
    }
}

fn render_attribution(report: &Report, principals: &PrincipalRegistry) -> Result<String> {
    let lead = match report.authorship.human_report_lead.as_deref() {
        Some(id) => principal_label(principals, id)?,
        None => "Not yet assigned".to_owned(),
    };
    let producers = report
        .review
        .material_producer_ids
        .iter()
        .map(|id| principal_label(principals, id))
        .collect::<Result<Vec<_>>>()?;
    let producers = if producers.is_empty() {
        "None recorded".to_owned()
    } else {
        producers.join(", ")
    };
    Ok(format!(
        "**Authorship and accountability.** Draft authors: {}. Accountable report lead: {lead}. Material producers: {producers}.\n",
        report.authorship.draft_authors.join(", ")
    ))
}

fn principal_label(principals: &PrincipalRegistry, id: &str) -> Result<String> {
    let principal = principals
        .principals
        .iter()
        .filter(|principal| principal.id == id)
        .max_by_key(|principal| principal.record_version)
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown principal '{id}'")))?;
    if principal.affiliations.is_empty() {
        Ok(principal.display_name.clone())
    } else {
        Ok(format!(
            "{} ({})",
            principal.display_name,
            principal.affiliations.join("; ")
        ))
    }
}

fn render_lifecycle(lock: &ReviewLock) -> String {
    let scientific = if lock.approval_lock_root.is_some() {
        "The ordered scientific, reproduction/publication, and assurance-steward approval chain is complete for this exact realization."
    } else {
        "Independent scientific, reproduction/publication, and assurance-steward approval remain pending; no approval lock exists."
    };
    format!(
        "**Assurance status.** This report is `{}`. {scientific} It does not authorize public export, vendoring, or an application-fitness determination.\n",
        lock.lifecycle
    )
}

fn find_table<'a>(report: &'a Report, id: &str) -> Result<&'a Table> {
    report
        .tables
        .iter()
        .find(|table| table.id == id)
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown table '{id}'")))
}

fn render_table(
    table: &Table,
    bindings: &BTreeMap<String, ResolvedValue>,
    usage: &mut Usage,
) -> Result<String> {
    validate_metadata_text(&table.title)?;
    validate_metadata_text(&table.caption)?;
    validate_metadata_text(&table.alternative_text)?;
    validate_metadata_text(&table.row_header)?;
    let mut output = format!(
        "**{}.** {}\n\n| {} |",
        markdown_escape(&table.title),
        markdown_escape(&table.caption),
        markdown_escape(&table.row_header)
    );
    for column in &table.columns {
        validate_metadata_text(&column.label)?;
        let label = match &column.unit_id {
            RequiredNullable::Value(unit_id) => {
                let symbol = bindings
                    .values()
                    .find(|binding| binding.unit_id == *unit_id)
                    .map(|binding| binding.unit_symbol.as_str())
                    .ok_or_else(|| {
                        AssuranceError::Invalid(format!(
                            "table '{}' has an unresolved unit '{}'",
                            table.id, unit_id
                        ))
                    })?;
                format!("{} (`{symbol}`)", markdown_escape(&column.label))
            }
            RequiredNullable::Null => markdown_escape(&column.label),
            RequiredNullable::Missing => {
                return Err(AssuranceError::Invalid(format!(
                    "table '{}' column unit_id is missing",
                    table.id
                )));
            }
        };
        let _ = write!(output, " {label} |");
    }
    output.push_str("\n| --- |");
    for _ in &table.columns {
        output.push_str(" ---: |");
    }
    output.push('\n');
    for row in &table.rows {
        validate_metadata_text(&row.label)?;
        let _ = write!(output, "| {} |", markdown_escape(&row.label));
        for binding_id in &row.value_binding_ids {
            let binding = bindings.get(binding_id).ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "table '{}' has unknown binding '{}'",
                    table.id, binding_id
                ))
            })?;
            usage.values.insert(binding_id.clone());
            let _ = write!(output, " {} |", binding.rendered);
        }
        output.push('\n');
    }
    let _ = writeln!(
        output,
        "\n*Accessible table summary: {}*",
        markdown_escape(&table.alternative_text)
    );
    Ok(output)
}

fn validate_metadata_text(value: &str) -> Result<()> {
    if value.contains(['\n', '\r']) || value.bytes().any(|byte| byte.is_ascii_control()) {
        return Err(AssuranceError::Invalid(
            "assembly metadata contains unsafe control text".to_owned(),
        ));
    }
    let lowercase = value.to_ascii_lowercase();
    if lowercase.contains("://") || lowercase.contains("www.") || value.contains('@') {
        return Err(AssuranceError::Invalid(
            "assembly metadata cannot introduce an external link".to_owned(),
        ));
    }
    Ok(())
}

fn markdown_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\\' => escaped.push_str("&#92;"),
            '`' | '*' | '_' | '{' | '}' | '[' | ']' | '|' | '!' => {
                escaped.push('\\');
                escaped.push(character);
            }
            _ => escaped.push(character),
        }
    }
    escaped
}

fn render_figure(
    figure: &super::Figure,
    bindings: &BTreeMap<String, ResolvedValue>,
    usage: &mut Usage,
) -> Result<(String, Vec<u8>)> {
    validate_metadata_text(&figure.title)?;
    validate_metadata_text(&figure.caption)?;
    validate_metadata_text(&figure.alternative_text)?;
    let values = figure
        .value_binding_ids
        .iter()
        .map(|id| {
            let value = bindings.get(id).ok_or_else(|| {
                AssuranceError::Invalid(format!(
                    "figure '{}' has unknown binding '{id}'",
                    figure.id
                ))
            })?;
            usage.values.insert(id.clone());
            Ok(value)
        })
        .collect::<Result<Vec<_>>>()?;
    let svg = render_svg(figure, &values)?;
    let unit = values
        .first()
        .map(|value| value.unit_symbol.as_str())
        .ok_or_else(|| AssuranceError::Invalid(format!("figure '{}' is empty", figure.id)))?;
    let mut markdown = format!(
        "![{}](figures/{}.svg)\n\n*Figure: {}*\n\n| Series | Value (`{unit}`) |\n| --- | ---: |\n",
        markdown_escape(&figure.alternative_text),
        figure.id,
        markdown_escape(&figure.caption)
    );
    for value in values {
        validate_metadata_text(&value.title)?;
        let _ = writeln!(
            markdown,
            "| {} | {} |",
            markdown_escape(&value.title),
            value.rendered
        );
    }
    let _ = writeln!(
        markdown,
        "\n*Accessible data alternative: {}*",
        markdown_escape(&figure.alternative_text)
    );
    Ok((markdown, svg.into_bytes()))
}

fn render_svg(figure: &super::Figure, values: &[&ResolvedValue]) -> Result<String> {
    let maximum = values.iter().map(|value| value.value).fold(0.0, f64::max);
    if !maximum.is_finite() || maximum <= 0.0 {
        return Err(AssuranceError::Invalid(format!(
            "figure '{}' has no positive finite magnitude",
            figure.id
        )));
    }
    let height = 130 + 62 * values.len();
    let title = xml_escape(&figure.title);
    let description = xml_escape(&figure.alternative_text);
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" viewBox=\"0 0 900 {height}\">\n  <title>{title}</title>\n  <desc>{description}</desc>\n  <defs>\n    <pattern id=\"solid\" width=\"8\" height=\"8\" patternUnits=\"userSpaceOnUse\"><rect width=\"8\" height=\"8\" fill=\"#d9d9d9\"/></pattern>\n    <pattern id=\"stripe\" width=\"8\" height=\"8\" patternUnits=\"userSpaceOnUse\"><rect width=\"8\" height=\"8\" fill=\"white\"/><path d=\"M0,8 L8,0\" stroke=\"#555\" stroke-width=\"2\"/></pattern>\n  </defs>\n  <rect width=\"900\" height=\"{height}\" fill=\"white\"/>\n  <text x=\"20\" y=\"32\" font-family=\"sans-serif\" font-size=\"20\">{title}</text>\n"
    );
    for (index, value) in values.iter().enumerate() {
        let y = 70 + index * 62;
        let width = 500.0 * value.value / maximum;
        let pattern = if index % 2 == 0 { "solid" } else { "stripe" };
        let label = xml_escape(&value.title);
        let rendered = xml_escape(&value.rendered);
        let unit = xml_escape(&value.unit_symbol);
        let _ = writeln!(
            svg,
            "  <text x=\"20\" y=\"{}\" font-family=\"sans-serif\" font-size=\"14\">{label}</text>",
            y + 18
        );
        let _ = writeln!(
            svg,
            "  <rect x=\"300\" y=\"{y}\" width=\"{width:.6}\" height=\"28\" fill=\"url(#{pattern})\" stroke=\"black\"/>"
        );
        let _ = writeln!(
            svg,
            "  <text x=\"815\" y=\"{}\" text-anchor=\"end\" font-family=\"monospace\" font-size=\"14\">{rendered} {unit}</text>",
            y + 19
        );
    }
    svg.push_str("</svg>\n");
    Ok(svg)
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn render_reference(reference: &super::Reference) -> Result<String> {
    validate_metadata_text(&reference.citation)?;
    let citation = markdown_escape(&reference.citation);
    if let Some(doi) = reference.immutable_identity.strip_prefix("doi:") {
        if doi.is_empty()
            || !doi.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'.' | b'/' | b'_' | b'-' | b'(' | b')')
            })
        {
            return Err(AssuranceError::Invalid(format!(
                "reference '{}' has an unsafe DOI identity",
                reference.id
            )));
        }
        let url_doi = doi.replace('(', "%28").replace(')', "%29");
        return Ok(format!("{citation} [doi:{doi}](https://doi.org/{url_doi})"));
    }
    if reference.immutable_identity.is_empty()
        || !reference.immutable_identity.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'.' | b'/' | b'_' | b'-')
        })
    {
        return Err(AssuranceError::Invalid(format!(
            "reference '{}' has an unsafe immutable identity",
            reference.id
        )));
    }
    Ok(format!("{citation} (`{}`)", reference.immutable_identity))
}

fn validate_usersum_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        || path.starts_with("assurance/reports")
    {
        return Err(AssuranceError::Invalid(
            "usersum link must be a confined non-report route".to_owned(),
        ));
    }
    Ok(())
}

fn validate_usersum_source(root: &Path, path: &Path) -> Result<()> {
    validate_usersum_path(path)?;
    read_regular_confined(root, &Path::new("usersum").join(path)).map(|_| ())
}

fn find_public_research_object<'a>(report: &'a Report, id: &str) -> Result<&'a ResearchObject> {
    let object = report
        .research_objects
        .iter()
        .find(|object| object.id == id)
        .ok_or_else(|| AssuranceError::Invalid(format!("unknown research object '{id}'")))?;
    if object.access != "public_safe" {
        return Err(AssuranceError::Invalid(format!(
            "restricted research object '{id}' cannot be staged"
        )));
    }
    Ok(object)
}

fn research_object_basename(object: &ResearchObject) -> Result<&str> {
    object
        .path
        .as_deref()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| {
            AssuranceError::Invalid(format!(
                "research object '{}' has no portable UTF-8 basename",
                object.id
            ))
        })
}

fn validate_usage(
    report: &Report,
    usage: &Usage,
    bindings: &BTreeMap<String, ResolvedValue>,
) -> Result<()> {
    require_exact_usage(
        "value binding",
        &bindings.keys().cloned().collect(),
        &usage.values,
    )?;
    require_exact_usage(
        "table",
        &report.tables.iter().map(|table| table.id.clone()).collect(),
        &usage.tables,
    )?;
    require_exact_usage(
        "figure",
        &report
            .figures
            .iter()
            .map(|figure| figure.id.clone())
            .collect(),
        &usage.figures,
    )?;
    require_exact_usage(
        "reference",
        &report
            .references
            .iter()
            .map(|reference| reference.id.clone())
            .collect(),
        &usage.references,
    )?;
    require_exact_usage(
        "public research object",
        &report
            .research_objects
            .iter()
            .filter(|object| object.access == "public_safe")
            .map(|object| object.id.clone())
            .collect(),
        &usage.research_objects,
    )
}

fn require_exact_usage(
    kind: &str,
    expected: &BTreeSet<String>,
    observed: &BTreeSet<String>,
) -> Result<()> {
    if expected == observed {
        return Ok(());
    }
    let unused = expected.difference(observed).cloned().collect::<Vec<_>>();
    let unknown = observed.difference(expected).cloned().collect::<Vec<_>>();
    Err(AssuranceError::Invalid(format!(
        "unused {kind} IDs {unused:?}; undeclared {kind} IDs {unknown:?}"
    )))
}

fn stage_research_objects(
    root: &Path,
    report: &Report,
    principals: &PrincipalRegistry,
    review_lock: &ReviewLock,
    usage: &Usage,
    version_root: &Path,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<()> {
    let mut basenames = BTreeSet::new();
    for id in &usage.research_objects {
        let object = find_public_research_object(report, id)?;
        let basename = research_object_basename(object)?;
        if !basenames.insert(basename.to_owned()) {
            return Err(AssuranceError::Invalid(format!(
                "research-object basename collision at '{basename}'"
            )));
        }
        let path = object.path.as_deref().ok_or_else(|| {
            AssuranceError::Invalid(format!("research object '{id}' has no path"))
        })?;
        let digest = object.sha256.as_deref().ok_or_else(|| {
            AssuranceError::Invalid(format!("research object '{id}' has no digest"))
        })?;
        let mut bytes = identified_bytes(root, path, digest)?;
        if path.file_name().and_then(|name| name.to_str()) == Some("agent-assistance-packet.json") {
            bytes = render_agent_packet_governance(&bytes, report, principals, review_lock, path)?;
        }
        insert_output(
            files,
            version_root.join("research-objects").join(basename),
            bytes,
        )?;
    }
    Ok(())
}

fn render_agent_packet_governance(
    bytes: &[u8],
    report: &Report,
    principals: &PrincipalRegistry,
    review_lock: &ReviewLock,
    path: &Path,
) -> Result<Vec<u8>> {
    let mut packet: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|error| AssuranceError::Parse {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;
    let object = packet.as_object_mut().ok_or_else(|| {
        AssuranceError::Invalid("agent-assistance packet is not an object".to_owned())
    })?;
    let lead = match report.authorship.human_report_lead.as_deref() {
        Some(id) => Some(principal_label(principals, id)?),
        None => None,
    };
    object.insert(
        "current_governance".to_owned(),
        serde_json::json!({
            "generated": true,
            "lifecycle": review_lock.lifecycle,
            "accountable_report_lead": lead,
            "material_producers": report.review.material_producer_ids,
            "scientific_approval_complete": review_lock.approval_lock_root.is_some(),
            "public_export_authorized": false,
        }),
    );
    let mut rendered = serde_json::to_vec_pretty(&packet).map_err(|error| {
        AssuranceError::Invalid(format!(
            "agent packet governance serialization failed: {error}"
        ))
    })?;
    rendered.push(b'\n');
    Ok(rendered)
}

fn insert_output(
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
    path: PathBuf,
    bytes: Vec<u8>,
) -> Result<()> {
    match files.entry(path) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(bytes);
            Ok(())
        }
        std::collections::btree_map::Entry::Occupied(entry) => Err(AssuranceError::Invalid(
            format!("assembly output collision at {}", entry.key().display()),
        )),
    }
}

#[derive(Serialize)]
struct BuildManifest<'a> {
    schema_version: u32,
    report_id: &'a str,
    report_version: &'a str,
    source_root_sha256: &'a str,
    assembly_tool: &'static str,
    files: Vec<ManifestFile>,
}

#[derive(Serialize)]
struct ManifestFile {
    path: String,
    sha256: String,
}

fn build_manifest(
    report: &Report,
    source_root_sha256: &str,
    version_root: &Path,
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<Vec<u8>> {
    let rows = files
        .iter()
        .map(|(path, bytes)| {
            let relative = path.strip_prefix(version_root).map_err(|error| {
                AssuranceError::Invalid(format!("failed to relativize build output: {error}"))
            })?;
            Ok(ManifestFile {
                path: relative.to_string_lossy().into_owned(),
                sha256: sha256_bytes(bytes),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let manifest = BuildManifest {
        schema_version: 1,
        report_id: &report.id,
        report_version: &report.version,
        source_root_sha256,
        assembly_tool: ASSEMBLY_TOOL_ID,
        files: rows,
    };
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| AssuranceError::Invalid(format!("build manifest JSON: {error}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

struct StagingRoot {
    absolute: PathBuf,
    directory: ConfinedDirectory,
}

impl StagingRoot {
    fn verify_identity(&self) -> Result<()> {
        self.directory.verify_ambient_identity(&self.absolute)
    }
}

fn prepare_staging_root(
    repository_root: &Path,
    requested: &Path,
    operation: Operation,
) -> Result<StagingRoot> {
    validate_requested_staging_path(requested)?;
    let repository_root = std::fs::canonicalize(repository_root)
        .map_err(|error| AssuranceError::io(repository_root, error))?;
    let absolute = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        repository_root.join(requested)
    };
    validate_staging_location(&repository_root, &absolute)?;
    let directory = ConfinedDirectory::open_ambient(&absolute, operation == Operation::Build)
        .map_err(|error| {
            if operation == Operation::Check && matches!(error, AssuranceError::Io { .. }) {
                AssuranceError::Drift(format!(
                    "staging root is unavailable: {} ({error})",
                    requested.display()
                ))
            } else {
                error
            }
        })?;
    Ok(StagingRoot {
        absolute,
        directory,
    })
}

fn validate_requested_staging_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(AssuranceError::Invalid(
            "staging root must be a nonempty path without '..'".to_owned(),
        ));
    }
    Ok(())
}

fn validate_staging_location(repository_root: &Path, staging_root: &Path) -> Result<()> {
    let Ok(relative) = staging_root.strip_prefix(repository_root) else {
        return Ok(());
    };
    let parts = relative
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => value.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>();
    let under_target = parts.first() == Some(&"target") && parts.len() > 1;
    let under_package_artifacts = parts.len() > 4
        && parts[0] == "docs"
        && parts[1] == "work-packages"
        && parts[3] == "artifacts";
    if under_target || under_package_artifacts {
        return Ok(());
    }
    Err(AssuranceError::Invalid(format!(
        "staging root cannot target tracked repository sources or public outputs: {}",
        staging_root.display()
    )))
}

struct StagedCommit {
    target: PathBuf,
    temporary: PathBuf,
    backup: PathBuf,
    restore: PathBuf,
    prior: Option<BTreeMap<PathBuf, Vec<u8>>>,
}

#[allow(clippy::too_many_arguments)]
fn build_reports_transactionally(
    repository_root: &Path,
    staging: &StagingRoot,
    shared_inputs: &BTreeMap<PathBuf, String>,
    identity: &IdentityLock,
    sources: &[&ReportSource],
    plan: &V2Plan,
    reports: &[RenderedReport],
    post_install: &mut dyn FnMut() -> Result<()>,
) -> Result<()> {
    staging.directory.create_dir_all(Path::new(OUTPUT_PREFIX))?;
    let mut commits = Vec::new();
    for report in reports {
        match prepare_report_commit(staging, report) {
            Ok(commit) => commits.push(commit),
            Err(error) => {
                let current_cleanup =
                    cleanup_report_working_directories(staging, &report.summary.id);
                return Err(combine_restoration_error(
                    combine_restoration_error(error, current_cleanup),
                    cleanup_uncommitted(staging, &commits),
                ));
            }
        }
    }
    if let Err(error) =
        verify_assembly_inputs(repository_root, shared_inputs, identity, sources, plan)
    {
        return Err(combine_restoration_error(
            error,
            cleanup_uncommitted(staging, &commits),
        ));
    }
    for (installed, commit) in commits.iter().enumerate() {
        if let Err(error) = install_commit(staging, commit) {
            let restoration = restore_commits(staging, &commits[..installed]);
            let cleanup = cleanup_uncommitted(staging, &commits[installed..]);
            return Err(combine_restoration_error(
                combine_restoration_error(error, restoration),
                cleanup,
            ));
        }
    }
    if let Err(error) = post_install() {
        let restoration = restore_commits(staging, &commits);
        return Err(combine_restoration_error(error, restoration));
    }
    if let Err(error) =
        verify_assembly_inputs(repository_root, shared_inputs, identity, sources, plan)
    {
        let restoration = restore_commits(staging, &commits);
        return Err(combine_restoration_error(error, restoration));
    }
    for report in reports {
        if let Err(error) = check_rendered_report(staging, report, true) {
            let restoration = restore_commits(staging, &commits);
            return Err(combine_restoration_error(error, restoration));
        }
    }
    for commit in &commits {
        if let Err(error) = staging.directory.remove_directory_if_exists(&commit.backup) {
            let restoration = restore_commits(staging, &commits);
            return Err(combine_restoration_error(error, restoration));
        }
    }
    if let Err(error) =
        verify_assembly_inputs(repository_root, shared_inputs, identity, sources, plan)
            .and_then(|()| staging.verify_identity())
    {
        let restoration = restore_commits(staging, &commits);
        return Err(combine_restoration_error(error, restoration));
    }
    Ok(())
}

fn prepare_report_commit(staging: &StagingRoot, report: &RenderedReport) -> Result<StagedCommit> {
    let parent = Path::new(OUTPUT_PREFIX);
    let temporary = parent.join(format!(".{}.next", report.summary.id));
    let backup = parent.join(format!(".{}.previous", report.summary.id));
    let restore = parent.join(format!(".{}.restore", report.summary.id));
    staging.directory.remove_directory_if_exists(&temporary)?;
    staging.directory.remove_directory_if_exists(&restore)?;
    if staging.directory.directory_exists(&backup)? {
        return Err(AssuranceError::Invalid(format!(
            "staging replacement backup already exists: {}",
            backup.display()
        )));
    }
    let prior = snapshot_directory(staging, &report.report_root)?;
    staging.directory.create_dir_all(&temporary)?;
    for (path, bytes) in &report.files {
        staging.directory.write_new(&temporary.join(path), bytes)?;
    }
    check_expected_files(staging, &temporary, &report.files)?;
    Ok(StagedCommit {
        target: report.report_root.clone(),
        temporary,
        backup,
        restore,
        prior,
    })
}

fn snapshot_directory(
    staging: &StagingRoot,
    target: &Path,
) -> Result<Option<BTreeMap<PathBuf, Vec<u8>>>> {
    if !staging.directory.directory_exists(target)? {
        return Ok(None);
    }
    let files = staging.directory.collect_regular_files(target)?;
    let mut snapshot = BTreeMap::new();
    for path in files {
        snapshot.insert(
            path.clone(),
            staging.directory.read_regular(&target.join(path))?,
        );
    }
    Ok(Some(snapshot))
}

fn install_commit(staging: &StagingRoot, commit: &StagedCommit) -> Result<()> {
    if commit.prior.is_some() {
        staging.directory.rename(&commit.target, &commit.backup)?;
    }
    if let Err(error) = staging.directory.rename(&commit.temporary, &commit.target) {
        if commit.prior.is_some() {
            let restoration = staging.directory.rename(&commit.backup, &commit.target);
            return Err(combine_restoration_error(error, restoration));
        }
        return Err(error);
    }
    Ok(())
}

fn restore_commits(staging: &StagingRoot, commits: &[StagedCommit]) -> Result<()> {
    let mut failures = Vec::new();
    for commit in commits.iter().rev() {
        if let Err(error) = restore_commit(staging, commit) {
            failures.push(error);
        }
    }
    combine_recovery_failures(failures)
}

fn restore_commit(staging: &StagingRoot, commit: &StagedCommit) -> Result<()> {
    staging
        .directory
        .remove_directory_if_exists(&commit.restore)?;
    if let Some(prior) = &commit.prior {
        staging.directory.create_dir_all(&commit.restore)?;
        for (path, bytes) in prior {
            staging
                .directory
                .write_new(&commit.restore.join(path), bytes)?;
        }
    }
    staging
        .directory
        .remove_directory_if_exists(&commit.temporary)?;
    if staging.directory.directory_exists(&commit.target)? {
        staging
            .directory
            .rename(&commit.target, &commit.temporary)?;
    }
    if commit.prior.is_some() {
        staging.directory.rename(&commit.restore, &commit.target)?;
    }
    staging
        .directory
        .remove_directory_if_exists(&commit.temporary)?;
    staging
        .directory
        .remove_directory_if_exists(&commit.backup)?;
    Ok(())
}

fn cleanup_uncommitted(staging: &StagingRoot, commits: &[StagedCommit]) -> Result<()> {
    let mut failures = Vec::new();
    for commit in commits {
        if let Err(error) = staging
            .directory
            .remove_directory_if_exists(&commit.temporary)
        {
            failures.push(error);
        }
        if let Err(error) = staging
            .directory
            .remove_directory_if_exists(&commit.restore)
        {
            failures.push(error);
        }
    }
    combine_recovery_failures(failures)
}

fn combine_restoration_error(primary: AssuranceError, restoration: Result<()>) -> AssuranceError {
    match restoration {
        Ok(()) => primary,
        Err(recovery) => AssuranceError::Recovery {
            primary: Box::new(primary),
            recovery: Box::new(recovery),
        },
    }
}

fn cleanup_report_working_directories(staging: &StagingRoot, report_id: &str) -> Result<()> {
    let parent = Path::new(OUTPUT_PREFIX);
    let paths = [
        parent.join(format!(".{report_id}.next")),
        parent.join(format!(".{report_id}.restore")),
    ];
    let mut failures = Vec::new();
    for path in paths {
        if let Err(error) = staging.directory.remove_directory_if_exists(&path) {
            failures.push(error);
        }
    }
    combine_recovery_failures(failures)
}

fn combine_recovery_failures(failures: Vec<AssuranceError>) -> Result<()> {
    let mut failures = failures.into_iter();
    let Some(first) = failures.next() else {
        return Ok(());
    };
    let combined = failures.fold(first, |primary, recovery| AssuranceError::Recovery {
        primary: Box::new(primary),
        recovery: Box::new(recovery),
    });
    Err(combined)
}

fn check_rendered_report(
    staging: &StagingRoot,
    report: &RenderedReport,
    require_complete_links: bool,
) -> Result<()> {
    staging
        .directory
        .ensure_directory(&report.report_root)
        .map_err(|_| {
            AssuranceError::Drift(format!(
                "staged report is missing: {}",
                report.report_root.display()
            ))
        })?;
    check_expected_files(staging, &report.report_root, &report.files)?;
    if require_complete_links {
        for (path, bytes) in &report.files {
            if path.extension().is_some_and(|extension| extension == "md") {
                validate_rendered_links(staging, &report.report_root.join(path), bytes)?;
            }
        }
    }
    Ok(())
}

fn check_expected_files(
    staging: &StagingRoot,
    target: &Path,
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<()> {
    let observed = staging.directory.collect_regular_files(target)?;
    let expected = files.keys().cloned().collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(AssuranceError::Drift(format!(
            "staged report output set differs at '{}': expected {expected:?}, observed {observed:?}",
            target.display()
        )));
    }
    for (path, bytes) in files {
        let observed_bytes = staging.directory.read_regular(&target.join(path))?;
        if observed_bytes != *bytes {
            return Err(AssuranceError::Drift(format!(
                "staged report output is stale: {}",
                target.join(path).display()
            )));
        }
    }
    Ok(())
}

fn validate_rendered_links(
    staging: &StagingRoot,
    markdown_path: &Path,
    bytes: &[u8],
) -> Result<()> {
    let text = std::str::from_utf8(bytes).map_err(|error| {
        AssuranceError::Invalid(format!("rendered Markdown is not UTF-8: {error}"))
    })?;
    for destination in markdown_destinations(text)? {
        if destination.starts_with("https://") {
            continue;
        }
        if destination.contains('#') || destination.starts_with('/') {
            return Err(AssuranceError::Invalid(format!(
                "rendered link is not a supported portable local path: {destination}"
            )));
        }
        let parent = markdown_path
            .parent()
            .ok_or_else(|| AssuranceError::Invalid("rendered Markdown has no parent".to_owned()))?;
        let target = resolve_staged_link(parent, Path::new(destination))?;
        if !target.starts_with("usersum") {
            return Err(AssuranceError::Invalid(format!(
                "rendered local link escapes the usersum consumer: {}",
                target.display()
            )));
        }
        staging.directory.read_regular(&target).map_err(|error| {
            AssuranceError::Drift(format!(
                "rendered local link is unresolved: {} ({error})",
                target.display()
            ))
        })?;
    }
    Ok(())
}

fn resolve_staged_link(parent: &Path, destination: &Path) -> Result<PathBuf> {
    let mut components = parent
        .components()
        .map(|component| match component {
            Component::Normal(value) => Ok(value.to_owned()),
            _ => Err(AssuranceError::Invalid(
                "rendered Markdown parent is not confined".to_owned(),
            )),
        })
        .collect::<Result<Vec<_>>>()?;
    for component in destination.components() {
        match component {
            Component::Normal(value) => components.push(value.to_owned()),
            Component::CurDir => {}
            Component::ParentDir if components.pop().is_some() => {}
            Component::ParentDir => {
                return Err(AssuranceError::Invalid(
                    "rendered local link escapes the staging root".to_owned(),
                ));
            }
            _ => {
                return Err(AssuranceError::Invalid(
                    "rendered local link is not a confined relative path".to_owned(),
                ));
            }
        }
    }
    Ok(components.into_iter().collect())
}

pub(super) fn has_canonical_markdown_link(markdown: &str, target: &str) -> bool {
    use pulldown_cmark::{Event, Options, Parser, Tag};

    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    Parser::new_ext(markdown, options).any(|event| {
        matches!(
            event,
            Event::Start(Tag::Link { dest_url, .. }) if dest_url.as_ref() == target
        )
    })
}

fn markdown_destinations(text: &str) -> Result<Vec<&str>> {
    let mut destinations = Vec::new();
    let mut remaining = text;
    while let Some(open) = remaining.find("](") {
        let after = &remaining[open + 2..];
        let close = after.find(')').ok_or_else(|| {
            AssuranceError::Invalid("rendered Markdown contains an unterminated link".to_owned())
        })?;
        let destination = &after[..close];
        if destination.is_empty() {
            return Err(AssuranceError::Invalid(
                "rendered Markdown contains an empty link".to_owned(),
            ));
        }
        destinations.push(destination);
        remaining = &after[close + 1..];
    }
    Ok(destinations)
}

fn verify_shared_inputs(root: &Path, inputs: &BTreeMap<PathBuf, String>) -> Result<()> {
    for (path, expected) in inputs {
        if sha256_bytes(&read_regular_confined(root, path)?) != *expected {
            return Err(AssuranceError::Drift(format!(
                "v2 assurance input changed during assembly: {}",
                path.display()
            )));
        }
    }
    let _ = digest_input_set("assembly-shared-inputs:1", inputs);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write as _;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{Operation, execute_with_post_install};
    use crate::v2::V2Repository;

    const REPORT_ID: &str = "linear-groundwater-reservoir-recurrence";
    const RESULT_PATH: &str = "assurance/v2/reports/linear-groundwater-reservoir-recurrence/results/two-day-recurrence.json";
    const OUTPUT_BASE: &str =
        "usersum/assurance/reports/linear-groundwater-reservoir-recurrence/1.0.0";

    #[test]
    fn post_install_source_drift_restores_prior_bytes_without_timing() {
        let source = fixture("assurance-assembly-post-install-drift");
        let stage = prepared_stage("assurance-assembly-post-install-stage");
        let repository = V2Repository::open(&source.path).expect("open source fixture");
        repository
            .build_report(REPORT_ID, &stage.path)
            .expect("build prior selected bytes");
        let prior = collect_files(&stage.path.join(OUTPUT_BASE));
        let plan = repository
            .plan_report(REPORT_ID)
            .expect("plan current source");
        let report_source = repository.sources.get(REPORT_ID).expect("selected source");
        let result = source.path.join(RESULT_PATH);
        let mut mutate_source = || {
            fs::OpenOptions::new()
                .append(true)
                .open(&result)
                .and_then(|mut file| file.write_all(b"\n"))
                .map_err(|error| crate::AssuranceError::io(&result, error))
        };
        let error = execute_with_post_install(
            &source.path,
            &stage.path,
            &repository.inputs,
            &repository.identity,
            &repository.principals,
            &[report_source],
            &plan,
            Operation::Build,
            &mut mutate_source,
        )
        .expect_err("post-install source drift must fail");
        assert!(error.to_string().contains("changed during assembly"));
        assert_eq!(prior, collect_files(&stage.path.join(OUTPUT_BASE)));
        for suffix in ["next", "previous", "restore"] {
            assert!(
                !stage
                    .path
                    .join(format!("usersum/assurance/reports/.{REPORT_ID}.{suffix}"))
                    .exists()
            );
        }
    }

    fn prepared_stage(label: &str) -> Scratch {
        let stage = Scratch::new(label);
        copy_file(
            &repository_root(),
            &stage.path,
            "usersum/hillslope-hydrology-and-sediment-physics.md",
        );
        stage
    }

    fn fixture(label: &str) -> Scratch {
        let source = repository_root();
        let target = Scratch::new(label);
        crate::copy_v2_test_fixture(&source, &target.path).expect("copy exact v2 fixture");
        for relative in [
            "assurance/catalog.yaml",
            "assurance/templates/catalog.md",
            "assurance/generated/wepppy-usersum.yaml",
            "usersum/assurance/README.md",
            "usersum/hillslope-hydrology-and-sediment-physics.md",
        ] {
            copy_file(&source, &target.path, relative);
        }
        target
    }

    fn copy_file(source_root: &Path, target_root: &Path, relative: &str) {
        let target = target_root.join(relative);
        fs::create_dir_all(target.parent().expect("target parent")).expect("create parent");
        fs::copy(source_root.join(relative), target).expect("copy file");
    }

    fn collect_files(root: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
        let mut files = std::collections::BTreeMap::new();
        collect_files_into(root, root, &mut files);
        files
    }

    fn collect_files_into(
        root: &Path,
        directory: &Path,
        files: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>,
    ) {
        for entry in fs::read_dir(directory).expect("read collected tree") {
            let entry = entry.expect("read collected entry");
            if entry.file_type().expect("collected type").is_dir() {
                collect_files_into(root, &entry.path(), files);
            } else {
                files.insert(
                    entry
                        .path()
                        .strip_prefix(root)
                        .expect("relative collected path")
                        .to_path_buf(),
                    fs::read(entry.path()).expect("read collected file"),
                );
            }
        }
    }

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("canonical repository root")
    }

    struct Scratch {
        path: PathBuf,
    }

    impl Scratch {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("openwepp-{label}-{}-{counter}", std::process::id()));
            if path.exists() {
                fs::remove_dir_all(&path).expect("remove stale scratch");
            }
            fs::create_dir_all(&path).expect("create scratch");
            Self { path }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            if self.path.exists() {
                fs::remove_dir_all(&self.path).expect("remove scratch");
            }
        }
    }
}
