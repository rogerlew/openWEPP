use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::error::{AssuranceError, Result};
use crate::graph::{DependencyGraph, Node, NodeKind};
use crate::hash::{hash_named_files, sha256_bytes, sha256_file};
use crate::model::{
    AuthoringAnalysis, Availability, Catalog, CatalogDossier, Dossier, EvidenceManifest, Method,
    Review, VerificationObligation, VerificationStatus,
};
use crate::path::{create_dir_all_no_symlinks, existing_file, safe_output, validate_relative};
use crate::render::{render_dossier, render_export, render_index, render_method, render_worksheet};
use crate::snapshot::{SnapshotResult, create_snapshot};

const CATALOG_PATH: &str = "assurance/catalog.yaml";
const MAX_SOURCE_BYTES: u64 = 2 * 1024 * 1024;
const SCHEMA_PATHS: [&str; 6] = [
    "assurance/schemas/catalog.schema.json",
    "assurance/schemas/method.schema.json",
    "assurance/schemas/dossier.schema.json",
    "assurance/schemas/evidence.schema.json",
    "assurance/schemas/review.schema.json",
    "assurance/schemas/authoring.schema.json",
];
const SCHEMA_IDS: [&str; 6] = [
    "https://openwepp.org/assurance/schema/catalog-v1",
    "https://openwepp.org/assurance/schema/method-v1",
    "https://openwepp.org/assurance/schema/dossier-v1",
    "https://openwepp.org/assurance/schema/evidence-manifest-v1",
    "https://openwepp.org/assurance/schema/review-v1",
    "https://openwepp.org/assurance/schema/authoring-v1",
];
const SCHEMA_SHA256: [&str; 6] = [
    "598872637496ab1a7677ea27ee8acd558400e9a3b80c39da20015d1ffbbfdc24",
    "b078a17cf17505003ae650705ee9e42d29c534fe4c9743d4552d9a300245dc6d",
    "6449d51665b506e26bcaafbc7464256be0d75564c8e93023965f845bcb88b650",
    "2f9a535cb50d63074cdcb28cd6969fdfc2eede32c670ec8d374b460948565833",
    "aef4705f063582e20310a01c2bd709289125b2c213620c826f40d229553dfd76",
    "7a2450ca60492afe4a0fcb846de5042a1bbd13ca88e6188ed9510111411cc3e1",
];
const JSON_SCHEMA_DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

#[derive(Debug, Clone)]
pub enum Selection {
    All,
    Dossier(String),
}

#[derive(Debug, Clone, Default)]
pub struct BuildOptions {
    pub output_root: Option<PathBuf>,
    pub snapshot: Option<String>,
    pub snapshot_root: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub inputs: Vec<(PathBuf, String)>,
    pub outputs: Vec<PathBuf>,
    pub source_roots: BTreeMap<String, String>,
    pub scientific_roots: BTreeMap<String, String>,
    pub review_implications: BTreeMap<String, String>,
    pub review_payloads: BTreeMap<String, String>,
    pub node_fingerprints: BTreeMap<String, String>,
}

impl Plan {
    #[must_use]
    pub fn render(&self) -> String {
        let mut output = String::from("inputs:\n");
        render_plan_inputs(&mut output, &self.inputs);
        output.push_str("outputs:\n");
        render_plan_outputs(&mut output, &self.outputs);
        output.push_str("source_roots:\n");
        render_plan_roots(&mut output, self);
        output.push_str("review_payloads:\n");
        render_plan_digests(&mut output, &self.review_payloads);
        output.push_str("node_fingerprints:\n");
        render_plan_digests(&mut output, &self.node_fingerprints);
        output
    }
}

fn render_plan_inputs(output: &mut String, inputs: &[(PathBuf, String)]) {
    for (path, digest) in inputs {
        let _ = writeln!(output, "  - {} sha256={digest}", path.display());
    }
}

fn render_plan_outputs(output: &mut String, outputs: &[PathBuf]) {
    for path in outputs {
        let _ = writeln!(output, "  - {}", path.display());
    }
}

fn render_plan_roots(output: &mut String, plan: &Plan) {
    for (id, digest) in &plan.source_roots {
        let review = plan
            .review_implications
            .get(id)
            .map_or("unknown", String::as_str);
        let scientific = plan
            .scientific_roots
            .get(id)
            .map_or("unknown", String::as_str);
        let _ = writeln!(
            output,
            "  - {id} publication_sha256={digest} scientific_sha256={scientific} review={review}"
        );
    }
}

fn render_plan_digests(output: &mut String, digests: &BTreeMap<String, String>) {
    for (id, digest) in digests {
        let _ = writeln!(output, "  - {id} sha256={digest}");
    }
}

#[derive(Debug, Clone)]
pub struct BuildResult {
    pub outputs: BTreeMap<PathBuf, String>,
    pub snapshot_manifest: Option<PathBuf>,
    pub snapshot_manifest_sha256: Option<String>,
    pub snapshot_confirmed_existing: bool,
}

#[derive(Debug)]
pub struct Assurance {
    root: PathBuf,
    catalog: Catalog,
    bundles: Vec<Bundle>,
    graph: DependencyGraph,
    tool_identity_paths: Vec<PathBuf>,
    tool_source_sha256: String,
    input_identities: BTreeMap<PathBuf, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct Bundle {
    pub entry: CatalogDossier,
    pub dossier: Dossier,
    pub method: Method,
    pub evidence: EvidenceManifest,
    pub review: Review,
    pub authoring: AuthoringAnalysis,
    pub interpretation: String,
    pub limitations: String,
    pub source_root: String,
    pub scientific_root: String,
}

impl Assurance {
    /// Loads and validates the assurance catalog and all declared source records.
    ///
    /// # Errors
    ///
    /// Returns an error when a source cannot be read, parsed, content-verified,
    /// or contained within the repository and approved roots.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root
            .as_ref()
            .canonicalize()
            .map_err(|error| AssuranceError::io(root.as_ref(), error))?;
        validate_schema_documents(&root)?;
        let discovered_catalog: Catalog = read_yaml(&root, Path::new(CATALOG_PATH))?;
        validate_catalog_header(&root, &discovered_catalog)?;
        let discovered_tool_paths = tool_identity_paths(&root)?;
        let discovered_bundles = load_bundles(&root, &discovered_catalog, &discovered_tool_paths)?;
        let discovered_inputs = complete_input_paths(
            &discovered_catalog,
            &discovered_bundles,
            &discovered_tool_paths,
        );
        let input_identities = capture_input_identities(&root, &discovered_inputs)?;

        // Parse and validate a second time while the complete discovered input set is frozen.
        // This binds the cached model to the same bytes later operations verify.
        validate_schema_documents(&root)?;
        let catalog: Catalog = read_yaml(&root, Path::new(CATALOG_PATH))?;
        validate_catalog_header(&root, &catalog)?;
        let tool_identity_paths = tool_identity_paths(&root)?;
        let tool_source_sha256 = hash_named_files(
            &root,
            &tool_identity_paths,
            "openwepp-assurance-tool-source-v1",
        )?;
        let bundles = load_bundles(&root, &catalog, &tool_identity_paths)?;
        let stable_inputs = complete_input_paths(&catalog, &bundles, &tool_identity_paths);
        if stable_inputs != discovered_inputs {
            return Err(AssuranceError::Drift(
                "assurance input set changed while opening the repository".to_owned(),
            ));
        }
        verify_input_identities_map(&root, &input_identities)?;
        let graph = build_graph(&catalog, &bundles, &tool_identity_paths)?;
        graph.validate()?;
        Ok(Self {
            root,
            catalog,
            bundles,
            graph,
            tool_identity_paths,
            tool_source_sha256,
            input_identities,
        })
    }

    /// Validates one dossier or the complete catalog without writing output.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown selector or invalid review lock.
    pub fn validate(&self, selection: &Selection) -> Result<()> {
        self.verify_input_identities()?;
        let selected = self.selected_bundles(selection)?;
        self.render_outputs(selection)?;
        for bundle in selected {
            crate::authoring::enforce_authoring_lock(&bundle.authoring, bundle.dossier.lifecycle)?;
            crate::review::enforce_review_lock(
                &bundle.review,
                bundle.dossier.lifecycle,
                &bundle.scientific_root,
                &bundle.source_root,
            )?;
        }
        self.verify_input_identities()?;
        Ok(())
    }

    /// Returns ordered transitive inputs, identities, outputs, and review impact.
    ///
    /// # Errors
    ///
    /// Returns an error when selection or an input identity is invalid.
    pub fn plan(&self, selection: &Selection) -> Result<Plan> {
        self.verify_input_identities()?;
        let selected = self.selected_bundles(selection)?;
        self.render_outputs(selection)?;
        let input_paths = self.plan_input_paths(&selected);
        let mut inputs = Vec::with_capacity(input_paths.len());
        for relative in input_paths {
            let absolute = existing_file(&self.root, &relative, "plan input")?;
            inputs.push((relative, sha256_file(&absolute)?));
        }
        let source_roots = selected
            .iter()
            .map(|bundle| {
                (
                    bundle.dossier.dossier_id.clone(),
                    bundle.source_root.clone(),
                )
            })
            .collect();
        let scientific_roots = selected
            .iter()
            .map(|bundle| {
                (
                    bundle.dossier.dossier_id.clone(),
                    bundle.scientific_root.clone(),
                )
            })
            .collect();
        let review_implications = selected
            .iter()
            .map(|bundle| {
                (
                    bundle.dossier.dossier_id.clone(),
                    crate::review::review_implication(
                        &bundle.review,
                        bundle.dossier.lifecycle,
                        &bundle.scientific_root,
                        &bundle.source_root,
                    ),
                )
            })
            .collect();
        let mut review_payloads = BTreeMap::new();
        for bundle in &selected {
            for (review_id, digest) in crate::review::approval_payloads(&bundle.review)? {
                review_payloads
                    .insert(format!("{}:{review_id}", bundle.dossier.dossier_id), digest);
            }
        }
        let graph_roots = graph_roots(&selected);
        let node_fingerprints = self.graph.fingerprints_for(&self.root, &graph_roots)?;
        let plan = Plan {
            inputs,
            outputs: self.output_paths(selection)?,
            source_roots,
            scientific_roots,
            review_implications,
            review_payloads,
            node_fingerprints,
        };
        self.verify_input_identities()?;
        Ok(plan)
    }

    /// Renders selected outputs and optionally creates an immutable snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid inputs, unsafe outputs, review requirements,
    /// write failures, or snapshot conflicts.
    pub fn build(&self, selection: &Selection, options: &BuildOptions) -> Result<BuildResult> {
        validate_build_options(selection, options)?;
        if options.snapshot.is_some()
            && self
                .selected_bundles(selection)?
                .iter()
                .any(|bundle| !bundle.dossier.lifecycle.snapshot_eligible())
        {
            return Err(AssuranceError::Invalid(
                "release snapshots may not contain DRAFT dossiers".to_owned(),
            ));
        }
        self.validate(selection)?;
        let rendered = self.render_outputs(selection)?;
        self.verify_input_identities()?;
        let output_root =
            prepare_output_root(options.output_root.as_deref().unwrap_or(&self.root))?;
        for (relative, bytes) in &rendered {
            let output = safe_output(&output_root, relative, "generated output")?;
            write_output(&output, bytes)?;
        }
        self.verify_input_identities()?;
        let snapshot = self.maybe_snapshot(selection, options, &rendered)?;
        self.verify_input_identities()?;
        Ok(build_result(rendered, snapshot))
    }

    /// Rebuilds in a temporary root and compares with committed generated files.
    ///
    /// # Errors
    ///
    /// Returns an error for any validation failure, output drift, or temporary
    /// filesystem failure.
    pub fn check(&self, selection: &Selection) -> Result<BuildResult> {
        self.validate(selection)?;
        let temporary = create_check_directory()?;
        let options = BuildOptions {
            output_root: Some(temporary.clone()),
            ..BuildOptions::default()
        };
        let result = self.build(selection, &options);
        let checked = match result {
            Ok(result) => self.compare_committed(&temporary, result),
            Err(error) => Err(error),
        };
        if temporary.exists() {
            fs::remove_dir_all(&temporary)
                .map_err(|error| AssuranceError::io(&temporary, error))?;
        }
        checked
    }

    #[must_use]
    pub fn graph(&self) -> &DependencyGraph {
        &self.graph
    }

    fn verify_input_identities(&self) -> Result<()> {
        let observed_tool_paths = tool_identity_paths(&self.root).map_err(|_| {
            AssuranceError::Drift(
                "assurance compiler input set is missing, unsafe, or unreadable after open"
                    .to_owned(),
            )
        })?;
        if observed_tool_paths != self.tool_identity_paths
            || complete_input_paths(&self.catalog, &self.bundles, &observed_tool_paths)
                != self.input_identities.keys().cloned().collect()
        {
            return Err(AssuranceError::Drift(
                "assurance input path set changed after open".to_owned(),
            ));
        }
        verify_input_identities_map(&self.root, &self.input_identities)
    }

    fn selected_bundles(&self, selection: &Selection) -> Result<Vec<&Bundle>> {
        match selection {
            Selection::All => Ok(self.bundles.iter().collect()),
            Selection::Dossier(id) => self
                .bundles
                .iter()
                .find(|bundle| bundle.dossier.dossier_id == *id)
                .map(|bundle| vec![bundle])
                .ok_or_else(|| AssuranceError::Invalid(format!("unknown dossier ID '{id}'"))),
        }
    }

    fn plan_input_paths(&self, selected: &[&Bundle]) -> Vec<PathBuf> {
        let mut paths = BTreeSet::from([PathBuf::from(CATALOG_PATH)]);
        paths.extend(SCHEMA_PATHS.into_iter().map(PathBuf::from));
        paths.extend(self.tool_identity_paths.iter().cloned());
        paths.extend(self.catalog.templates.ordered_paths().into_iter().cloned());
        let selected_ids = selected
            .iter()
            .map(|bundle| bundle.dossier.dossier_id.as_str())
            .collect::<BTreeSet<_>>();
        for bundle in &self.bundles {
            paths.insert(bundle.entry.source.clone());
            paths.insert(bundle.entry.method.clone());
            paths.insert(Path::new("usersum").join(&bundle.dossier.narrative.path));
            if selected_ids.contains(bundle.dossier.dossier_id.as_str()) {
                paths.extend(bundle.entry.source_paths().into_iter().cloned());
                paths.extend(
                    bundle
                        .evidence
                        .entries
                        .iter()
                        .filter_map(|entry| entry.path.clone()),
                );
                paths.extend(
                    bundle
                        .authoring
                        .inputs
                        .iter()
                        .filter(|input| {
                            input.availability == crate::model::AnalysisAvailability::Tracked
                        })
                        .map(|input| input.path.clone()),
                );
                paths.extend(
                    bundle
                        .authoring
                        .accepted_outputs
                        .iter()
                        .map(|output| output.path.clone()),
                );
            }
        }
        paths.into_iter().collect()
    }

    fn output_paths(&self, selection: &Selection) -> Result<Vec<PathBuf>> {
        let mut paths = BTreeSet::from([
            self.catalog.shared_outputs.index.clone(),
            self.catalog.shared_outputs.worksheet.clone(),
            self.catalog.export_output.clone(),
        ]);
        for bundle in self.selected_bundles(selection)? {
            paths.insert(bundle.entry.outputs.dossier.clone());
            paths.insert(bundle.entry.outputs.method.clone());
        }
        Ok(paths.into_iter().collect())
    }

    fn render_outputs(&self, selection: &Selection) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
        let index_template = read_text(&self.root, &self.catalog.templates.index)?;
        let method_template = read_text(&self.root, &self.catalog.templates.method)?;
        let dossier_template = read_text(&self.root, &self.catalog.templates.dossier)?;
        let worksheet_template = read_text(&self.root, &self.catalog.templates.worksheet)?;
        let mut outputs = BTreeMap::new();
        insert_output(
            &mut outputs,
            self.catalog.shared_outputs.index.clone(),
            render_index(&index_template, &self.catalog, &self.bundles)?.into_bytes(),
        )?;
        insert_output(
            &mut outputs,
            self.catalog.shared_outputs.worksheet.clone(),
            render_worksheet(&worksheet_template)?.into_bytes(),
        )?;
        insert_output(
            &mut outputs,
            self.catalog.export_output.clone(),
            render_export(&self.catalog, &self.bundles)?.into_bytes(),
        )?;
        for bundle in self.selected_bundles(selection)? {
            insert_output(
                &mut outputs,
                bundle.entry.outputs.method.clone(),
                render_method(&method_template, bundle)?.into_bytes(),
            )?;
            insert_output(
                &mut outputs,
                bundle.entry.outputs.dossier.clone(),
                render_dossier(&dossier_template, bundle)?.into_bytes(),
            )?;
        }
        self.validate_public_outputs(&outputs)?;
        Ok(outputs)
    }

    fn validate_public_outputs(&self, outputs: &BTreeMap<PathBuf, Vec<u8>>) -> Result<()> {
        let mut documents = BTreeMap::new();
        let mut allowed_paths = BTreeSet::from([
            self.catalog.shared_outputs.index.clone(),
            self.catalog.shared_outputs.worksheet.clone(),
        ]);
        for bundle in &self.bundles {
            allowed_paths.insert(bundle.entry.outputs.dossier.clone());
            allowed_paths.insert(bundle.entry.outputs.method.clone());
            allowed_paths.insert(Path::new("usersum").join(&bundle.dossier.narrative.path));
        }
        for (path, bytes) in outputs {
            if path.starts_with("usersum")
                && path.extension().and_then(|value| value.to_str()) == Some("md")
            {
                let text = String::from_utf8(bytes.clone()).map_err(|error| {
                    AssuranceError::Invalid(format!("generated Markdown is not UTF-8: {error}"))
                })?;
                documents.insert(path.clone(), text);
            }
        }
        for bundle in &self.bundles {
            let path = Path::new("usersum").join(&bundle.dossier.narrative.path);
            allowed_paths.insert(path.clone());
            documents.insert(path.clone(), read_text(&self.root, &path)?);
        }
        crate::publication::validate_public_markdown(&documents, &allowed_paths)
    }

    fn maybe_snapshot(
        &self,
        selection: &Selection,
        options: &BuildOptions,
        generated_files: &BTreeMap<PathBuf, Vec<u8>>,
    ) -> Result<Option<SnapshotResult>> {
        let (Some(id), Some(root)) = (&options.snapshot, &options.snapshot_root) else {
            return Ok(None);
        };
        let bundles = self
            .selected_bundles(selection)?
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();
        let catalog_path = existing_file(
            &self.root,
            Path::new(CATALOG_PATH),
            "snapshot catalog input",
        )?;
        let catalog_sha256 = sha256_file(&catalog_path)?;
        let mut files = generated_files.clone();
        self.verify_input_identities()?;
        for bundle in &bundles {
            let narrative = Path::new("usersum").join(&bundle.dossier.narrative.path);
            let absolute = existing_file(&self.root, &narrative, "snapshot narrative")?;
            let bytes = read_limited(&absolute)?;
            insert_output(&mut files, narrative, bytes)?;
        }
        self.verify_input_identities()?;
        create_snapshot(
            root,
            id,
            &catalog_sha256,
            self.catalog.contract_version,
            &self.tool_source_sha256,
            &bundles,
            &files,
        )
        .map(Some)
    }

    fn compare_committed(&self, temporary: &Path, result: BuildResult) -> Result<BuildResult> {
        let mut drift = Vec::new();
        let expected_inventory = self
            .output_paths(&Selection::All)?
            .into_iter()
            .collect::<BTreeSet<_>>();
        let observed_inventory = generated_output_inventory(&self.root)?;
        for path in expected_inventory.difference(&observed_inventory) {
            drift.push(format!("missing {}", path.display()));
        }
        for path in observed_inventory.difference(&expected_inventory) {
            drift.push(format!("undeclared {}", path.display()));
        }
        for relative in result.outputs.keys() {
            let generated = temporary.join(relative);
            let committed = self.root.join(relative);
            let expected =
                fs::read(&generated).map_err(|error| AssuranceError::io(&generated, error))?;
            match fs::read(&committed) {
                Ok(observed) if observed == expected => {}
                Ok(_) => drift.push(format!("changed {}", relative.display())),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    drift.push(format!("missing {}", relative.display()));
                }
                Err(error) => return Err(AssuranceError::io(&committed, error)),
            }
        }
        if drift.is_empty() {
            Ok(result)
        } else {
            Err(AssuranceError::Drift(format!(
                "generated assurance output drift: {}",
                drift.join(", ")
            )))
        }
    }
}

fn complete_input_paths(
    catalog: &Catalog,
    bundles: &[Bundle],
    tool_identity_paths: &[PathBuf],
) -> BTreeSet<PathBuf> {
    let mut paths = BTreeSet::from([PathBuf::from(CATALOG_PATH)]);
    paths.extend(SCHEMA_PATHS.into_iter().map(PathBuf::from));
    paths.extend(tool_identity_paths.iter().cloned());
    paths.extend(catalog.templates.ordered_paths().into_iter().cloned());
    for bundle in bundles {
        paths.extend(bundle.entry.source_paths().into_iter().cloned());
        paths.insert(Path::new("usersum").join(&bundle.dossier.narrative.path));
        paths.extend(
            bundle
                .evidence
                .entries
                .iter()
                .filter_map(|entry| entry.path.clone()),
        );
        paths.extend(
            bundle
                .authoring
                .inputs
                .iter()
                .filter(|input| input.availability == crate::model::AnalysisAvailability::Tracked)
                .map(|input| input.path.clone()),
        );
        paths.extend(
            bundle
                .authoring
                .accepted_outputs
                .iter()
                .map(|output| output.path.clone()),
        );
    }
    paths
}

fn capture_input_identities(
    root: &Path,
    paths: &BTreeSet<PathBuf>,
) -> Result<BTreeMap<PathBuf, String>> {
    paths
        .iter()
        .map(|relative| {
            let absolute = existing_file(root, relative, "assurance input identity")?;
            Ok((relative.clone(), sha256_file(&absolute)?))
        })
        .collect()
}

fn verify_input_identities_map(root: &Path, identities: &BTreeMap<PathBuf, String>) -> Result<()> {
    for (relative, expected) in identities {
        let observed = existing_file(root, relative, "assurance input identity")
            .and_then(|absolute| sha256_file(&absolute))
            .map_err(|_| {
                AssuranceError::Drift(format!(
                    "assurance input is missing, unsafe, or unreadable after open: {}",
                    relative.display()
                ))
            })?;
        if observed != *expected {
            return Err(AssuranceError::Drift(format!(
                "assurance input changed after open: {}",
                relative.display()
            )));
        }
    }
    Ok(())
}

fn generated_output_inventory(root: &Path) -> Result<BTreeSet<PathBuf>> {
    let mut paths = BTreeSet::new();
    for relative in [
        Path::new("usersum/assurance"),
        Path::new("assurance/generated"),
    ] {
        collect_generated_output_files(root, relative, &mut paths)?;
    }
    Ok(paths)
}

fn collect_generated_output_files(
    root: &Path,
    relative: &Path,
    paths: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    let directory = root.join(relative);
    let metadata = match fs::symlink_metadata(&directory) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(AssuranceError::io(&directory, error)),
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(AssuranceError::Invalid(format!(
            "generated output inventory root is a symlink or non-directory: {}",
            relative.display()
        )));
    }
    let mut entries = fs::read_dir(&directory)
        .map_err(|error| AssuranceError::io(&directory, error))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| AssuranceError::io(&directory, error))?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let file_type = entry
            .file_type()
            .map_err(|error| AssuranceError::io(entry.path(), error))?;
        let child = relative.join(entry.file_name());
        if file_type.is_dir() {
            collect_generated_output_files(root, &child, paths)?;
        } else if file_type.is_file() {
            paths.insert(child);
        } else {
            return Err(AssuranceError::Invalid(format!(
                "generated output inventory contains a symlink or special entry: {}",
                child.display()
            )));
        }
    }
    Ok(())
}

fn load_bundles(
    root: &Path,
    catalog: &Catalog,
    tool_identity_paths: &[PathBuf],
) -> Result<Vec<Bundle>> {
    let mut dossier_ids = BTreeSet::new();
    let mut method_ids = BTreeSet::new();
    let mut document_ids = BTreeSet::from([
        "vendor.openwepp.assurance.index".to_owned(),
        "vendor.openwepp.assurance.application_context".to_owned(),
    ]);
    let mut navigation_keys = BTreeSet::from([
        "assurance.index".to_owned(),
        "assurance.application_context".to_owned(),
    ]);
    let mut outputs = BTreeSet::from([
        catalog.export_output.clone(),
        catalog.shared_outputs.index.clone(),
        catalog.shared_outputs.worksheet.clone(),
    ]);
    let mut bundles = Vec::with_capacity(catalog.dossiers.len());
    for entry in &catalog.dossiers {
        validate_catalog_entry(root, entry, &mut dossier_ids, &mut outputs)?;
        let dossier: Dossier = read_yaml(root, &entry.source)?;
        let method: Method = read_yaml(root, &entry.method)?;
        let evidence: EvidenceManifest = read_yaml(root, &entry.evidence)?;
        let authoring: AuthoringAnalysis = read_yaml(root, &entry.authoring)?;
        let review: Review = read_yaml(root, &entry.review)?;
        let interpretation = read_text(root, &entry.interpretation)?;
        let limitations = read_text(root, &entry.limitations)?;
        validate_bundle(
            root, entry, &dossier, &method, &evidence, &authoring, &review,
        )?;
        if !method_ids.insert(method.method_id.clone()) {
            return Err(AssuranceError::Invalid(format!(
                "duplicate method ID '{}'",
                method.method_id
            )));
        }
        register_public_identity(
            &mut document_ids,
            &mut navigation_keys,
            &format!("vendor.openwepp.assurance.method.{}", method.method_id),
            &format!("assurance.method.{}", method.method_id),
        )?;
        register_public_identity(
            &mut document_ids,
            &mut navigation_keys,
            &format!("vendor.openwepp.assurance.dossier.{}", dossier.dossier_id),
            &format!("assurance.dossier.{}", dossier.dossier_id),
        )?;
        register_public_identity(
            &mut document_ids,
            &mut navigation_keys,
            &dossier.narrative.doc_id,
            &dossier.narrative.nav_key,
        )?;
        let scientific_root = scientific_source_root(root, entry, &dossier)?;
        let source_root =
            publication_source_root(root, catalog, entry, &dossier, tool_identity_paths)?;
        bundles.push(Bundle {
            entry: entry.clone(),
            dossier,
            method,
            evidence,
            review,
            authoring,
            interpretation,
            limitations,
            source_root,
            scientific_root,
        });
    }
    if bundles.is_empty() {
        return Err(AssuranceError::Invalid(
            "assurance catalog must contain at least one dossier".to_owned(),
        ));
    }
    bundles.sort_by(|left, right| left.dossier.dossier_id.cmp(&right.dossier.dossier_id));
    Ok(bundles)
}

fn register_public_identity(
    document_ids: &mut BTreeSet<String>,
    navigation_keys: &mut BTreeSet<String>,
    document_id: &str,
    navigation_key: &str,
) -> Result<()> {
    if document_ids.insert(document_id.to_owned())
        && navigation_keys.insert(navigation_key.to_owned())
    {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "duplicate public document ID or navigation key: {document_id} / {navigation_key}"
        )))
    }
}

fn validate_catalog_header(root: &Path, catalog: &Catalog) -> Result<()> {
    if catalog.schema_version != 1 || catalog.contract_version != 1 {
        return Err(AssuranceError::Invalid(format!(
            "unsupported catalog schema/contract version: {}/{}",
            catalog.schema_version, catalog.contract_version
        )));
    }
    if catalog.generated_root != Path::new("usersum/assurance") {
        return Err(AssuranceError::Invalid(
            "generated_root must be usersum/assurance".to_owned(),
        ));
    }
    validate_generated_path(&catalog.export_output, true)?;
    validate_generated_path(&catalog.shared_outputs.index, false)?;
    validate_generated_path(&catalog.shared_outputs.worksheet, false)?;
    for path in catalog.templates.ordered_paths() {
        validate_assurance_source(path)?;
        existing_file(root, path, "template source")?;
    }
    Ok(())
}

fn validate_catalog_entry(
    root: &Path,
    entry: &CatalogDossier,
    ids: &mut BTreeSet<String>,
    outputs: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    validate_id(&entry.dossier_id, "dossier ID")?;
    validate_version(&entry.dossier_version, "dossier version")?;
    if !ids.insert(entry.dossier_id.clone()) {
        return Err(AssuranceError::Invalid(format!(
            "duplicate dossier ID '{}'",
            entry.dossier_id
        )));
    }
    for path in entry.source_paths() {
        validate_assurance_source(path)?;
        existing_file(root, path, "dossier source")?;
    }
    for path in [&entry.outputs.dossier, &entry.outputs.method] {
        validate_generated_path(path, false)?;
        if !outputs.insert(path.clone()) {
            return Err(AssuranceError::Invalid(format!(
                "generated output collision at {}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn validate_bundle(
    root: &Path,
    entry: &CatalogDossier,
    dossier: &Dossier,
    method: &Method,
    evidence: &EvidenceManifest,
    authoring: &AuthoringAnalysis,
    review: &Review,
) -> Result<()> {
    validate_record_versions(dossier, method, evidence, authoring, review)?;
    if entry.dossier_id != dossier.dossier_id
        || entry.dossier_version != dossier.version
        || entry.lifecycle != dossier.lifecycle
        || dossier.method_id != method.method_id
        || evidence.dossier_id != dossier.dossier_id
        || authoring.dossier_id != dossier.dossier_id
        || authoring.dossier_version != dossier.version
        || review.dossier_id != dossier.dossier_id
        || review.dossier_version != dossier.version
    {
        return Err(AssuranceError::Invalid(format!(
            "catalog/source identity mismatch for '{}'",
            entry.dossier_id
        )));
    }
    validate_id(&method.method_id, "method ID")?;
    validate_version(&method.version, "method version")?;
    validate_date(&dossier.evidence_as_of)?;
    validate_narrative(root, &dossier.narrative)?;
    validate_evidence(root, evidence)?;
    crate::authoring::validate_authoring(root, authoring)?;
    crate::review::validate_review(review)?;
    validate_verification(dossier, evidence)?;
    validate_public_fields(dossier, method, evidence)?;
    validate_required_lists(dossier, method)?;
    if dossier.verification.label() != "PASS" && dossier.empirical.is_favorable() {
        return Err(AssuranceError::Invalid(format!(
            "favorable empirical status requires PASS verification for '{}'",
            dossier.dossier_id
        )));
    }
    Ok(())
}

fn validate_record_versions(
    dossier: &Dossier,
    method: &Method,
    evidence: &EvidenceManifest,
    authoring: &AuthoringAnalysis,
    review: &Review,
) -> Result<()> {
    if [
        dossier.schema_version,
        method.schema_version,
        evidence.schema_version,
        authoring.schema_version,
        review.schema_version,
    ]
    .iter()
    .any(|version| *version != 1)
    {
        return Err(AssuranceError::Invalid(
            "unsupported assurance source schema version".to_owned(),
        ));
    }
    Ok(())
}

fn validate_evidence(root: &Path, evidence: &EvidenceManifest) -> Result<()> {
    let mut ids = BTreeSet::new();
    for entry in &evidence.entries {
        validate_evidence_id(entry, &mut ids)?;
        validate_evidence_entry(root, entry)?;
    }
    Ok(())
}

fn validate_evidence_id<'a>(
    entry: &'a crate::model::EvidenceEntry,
    ids: &mut BTreeSet<&'a String>,
) -> Result<()> {
    validate_id(&entry.evidence_id, "evidence ID")?;
    if !ids.insert(&entry.evidence_id) {
        return Err(AssuranceError::Invalid(format!(
            "duplicate evidence ID '{}'",
            entry.evidence_id
        )));
    }
    Ok(())
}

fn validate_evidence_entry(root: &Path, entry: &crate::model::EvidenceEntry) -> Result<()> {
    match entry.availability {
        Availability::Tracked => validate_tracked_evidence(root, entry),
        Availability::External | Availability::Restricted => validate_external_evidence(entry),
        Availability::Unavailable => validate_unavailable_evidence(entry),
    }
}

fn validate_external_evidence(entry: &crate::model::EvidenceEntry) -> Result<()> {
    validate_untracked_location(entry, "non-tracked evidence")?;
    let digest = entry.sha256.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "external or restricted evidence '{}' requires SHA-256 identity",
            entry.evidence_id
        ))
    })?;
    validate_digest(digest, "external evidence SHA-256")
}

fn validate_unavailable_evidence(entry: &crate::model::EvidenceEntry) -> Result<()> {
    validate_untracked_location(entry, "unavailable evidence")?;
    if let Some(digest) = &entry.sha256 {
        validate_digest(digest, "unavailable evidence SHA-256")?;
    }
    Ok(())
}

fn validate_untracked_location(entry: &crate::model::EvidenceEntry, label: &str) -> Result<()> {
    if entry.location.as_deref().unwrap_or_default().is_empty() || entry.path.is_some() {
        Err(AssuranceError::Invalid(format!(
            "{label} '{}' requires location and forbids path",
            entry.evidence_id
        )))
    } else {
        Ok(())
    }
}

fn validate_tracked_evidence(root: &Path, entry: &crate::model::EvidenceEntry) -> Result<()> {
    if entry.location.is_some() {
        return Err(AssuranceError::Invalid(format!(
            "tracked evidence '{}' must use path, not location",
            entry.evidence_id
        )));
    }
    let path = entry.path.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "tracked evidence '{}' has no path",
            entry.evidence_id
        ))
    })?;
    let expected = entry.sha256.as_deref().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "tracked evidence '{}' has no SHA-256",
            entry.evidence_id
        ))
    })?;
    validate_digest(expected, "evidence SHA-256")?;
    let absolute = existing_file(root, path, "tracked evidence")?;
    let observed = sha256_file(&absolute)?;
    if observed != expected {
        return Err(AssuranceError::Invalid(format!(
            "tracked evidence digest mismatch for '{}': expected {expected}, observed {observed}",
            entry.evidence_id
        )));
    }
    Ok(())
}

fn validate_required_lists(dossier: &Dossier, method: &Method) -> Result<()> {
    let empty = dossier.quantities.is_empty()
        || dossier.tested_domain.is_empty()
        || dossier.verification_obligations.is_empty()
        || dossier.applies_to.is_empty()
        || dossier.unknowns.is_empty()
        || method.quantities.is_empty()
        || method.domain.is_empty()
        || method.datasets.is_empty()
        || method.metrics.is_empty()
        || method.criteria.is_empty()
        || method.uncertainty.is_empty()
        || method.reproduction.is_empty();
    if empty {
        Err(AssuranceError::Invalid(format!(
            "dossier '{}' or method '{}' contains an empty required list",
            dossier.dossier_id, method.method_id
        )))
    } else {
        Ok(())
    }
}

fn validate_verification(dossier: &Dossier, evidence: &EvidenceManifest) -> Result<()> {
    let evidence_ids = evidence
        .entries
        .iter()
        .map(|entry| entry.evidence_id.as_str())
        .collect::<BTreeSet<_>>();
    let mandatory = validate_obligations(dossier, &evidence_ids)?;
    let expected = aggregate_verification(&mandatory, &dossier.dossier_id)?;
    if dossier.verification != expected {
        return Err(AssuranceError::Invalid(format!(
            "aggregate verification for '{}' is {}, but mandatory obligations require {}",
            dossier.dossier_id,
            dossier.verification.label(),
            expected.label()
        )));
    }
    Ok(())
}

fn validate_obligations(
    dossier: &Dossier,
    evidence_ids: &BTreeSet<&str>,
) -> Result<Vec<VerificationStatus>> {
    let mut obligation_ids = BTreeSet::new();
    let mut mandatory = Vec::new();
    for obligation in &dossier.verification_obligations {
        collect_obligation(
            obligation,
            evidence_ids,
            &mut obligation_ids,
            &mut mandatory,
        )?;
    }
    Ok(mandatory)
}

fn collect_obligation<'a>(
    obligation: &'a VerificationObligation,
    evidence_ids: &BTreeSet<&str>,
    obligation_ids: &mut BTreeSet<&'a String>,
    mandatory: &mut Vec<VerificationStatus>,
) -> Result<()> {
    validate_obligation_id(obligation, obligation_ids)?;
    validate_obligation(obligation, evidence_ids)?;
    if obligation.mandatory {
        mandatory.push(obligation.status);
    }
    Ok(())
}

fn validate_obligation_id<'a>(
    obligation: &'a VerificationObligation,
    obligation_ids: &mut BTreeSet<&'a String>,
) -> Result<()> {
    validate_id(&obligation.obligation_id, "verification obligation ID")?;
    if !obligation_ids.insert(&obligation.obligation_id) {
        return Err(AssuranceError::Invalid(format!(
            "duplicate verification obligation '{}'",
            obligation.obligation_id
        )));
    }
    Ok(())
}

fn validate_obligation(
    obligation: &VerificationObligation,
    evidence_ids: &BTreeSet<&str>,
) -> Result<()> {
    validate_obligation_text(obligation)?;
    validate_obligation_execution(obligation)?;
    validate_obligation_evidence(obligation, evidence_ids)
}

fn validate_obligation_text(obligation: &VerificationObligation) -> Result<()> {
    for (value, label) in [
        (&obligation.title, "verification obligation title"),
        (&obligation.realization, "verification realization"),
        (&obligation.requirement, "verification requirement"),
        (&obligation.tolerance, "verification tolerance"),
        (&obligation.result, "verification result"),
    ] {
        crate::publication::validate_public_scalar(value, label)?;
    }
    Ok(())
}

fn validate_obligation_execution(obligation: &VerificationObligation) -> Result<()> {
    validate_obligation_date(obligation)?;
    validate_execution_evidence_requirement(obligation)
}

fn validate_obligation_date(obligation: &VerificationObligation) -> Result<()> {
    if let Some(date) = &obligation.executed_on {
        validate_date(date)?;
    }
    Ok(())
}

fn validate_execution_evidence_requirement(obligation: &VerificationObligation) -> Result<()> {
    if is_executed(obligation.status)
        && (obligation.executed_on.is_none() || obligation.evidence_ids.is_empty())
    {
        return Err(AssuranceError::Invalid(format!(
            "executed verification obligation '{}' requires date and evidence",
            obligation.obligation_id
        )));
    }
    Ok(())
}

const fn is_executed(status: VerificationStatus) -> bool {
    matches!(status, VerificationStatus::Pass | VerificationStatus::Fail)
}

fn validate_obligation_evidence(
    obligation: &VerificationObligation,
    evidence_ids: &BTreeSet<&str>,
) -> Result<()> {
    for evidence_id in &obligation.evidence_ids {
        if !evidence_ids.contains(evidence_id.as_str()) {
            return Err(AssuranceError::Invalid(format!(
                "verification obligation '{}' names unknown evidence '{}'",
                obligation.obligation_id, evidence_id
            )));
        }
    }
    Ok(())
}

fn aggregate_verification(
    mandatory: &[VerificationStatus],
    dossier_id: &str,
) -> Result<VerificationStatus> {
    require_mandatory_verification(mandatory, dossier_id)?;
    Ok(aggregate_verification_status(mandatory))
}

fn require_mandatory_verification(
    mandatory: &[VerificationStatus],
    dossier_id: &str,
) -> Result<()> {
    if mandatory.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "dossier '{dossier_id}' has no mandatory verification obligation"
        )));
    }
    Ok(())
}

fn aggregate_verification_status(mandatory: &[VerificationStatus]) -> VerificationStatus {
    if mandatory.contains(&VerificationStatus::Fail) {
        VerificationStatus::Fail
    } else if mandatory.contains(&VerificationStatus::Blocked) {
        VerificationStatus::Blocked
    } else if mandatory.contains(&VerificationStatus::NotRun) {
        VerificationStatus::NotRun
    } else {
        VerificationStatus::Pass
    }
}

fn validate_public_fields(
    dossier: &Dossier,
    method: &Method,
    evidence: &EvidenceManifest,
) -> Result<()> {
    for (value, label) in [
        (&dossier.title, "dossier title"),
        (&dossier.assessment_owner, "assessment owner"),
        (&dossier.question, "dossier question"),
        (&dossier.summary, "dossier summary"),
        (&dossier.application_boundary, "application boundary"),
        (&dossier.source_identity, "source identity"),
        (&dossier.narrative.title, "narrative title"),
        (&method.title, "method title"),
        (&method.owner, "method owner"),
        (&method.question, "method question"),
    ] {
        crate::publication::validate_public_scalar(value, label)?;
    }
    for (values, label) in [
        (&dossier.quantities, "dossier quantity"),
        (&dossier.tested_domain, "tested domain"),
        (&dossier.applies_to, "application"),
        (&dossier.unknowns, "unknown"),
        (&method.quantities, "method quantity"),
        (&method.domain, "method domain"),
        (&method.datasets, "method dataset"),
        (&method.metrics, "method metric"),
        (&method.criteria, "method criterion"),
        (&method.uncertainty, "method uncertainty"),
        (&method.reproduction, "method reproduction"),
    ] {
        for value in values {
            crate::publication::validate_public_scalar(value, label)?;
        }
    }
    for entry in &evidence.entries {
        crate::publication::validate_public_scalar(&entry.role, "evidence role")?;
        crate::publication::validate_public_scalar(&entry.note, "evidence note")?;
    }
    for obligation in &dossier.verification_obligations {
        for (value, label) in [
            (&obligation.title, "verification title"),
            (&obligation.realization, "verification realization"),
            (&obligation.requirement, "verification requirement"),
            (&obligation.tolerance, "verification tolerance"),
            (&obligation.result, "verification result"),
        ] {
            crate::publication::validate_public_scalar(value, label)?;
        }
    }
    Ok(())
}

fn scientific_source_root(
    root: &Path,
    entry: &CatalogDossier,
    dossier: &Dossier,
) -> Result<String> {
    let paths = vec![
        entry.source.clone(),
        entry.method.clone(),
        entry.evidence.clone(),
        entry.interpretation.clone(),
        entry.limitations.clone(),
        entry.authoring.clone(),
        Path::new("usersum").join(&dossier.narrative.path),
    ];
    hash_named_files(root, &paths, "openwepp-assurance-scientific-source-root-v1")
}

fn publication_source_root(
    root: &Path,
    catalog: &Catalog,
    entry: &CatalogDossier,
    dossier: &Dossier,
    tool_identity_paths: &[PathBuf],
) -> Result<String> {
    let mut paths = vec![
        entry.source.clone(),
        entry.method.clone(),
        entry.evidence.clone(),
        entry.interpretation.clone(),
        entry.limitations.clone(),
        entry.authoring.clone(),
        Path::new("usersum").join(&dossier.narrative.path),
    ];
    paths.extend(catalog.templates.ordered_paths().into_iter().cloned());
    paths.extend(SCHEMA_PATHS.into_iter().map(PathBuf::from));
    paths.extend(tool_identity_paths.iter().cloned());
    let file_root = hash_named_files(
        root,
        &paths,
        "openwepp-assurance-publication-source-files-v1",
    )?;
    Ok(hash_text_fields(&[
        "openwepp-assurance-publication-source-root-v2".to_owned(),
        file_root,
        catalog.schema_version.to_string(),
        catalog.contract_version.to_string(),
        catalog.export_output.to_string_lossy().into_owned(),
        catalog.shared_outputs.index.to_string_lossy().into_owned(),
        catalog
            .shared_outputs
            .worksheet
            .to_string_lossy()
            .into_owned(),
        entry.outputs.dossier.to_string_lossy().into_owned(),
        entry.outputs.method.to_string_lossy().into_owned(),
        entry.review.to_string_lossy().into_owned(),
    ]))
}

fn hash_text_fields(fields: &[String]) -> String {
    let mut bytes = Vec::new();
    for field in fields {
        bytes.extend_from_slice(&(field.len() as u64).to_be_bytes());
        bytes.extend_from_slice(field.as_bytes());
    }
    sha256_bytes(&bytes)
}

fn build_graph(
    catalog: &Catalog,
    bundles: &[Bundle],
    tool_identity_paths: &[PathBuf],
) -> Result<DependencyGraph> {
    let mut graph = DependencyGraph::new(catalog.contract_version, catalog.schema_version);
    graph.insert(node("catalog", NodeKind::Catalog, CATALOG_PATH, vec![]))?;
    for relative in SCHEMA_PATHS {
        graph.insert(Node {
            id: format!("schema:{relative}"),
            kind: NodeKind::Schema,
            path: PathBuf::from(relative),
            dependencies: vec!["catalog".to_owned()],
        })?;
    }
    let schema_dependencies = SCHEMA_PATHS
        .into_iter()
        .map(|relative| format!("schema:{relative}"))
        .collect();
    graph.insert(Node {
        id: "schema:identity".to_owned(),
        kind: NodeKind::Schema,
        path: PathBuf::from("assurance/schemas"),
        dependencies: schema_dependencies,
    })?;
    let mut tool_dependencies = Vec::new();
    for (index, path) in tool_identity_paths.iter().enumerate() {
        let id = format!("tool:file:{index}");
        graph.insert(Node {
            id: id.clone(),
            kind: NodeKind::Tool,
            path: path.clone(),
            dependencies: vec![],
        })?;
        tool_dependencies.push(id);
    }
    graph.insert(Node {
        id: "tool:identity".to_owned(),
        kind: NodeKind::Tool,
        path: PathBuf::from("crates/openwepp-assurance"),
        dependencies: tool_dependencies,
    })?;
    for (id, path) in [
        ("template:index", &catalog.templates.index),
        ("template:method", &catalog.templates.method),
        ("template:dossier", &catalog.templates.dossier),
        ("template:worksheet", &catalog.templates.worksheet),
    ] {
        graph.insert(node(id, NodeKind::Template, path, vec!["catalog"]))?;
    }
    let mut index_dependencies = vec![
        "catalog".to_owned(),
        "template:index".to_owned(),
        "tool:identity".to_owned(),
        "schema:identity".to_owned(),
    ];
    let mut export_dependencies = vec![
        "catalog".to_owned(),
        "tool:identity".to_owned(),
        "schema:identity".to_owned(),
    ];
    for bundle in bundles {
        add_bundle_nodes(&mut graph, bundle)?;
        index_dependencies.push(format!("dossier:{}", bundle.dossier.dossier_id));
        index_dependencies.push(format!("method:{}", bundle.dossier.dossier_id));
        index_dependencies.push(format!("narrative:{}", bundle.dossier.dossier_id));
        export_dependencies.push(format!("dossier:{}", bundle.dossier.dossier_id));
        export_dependencies.push(format!("method:{}", bundle.dossier.dossier_id));
        export_dependencies.push(format!("narrative:{}", bundle.dossier.dossier_id));
    }
    graph.insert(Node {
        id: "output:index".to_owned(),
        kind: NodeKind::PublicOutput,
        path: catalog.shared_outputs.index.clone(),
        dependencies: index_dependencies,
    })?;
    graph.insert(Node {
        id: "output:worksheet".to_owned(),
        kind: NodeKind::PublicOutput,
        path: catalog.shared_outputs.worksheet.clone(),
        dependencies: vec![
            "catalog".to_owned(),
            "template:worksheet".to_owned(),
            "tool:identity".to_owned(),
            "schema:identity".to_owned(),
        ],
    })?;
    graph.insert(Node {
        id: "output:export".to_owned(),
        kind: NodeKind::Export,
        path: catalog.export_output.clone(),
        dependencies: export_dependencies,
    })?;
    Ok(graph)
}

fn add_bundle_nodes(graph: &mut DependencyGraph, bundle: &Bundle) -> Result<()> {
    add_bundle_source_nodes(graph, bundle)?;
    add_bundle_evidence_nodes(graph, bundle)?;
    add_bundle_authoring_nodes(graph, bundle)?;
    add_bundle_output_nodes(graph, bundle)
}

fn add_bundle_source_nodes(graph: &mut DependencyGraph, bundle: &Bundle) -> Result<()> {
    let id = &bundle.dossier.dossier_id;
    for (prefix, kind, path) in [
        ("method", NodeKind::Method, &bundle.entry.method),
        ("dossier", NodeKind::Dossier, &bundle.entry.source),
        (
            "interpretation",
            NodeKind::Interpretation,
            &bundle.entry.interpretation,
        ),
        (
            "limitations",
            NodeKind::Limitations,
            &bundle.entry.limitations,
        ),
        ("review", NodeKind::Review, &bundle.entry.review),
    ] {
        graph.insert(Node {
            id: format!("{prefix}:{id}"),
            kind,
            path: path.clone(),
            dependencies: vec!["catalog".to_owned()],
        })?;
    }
    graph.insert(Node {
        id: format!("narrative:{id}"),
        kind: NodeKind::Narrative,
        path: Path::new("usersum").join(&bundle.dossier.narrative.path),
        dependencies: vec!["catalog".to_owned()],
    })
}

fn add_bundle_evidence_nodes(graph: &mut DependencyGraph, bundle: &Bundle) -> Result<()> {
    let id = &bundle.dossier.dossier_id;
    let mut evidence_dependencies = vec!["catalog".to_owned()];
    for entry in &bundle.evidence.entries {
        if let Some(path) = &entry.path {
            let asset_id = format!("evidence-asset:{id}:{}", entry.evidence_id);
            graph.insert(Node {
                id: asset_id.clone(),
                kind: NodeKind::EvidenceAsset,
                path: path.clone(),
                dependencies: vec![],
            })?;
            evidence_dependencies.push(asset_id);
        }
    }
    graph.insert(Node {
        id: format!("evidence:{id}"),
        kind: NodeKind::EvidenceManifest,
        path: bundle.entry.evidence.clone(),
        dependencies: evidence_dependencies,
    })
}

fn add_bundle_authoring_nodes(graph: &mut DependencyGraph, bundle: &Bundle) -> Result<()> {
    let id = &bundle.dossier.dossier_id;
    let mut authoring_dependencies = vec!["catalog".to_owned()];
    for (index, input) in bundle
        .authoring
        .inputs
        .iter()
        .filter(|input| input.availability == crate::model::AnalysisAvailability::Tracked)
        .enumerate()
    {
        let node_id = format!("authoring-input:{id}:{index}");
        graph.insert(Node {
            id: node_id.clone(),
            kind: NodeKind::AuthoringInput,
            path: input.path.clone(),
            dependencies: vec![],
        })?;
        authoring_dependencies.push(node_id);
    }
    for (index, output) in bundle.authoring.accepted_outputs.iter().enumerate() {
        let node_id = format!("authoring-output:{id}:{index}");
        graph.insert(Node {
            id: node_id.clone(),
            kind: NodeKind::AuthoringOutput,
            path: output.path.clone(),
            dependencies: vec![],
        })?;
        authoring_dependencies.push(node_id);
    }
    graph.insert(Node {
        id: format!("authoring:{id}"),
        kind: NodeKind::AuthoringRecord,
        path: bundle.entry.authoring.clone(),
        dependencies: authoring_dependencies,
    })
}

fn add_bundle_output_nodes(graph: &mut DependencyGraph, bundle: &Bundle) -> Result<()> {
    let id = &bundle.dossier.dossier_id;
    graph.insert(Node {
        id: format!("output:method:{id}"),
        kind: NodeKind::PublicOutput,
        path: bundle.entry.outputs.method.clone(),
        dependencies: vec![
            format!("method:{id}"),
            "template:method".to_owned(),
            "tool:identity".to_owned(),
            "schema:identity".to_owned(),
        ],
    })?;
    graph.insert(Node {
        id: format!("output:dossier:{id}"),
        kind: NodeKind::PublicOutput,
        path: bundle.entry.outputs.dossier.clone(),
        dependencies: vec![
            format!("dossier:{id}"),
            format!("method:{id}"),
            format!("evidence:{id}"),
            format!("interpretation:{id}"),
            format!("limitations:{id}"),
            format!("authoring:{id}"),
            format!("review:{id}"),
            format!("narrative:{id}"),
            "template:index".to_owned(),
            "template:method".to_owned(),
            "template:dossier".to_owned(),
            "template:worksheet".to_owned(),
            "tool:identity".to_owned(),
            "schema:identity".to_owned(),
        ],
    })?;
    Ok(())
}

fn graph_roots(selected: &[&Bundle]) -> Vec<String> {
    let mut roots = vec![
        "output:index".to_owned(),
        "output:worksheet".to_owned(),
        "output:export".to_owned(),
    ];
    for bundle in selected {
        roots.push(format!("output:method:{}", bundle.dossier.dossier_id));
        roots.push(format!("output:dossier:{}", bundle.dossier.dossier_id));
    }
    roots
}

fn node(id: &str, kind: NodeKind, path: impl Into<PathBuf>, dependencies: Vec<&str>) -> Node {
    Node {
        id: id.to_owned(),
        kind,
        path: path.into(),
        dependencies: dependencies.into_iter().map(str::to_owned).collect(),
    }
}

fn validate_schema_documents(root: &Path) -> Result<()> {
    for ((relative, expected_id), expected_sha256) in
        SCHEMA_PATHS.into_iter().zip(SCHEMA_IDS).zip(SCHEMA_SHA256)
    {
        let path = existing_file(root, Path::new(relative), "schema")?;
        let bytes = read_limited(&path)?;
        let observed_sha256 = sha256_bytes(&bytes);
        if observed_sha256 != expected_sha256 {
            return Err(AssuranceError::Invalid(format!(
                "schema bytes do not match the compiler-bound v1 identity for {relative}: expected {expected_sha256}, observed {observed_sha256}"
            )));
        }
        let value: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
                path: path.clone(),
                message: error.to_string(),
            })?;
        if value.get("$schema").and_then(serde_json::Value::as_str) != Some(JSON_SCHEMA_DIALECT)
            || value.get("$id").and_then(serde_json::Value::as_str) != Some(expected_id)
            || value
                .pointer("/properties/schema_version/const")
                .and_then(serde_json::Value::as_u64)
                != Some(1)
        {
            return Err(AssuranceError::Invalid(format!(
                "schema dialect, ID, or version binding is invalid: {relative}"
            )));
        }
    }
    Ok(())
}

fn tool_identity_paths(root: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = vec![
        PathBuf::from("Cargo.toml"),
        PathBuf::from("Cargo.lock"),
        PathBuf::from("crates/openwepp-assurance/Cargo.toml"),
    ];
    collect_rust_sources(root, Path::new("crates/openwepp-assurance/src"), &mut paths)?;
    paths.sort();
    paths.dedup();
    for path in &paths {
        existing_file(root, path, "assurance tool identity input")?;
    }
    Ok(paths)
}

fn collect_rust_sources(root: &Path, relative: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    let directory = root.join(relative);
    let mut entries = fs::read_dir(&directory)
        .map_err(|error| AssuranceError::io(&directory, error))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| AssuranceError::io(&directory, error))?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let file_type = entry
            .file_type()
            .map_err(|error| AssuranceError::io(entry.path(), error))?;
        let child = relative.join(entry.file_name());
        if file_type.is_dir() {
            collect_rust_sources(root, &child, paths)?;
        } else if file_type.is_file()
            && child.extension().and_then(|value| value.to_str()) == Some("rs")
        {
            paths.push(child);
        } else if file_type.is_symlink() {
            return Err(AssuranceError::Invalid(format!(
                "assurance tool source may not be a symlink: {}",
                child.display()
            )));
        }
    }
    Ok(())
}

fn read_yaml<T: DeserializeOwned>(root: &Path, relative: &Path) -> Result<T> {
    let path = existing_file(root, relative, "YAML source")?;
    let bytes = read_limited(&path)?;
    serde_yaml::from_slice(&bytes).map_err(|error| AssuranceError::Parse {
        path,
        message: error.to_string(),
    })
}

fn read_text(root: &Path, relative: &Path) -> Result<String> {
    let path = existing_file(root, relative, "text source")?;
    let bytes = read_limited(&path)?;
    String::from_utf8(bytes).map_err(|error| AssuranceError::Parse {
        path,
        message: error.to_string(),
    })
}

fn read_limited(path: &Path) -> Result<Vec<u8>> {
    let file = File::open(path).map_err(|error| AssuranceError::io(path, error))?;
    let mut limited = file.take(MAX_SOURCE_BYTES + 1);
    let mut bytes = Vec::with_capacity(16 * 1024);
    limited
        .read_to_end(&mut bytes)
        .map_err(|error| AssuranceError::io(path, error))?;
    if bytes.len() as u64 > MAX_SOURCE_BYTES {
        return Err(AssuranceError::Invalid(format!(
            "assurance source exceeds {MAX_SOURCE_BYTES} bytes: {}",
            path.display()
        )));
    }
    Ok(bytes)
}

fn validate_assurance_source(path: &Path) -> Result<()> {
    validate_relative(path, "assurance source")?;
    if !path.starts_with("assurance") || path.starts_with("assurance/generated") {
        return Err(AssuranceError::Invalid(format!(
            "source must be under assurance/ and outside generated/: {}",
            path.display()
        )));
    }
    Ok(())
}

fn validate_generated_path(path: &Path, export: bool) -> Result<()> {
    validate_relative(path, "generated output")?;
    let valid = if export {
        path.starts_with("assurance/generated")
    } else {
        path.starts_with("usersum/assurance")
    };
    if valid {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "generated output is outside its approved root: {}",
            path.display()
        )))
    }
}

fn validate_narrative(root: &Path, narrative: &crate::model::NarrativeRef) -> Result<()> {
    validate_relative(&narrative.path, "usersum narrative")?;
    if narrative.path.starts_with("assurance") {
        return Err(AssuranceError::Invalid(
            "hand-authored narrative may not occupy the generated usersum/assurance root"
                .to_owned(),
        ));
    }
    if narrative.path.extension().and_then(|value| value.to_str()) != Some("md") {
        return Err(AssuranceError::Invalid(
            "usersum narrative must be Markdown".to_owned(),
        ));
    }
    validate_public_key(&narrative.doc_id, "narrative document ID")?;
    validate_public_key(&narrative.nav_key, "narrative navigation key")?;
    existing_file(
        root,
        &Path::new("usersum").join(&narrative.path),
        "usersum narrative",
    )?;
    Ok(())
}

fn validate_public_key(value: &str, label: &str) -> Result<()> {
    let valid = !value.is_empty()
        && !value.starts_with(['.', '_', '-'])
        && !value.ends_with(['.', '_', '-'])
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        });
    if valid {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "invalid {label}: '{value}'"
        )))
    }
}

pub(crate) fn validate_id(value: &str, label: &str) -> Result<()> {
    let valid = !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.contains("--");
    if valid {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "invalid {label}: '{value}'"
        )))
    }
}

fn validate_version(value: &str, label: &str) -> Result<()> {
    let parts = value.split('.').collect::<Vec<_>>();
    let valid = parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()));
    if valid {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "invalid {label}: '{value}'"
        )))
    }
}

pub(crate) fn validate_date(value: &str) -> Result<()> {
    let bytes = value.as_bytes();
    let shape = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit());
    let valid = shape
        && value[0..4].parse::<u32>().is_ok_and(|year| year > 0)
        && value[5..7]
            .parse::<u32>()
            .is_ok_and(|month| (1..=12).contains(&month))
        && valid_day(value);
    if valid {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "invalid evidence as-of date: '{value}'"
        )))
    }
}

fn valid_day(value: &str) -> bool {
    let Ok(year) = value[0..4].parse::<u32>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u32>() else {
        return false;
    };
    let Ok(day) = value[8..10].parse::<u32>() else {
        return false;
    };
    let divisible_by = |divisor| year.checked_rem(divisor) == Some(0);
    let leap = divisible_by(4) && (!divisible_by(100) || divisible_by(400));
    let maximum = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => return false,
    };
    (1..=maximum).contains(&day)
}

pub(crate) fn validate_digest(value: &str, label: &str) -> Result<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(AssuranceError::Invalid(format!(
            "invalid {label}: '{value}'"
        )))
    }
}

fn validate_build_options(selection: &Selection, options: &BuildOptions) -> Result<()> {
    if options.snapshot.is_some() != options.snapshot_root.is_some() {
        return Err(AssuranceError::Usage(
            "--snapshot and --snapshot-root must be supplied together".to_owned(),
        ));
    }
    if options.snapshot.is_some() && !matches!(selection, Selection::All) {
        return Err(AssuranceError::Usage(
            "release snapshots require build --all".to_owned(),
        ));
    }
    if let Some(snapshot_id) = &options.snapshot {
        crate::path::validate_snapshot_id(snapshot_id)?;
    }
    Ok(())
}

fn insert_output(
    outputs: &mut BTreeMap<PathBuf, Vec<u8>>,
    path: PathBuf,
    bytes: Vec<u8>,
) -> Result<()> {
    if outputs.contains_key(&path) {
        return Err(AssuranceError::Invalid(format!(
            "generated output collision at {}",
            path.display()
        )));
    }
    outputs.insert(path, bytes);
    Ok(())
}

fn prepare_output_root(path: &Path) -> Result<PathBuf> {
    create_dir_all_no_symlinks(path, "generated output root")?;
    path.canonicalize()
        .map_err(|error| AssuranceError::io(path, error))
}

fn write_output(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().ok_or_else(|| {
        AssuranceError::Invalid(format!(
            "generated output has no parent: {}",
            path.display()
        ))
    })?;
    create_dir_all_no_symlinks(parent, "generated output parent")?;
    fs::write(path, bytes).map_err(|error| AssuranceError::io(path, error))
}

fn build_result(
    rendered: BTreeMap<PathBuf, Vec<u8>>,
    snapshot: Option<SnapshotResult>,
) -> BuildResult {
    let outputs = rendered
        .into_iter()
        .map(|(path, bytes)| (path, sha256_bytes(&bytes)))
        .collect();
    BuildResult {
        outputs,
        snapshot_manifest: snapshot.as_ref().map(|value| value.manifest_path.clone()),
        snapshot_manifest_sha256: snapshot.as_ref().map(|value| value.manifest_sha256.clone()),
        snapshot_confirmed_existing: snapshot.is_some_and(|value| value.confirmed_existing),
    }
}

fn create_check_directory() -> Result<PathBuf> {
    let base = std::env::temp_dir();
    for counter in 0..100_u32 {
        let path = base.join(format!(
            "openwepp-assurance-check-{}-{counter}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(AssuranceError::io(path, error)),
        }
    }
    Err(AssuranceError::Invalid(
        "could not allocate a temporary check directory".to_owned(),
    ))
}
