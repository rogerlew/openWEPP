use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AssuranceError, Result};
use crate::hash::sha256_bytes;

const CATALOG_PATH: &str = "assurance/catalog.yaml";
const EXPECTED_STATE: &str = "v1_retired_zero_reports";
const EXPECTED_GENERATED_ROOT: &str = "usersum/assurance";
const EXPECTED_EXPORT: &str = "assurance/generated/wepppy-usersum.yaml";
const EXPECTED_TEMPLATE: &str = "assurance/templates/catalog.md";
const PUBLIC_CATALOG: &str = "usersum/assurance/README.md";
const RETIRED_PATHS: &[&str] = &[
    "assurance/dossiers",
    "assurance/methods",
    "assurance/schemas",
    "assurance/templates/application-context-worksheet.md",
    "assurance/templates/dossier.md",
    "assurance/templates/method.md",
    "usersum/assurance/application-context-worksheet.md",
    "usersum/assurance/dossiers",
    "usersum/assurance/methods",
];

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuildOptions {
    pub output_root: Option<PathBuf>,
    pub snapshot: Option<String>,
    pub snapshot_root: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct BuildResult {
    pub outputs: BTreeMap<PathBuf, String>,
    pub snapshot_manifest: Option<PathBuf>,
    pub snapshot_manifest_sha256: Option<String>,
    pub snapshot_confirmed_existing: bool,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub publication_state: String,
    pub inputs: BTreeMap<PathBuf, String>,
    pub outputs: BTreeMap<PathBuf, String>,
}

impl Plan {
    #[must_use]
    pub fn render(&self) -> String {
        let mut output = format!(
            "plan: PASS\npublication_state: {}\nreports: 0\ninputs:\n",
            self.publication_state
        );
        append_digest_rows(&mut output, &self.inputs);
        output.push_str("outputs:\n");
        append_digest_rows(&mut output, &self.outputs);
        output
    }
}

fn append_digest_rows(output: &mut String, rows: &BTreeMap<PathBuf, String>) {
    use std::fmt::Write as _;
    for (path, digest) in rows {
        let _ = writeln!(output, "  - {} sha256={digest}", path.display());
    }
}

#[derive(Debug)]
pub struct Assurance {
    root: PathBuf,
    catalog: Catalog,
    template: Vec<u8>,
    inputs: BTreeMap<PathBuf, String>,
}

impl Assurance {
    /// Loads and validates the deliberately empty ASSURE-03 transition source.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed, nonempty, unsafe, stale, or retired v1
    /// source/public state.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root
            .as_ref()
            .canonicalize()
            .map_err(|error| AssuranceError::io(root.as_ref(), error))?;
        reject_retired_paths(&root)?;
        let catalog_bytes = read_confined(&root, Path::new(CATALOG_PATH))?;
        let catalog: Catalog =
            serde_yaml::from_slice(&catalog_bytes).map_err(|error| AssuranceError::Parse {
                path: PathBuf::from(CATALOG_PATH),
                message: error.to_string(),
            })?;
        validate_catalog(&catalog)?;
        let template = read_confined(&root, &catalog.template)?;
        validate_template(&template)?;
        let inputs = BTreeMap::from([
            (PathBuf::from(CATALOG_PATH), sha256_bytes(&catalog_bytes)),
            (catalog.template.clone(), sha256_bytes(&template)),
        ]);
        Ok(Self {
            root,
            catalog,
            template,
            inputs,
        })
    }

    /// Revalidates the frozen zero-report source.
    ///
    /// # Errors
    ///
    /// Returns an error when an input changed after `open` or a retired route
    /// appeared.
    pub fn validate(&self) -> Result<()> {
        reject_retired_paths(&self.root)?;
        self.verify_inputs()
    }

    /// Plans the complete two-output zero-report build.
    ///
    /// # Errors
    ///
    /// Returns an error when the frozen source drifted.
    pub fn plan(&self) -> Result<Plan> {
        self.validate()?;
        Ok(Plan {
            publication_state: self.catalog.publication_state.clone(),
            inputs: self.inputs.clone(),
            outputs: digest_outputs(&self.render_outputs()),
        })
    }

    /// Builds the neutral catalog/export and optionally creates an immutable
    /// zero-report release snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error on source drift, unsafe output/snapshot paths, I/O
    /// failures, or an immutable snapshot conflict.
    pub fn build(&self, options: &BuildOptions) -> Result<BuildResult> {
        self.validate()?;
        validate_snapshot_options(options)?;
        let files = self.render_outputs();
        let output_root = prepare_output_root(options.output_root.as_deref(), &self.root)?;
        for (path, bytes) in &files {
            write_confined(&output_root, path, bytes)?;
        }
        self.verify_inputs()?;
        let mut result = BuildResult {
            outputs: digest_outputs(&files),
            snapshot_manifest: None,
            snapshot_manifest_sha256: None,
            snapshot_confirmed_existing: false,
        };
        if let (Some(snapshot_id), Some(snapshot_root)) =
            (&options.snapshot, &options.snapshot_root)
        {
            let snapshot = create_snapshot(snapshot_root, snapshot_id, &self.inputs, &files)?;
            result.snapshot_manifest = Some(snapshot.manifest_path);
            result.snapshot_manifest_sha256 = Some(snapshot.manifest_sha256);
            result.snapshot_confirmed_existing = snapshot.confirmed_existing;
        }
        Ok(result)
    }

    /// Checks that tracked outputs exactly match a deterministic rebuild and
    /// that no additional public assurance file exists.
    ///
    /// # Errors
    ///
    /// Returns drift when bytes or the public output set differ.
    pub fn check(&self) -> Result<BuildResult> {
        self.validate()?;
        let expected = self.render_outputs();
        for (path, bytes) in &expected {
            let observed = read_confined(&self.root, path)?;
            if observed != *bytes {
                return Err(AssuranceError::Drift(format!(
                    "generated assurance output is stale: {}",
                    path.display()
                )));
            }
        }
        require_exact_public_files(&self.root)?;
        Ok(BuildResult {
            outputs: digest_outputs(&expected),
            snapshot_manifest: None,
            snapshot_manifest_sha256: None,
            snapshot_confirmed_existing: false,
        })
    }

    fn verify_inputs(&self) -> Result<()> {
        for (path, expected) in &self.inputs {
            let observed = sha256_bytes(&read_confined(&self.root, path)?);
            if observed != *expected {
                return Err(AssuranceError::Drift(format!(
                    "assurance input changed after open: {}",
                    path.display()
                )));
            }
        }
        Ok(())
    }

    fn render_outputs(&self) -> BTreeMap<PathBuf, Vec<u8>> {
        BTreeMap::from([
            (self.catalog.shared_output.clone(), self.template.clone()),
            (
                self.catalog.export_output.clone(),
                render_dormant_export().into_bytes(),
            ),
        ])
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    schema_version: u32,
    contract_version: u32,
    publication_state: String,
    generated_root: PathBuf,
    export_output: PathBuf,
    template: PathBuf,
    dossiers: Vec<serde_yaml::Value>,
    shared_output: PathBuf,
}

fn validate_catalog(catalog: &Catalog) -> Result<()> {
    if catalog.schema_version != 1 || catalog.contract_version != 2 {
        return Err(AssuranceError::Invalid(
            "zero-report catalog requires schema_version 1 and contract_version 2".to_owned(),
        ));
    }
    if catalog.publication_state != EXPECTED_STATE {
        return Err(AssuranceError::Invalid(format!(
            "publication_state must be '{EXPECTED_STATE}'"
        )));
    }
    validate_catalog_paths(catalog)?;
    if !catalog.dossiers.is_empty() {
        return Err(AssuranceError::Invalid(
            "v1 assurance dossiers are retired; active catalog must contain zero reports"
                .to_owned(),
        ));
    }
    Ok(())
}

fn validate_catalog_paths(catalog: &Catalog) -> Result<()> {
    let expected = [
        (&catalog.generated_root, EXPECTED_GENERATED_ROOT),
        (&catalog.export_output, EXPECTED_EXPORT),
        (&catalog.template, EXPECTED_TEMPLATE),
        (&catalog.shared_output, PUBLIC_CATALOG),
    ];
    for (observed, required) in expected {
        validate_relative(observed)?;
        if observed != Path::new(required) {
            return Err(AssuranceError::Invalid(format!(
                "zero-report catalog path must be '{required}', observed '{}'",
                observed.display()
            )));
        }
    }
    Ok(())
}

fn validate_template(template: &[u8]) -> Result<()> {
    let text = std::str::from_utf8(template).map_err(|error| {
        AssuranceError::Invalid(format!("catalog template is not UTF-8: {error}"))
    })?;
    for required in [
        "Generated by openwepp-assurance",
        "# Scientific Assurance Reports",
        "No scientific model-evaluation report has completed",
        "does not mean that openWEPP processes lack",
    ] {
        if !text.contains(required) {
            return Err(AssuranceError::Invalid(format!(
                "neutral catalog template is missing required text: {required}"
            )));
        }
    }
    if text.contains("{{") {
        return Err(AssuranceError::Invalid(
            "zero-report catalog template cannot contain renderer placeholders".to_owned(),
        ));
    }
    Ok(())
}

fn render_dormant_export() -> String {
    "# Generated by openwepp-assurance; DO NOT EDIT. Source: assurance/catalog.yaml\n\
schema_version: 1\n\
vendor_id: openwepp\n\
publication_state: v1_retired_zero_reports\n\
vendoring_authorized: false\n\
documents: []\n"
        .to_owned()
}

fn digest_outputs(files: &BTreeMap<PathBuf, Vec<u8>>) -> BTreeMap<PathBuf, String> {
    files
        .iter()
        .map(|(path, bytes)| (path.clone(), sha256_bytes(bytes)))
        .collect()
}

fn reject_retired_paths(root: &Path) -> Result<()> {
    for retired in RETIRED_PATHS {
        if retired_path_is_active(&root.join(retired))? {
            return Err(AssuranceError::Invalid(format!(
                "retired v1 assurance route still exists: {retired}"
            )));
        }
    }
    Ok(())
}

fn retired_path_is_active(path: &Path) -> Result<bool> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(false);
    };
    if metadata.file_type().is_symlink() || metadata.is_file() {
        return Ok(true);
    }
    for entry in fs::read_dir(path).map_err(|error| AssuranceError::io(path, error))? {
        let entry = entry.map_err(|error| AssuranceError::io(path, error))?;
        if retired_path_is_active(&entry.path())? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn require_exact_public_files(root: &Path) -> Result<()> {
    let public_root = root.join(EXPECTED_GENERATED_ROOT);
    let mut files = BTreeSet::new();
    collect_files(&public_root, &public_root, &mut files)?;
    files.retain(|path| !path.starts_with("review-drafts"));
    let expected = BTreeSet::from([PathBuf::from("README.md")]);
    if files != expected {
        return Err(AssuranceError::Drift(format!(
            "public assurance output set must contain only README.md; observed {files:?}"
        )));
    }
    Ok(())
}

fn collect_files(root: &Path, directory: &Path, files: &mut BTreeSet<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(directory)
        .map_err(|error| AssuranceError::io(directory, error))?
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|error| AssuranceError::io(directory, error))?;
    for entry in entries {
        let file_type = entry
            .file_type()
            .map_err(|error| AssuranceError::io(entry.path(), error))?;
        if file_type.is_symlink() {
            return Err(AssuranceError::Invalid(format!(
                "assurance output cannot contain symlinks: {}",
                entry.path().display()
            )));
        }
        if file_type.is_dir() {
            collect_files(root, &entry.path(), files)?;
        } else if file_type.is_file() {
            let path = entry.path();
            let relative = path.strip_prefix(root).map_err(|error| {
                AssuranceError::Invalid(format!("failed to relativize output: {error}"))
            })?;
            files.insert(relative.to_path_buf());
        } else {
            return Err(AssuranceError::Invalid(format!(
                "assurance output contains an unsupported filesystem entry: {}",
                entry.path().display()
            )));
        }
    }
    Ok(())
}

fn read_confined(root: &Path, relative: &Path) -> Result<Vec<u8>> {
    validate_relative(relative)?;
    crate::v2::read_regular_confined(root, relative)
}

fn validate_relative(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(AssuranceError::Invalid(format!(
            "path must be a confined relative path: {}",
            path.display()
        )));
    }
    Ok(())
}

fn prepare_output_root(requested: Option<&Path>, repository: &Path) -> Result<PathBuf> {
    let root = requested.unwrap_or(repository);
    fs::create_dir_all(root).map_err(|error| AssuranceError::io(root, error))?;
    root.canonicalize()
        .map_err(|error| AssuranceError::io(root, error))
}

fn write_confined(root: &Path, relative: &Path, bytes: &[u8]) -> Result<()> {
    validate_relative(relative)?;
    let path = root.join(relative);
    let parent = path
        .parent()
        .ok_or_else(|| AssuranceError::Invalid("output has no parent".to_owned()))?;
    fs::create_dir_all(parent).map_err(|error| AssuranceError::io(parent, error))?;
    let canonical_parent = parent
        .canonicalize()
        .map_err(|error| AssuranceError::io(parent, error))?;
    if !canonical_parent.starts_with(root) {
        return Err(AssuranceError::Invalid(format!(
            "output escapes selected root: {}",
            relative.display()
        )));
    }
    if path
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err(AssuranceError::Invalid(format!(
            "output target cannot be a symlink: {}",
            path.display()
        )));
    }
    fs::write(&path, bytes).map_err(|error| AssuranceError::io(&path, error))
}

fn validate_snapshot_options(options: &BuildOptions) -> Result<()> {
    if options.snapshot.is_some() == options.snapshot_root.is_some() {
        return Ok(());
    }
    Err(AssuranceError::Usage(
        "--snapshot and --snapshot-root must be supplied together".to_owned(),
    ))
}

fn validate_snapshot_id(snapshot_id: &str) -> Result<()> {
    if !snapshot_id.is_empty()
        && snapshot_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Ok(());
    }
    Err(AssuranceError::Invalid(
        "snapshot ID must contain only ASCII letters, digits, '_' or '-'".to_owned(),
    ))
}

#[derive(Debug)]
struct SnapshotResult {
    manifest_path: PathBuf,
    manifest_sha256: String,
    confirmed_existing: bool,
}

fn create_snapshot(
    requested_root: &Path,
    snapshot_id: &str,
    inputs: &BTreeMap<PathBuf, String>,
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<SnapshotResult> {
    validate_snapshot_id(snapshot_id)?;
    fs::create_dir_all(requested_root)
        .map_err(|error| AssuranceError::io(requested_root, error))?;
    let root = requested_root
        .canonicalize()
        .map_err(|error| AssuranceError::io(requested_root, error))?;
    let target = root.join(snapshot_id);
    let manifest = snapshot_manifest(snapshot_id, inputs, files)?;
    match fs::symlink_metadata(&target) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Err(AssuranceError::Invalid(format!(
                    "snapshot target cannot be a symlink: {}",
                    target.display()
                )));
            }
            if !metadata.is_dir() {
                return Err(AssuranceError::SnapshotConflict(format!(
                    "snapshot target is not a directory: {}",
                    target.display()
                )));
            }
            let confined_target = canonical_snapshot_target(&root, &target)?;
            return confirm_snapshot(&confined_target, &manifest, files);
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(AssuranceError::io(&target, error)),
    }
    fs::create_dir(&target).map_err(|error| AssuranceError::io(&target, error))?;
    let confined_target = canonical_snapshot_target(&root, &target)?;
    for (path, bytes) in files {
        write_confined(&confined_target, &Path::new("files").join(path), bytes)?;
    }
    write_confined(&confined_target, Path::new("manifest.json"), &manifest)?;
    confirm_snapshot(&confined_target, &manifest, files).map(|mut result| {
        result.confirmed_existing = false;
        result
    })
}

fn canonical_snapshot_target(root: &Path, target: &Path) -> Result<PathBuf> {
    let canonical = target
        .canonicalize()
        .map_err(|error| AssuranceError::io(target, error))?;
    if !canonical.starts_with(root) {
        return Err(AssuranceError::Invalid(format!(
            "snapshot target escapes selected root: {}",
            target.display()
        )));
    }
    Ok(canonical)
}

fn snapshot_manifest(
    snapshot_id: &str,
    inputs: &BTreeMap<PathBuf, String>,
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<Vec<u8>> {
    let record = SnapshotManifest {
        schema_version: 2,
        snapshot_id,
        tool_version: env!("CARGO_PKG_VERSION"),
        publication_state: EXPECTED_STATE,
        report_count: 0,
        reports: Vec::new(),
        inputs: inputs
            .iter()
            .map(|(path, sha256)| SnapshotFile {
                path: path.to_string_lossy().into_owned(),
                sha256: sha256.clone(),
            })
            .collect(),
        files: files
            .iter()
            .map(|(path, bytes)| SnapshotFile {
                path: path.to_string_lossy().into_owned(),
                sha256: sha256_bytes(bytes),
            })
            .collect(),
    };
    let mut bytes = serde_json::to_vec_pretty(&record).map_err(|error| {
        AssuranceError::Invalid(format!("failed to serialize snapshot manifest: {error}"))
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn confirm_snapshot(
    target: &Path,
    manifest: &[u8],
    files: &BTreeMap<PathBuf, Vec<u8>>,
) -> Result<SnapshotResult> {
    let manifest_path = target.join("manifest.json");
    let mut observed = BTreeSet::new();
    collect_files(target, target, &mut observed)?;
    let mut expected = files
        .keys()
        .map(|path| Path::new("files").join(path))
        .collect::<BTreeSet<_>>();
    expected.insert(PathBuf::from("manifest.json"));
    if observed != expected {
        return Err(AssuranceError::SnapshotConflict(
            "snapshot contains unexpected or missing files".to_owned(),
        ));
    }
    require_snapshot_file(&manifest_path, manifest)?;
    for (path, expected) in files {
        require_snapshot_file(&target.join("files").join(path), expected)?;
    }
    Ok(SnapshotResult {
        manifest_path,
        manifest_sha256: sha256_bytes(manifest),
        confirmed_existing: true,
    })
}

fn require_snapshot_file(path: &Path, expected: &[u8]) -> Result<()> {
    let observed = fs::read(path).map_err(|error| AssuranceError::io(path, error))?;
    if observed != expected {
        return Err(AssuranceError::SnapshotConflict(format!(
            "immutable file differs: {}",
            path.display()
        )));
    }
    Ok(())
}

#[derive(Serialize)]
struct SnapshotManifest<'a> {
    schema_version: u32,
    snapshot_id: &'a str,
    tool_version: &'static str,
    publication_state: &'static str,
    report_count: u32,
    reports: Vec<serde_json::Value>,
    inputs: Vec<SnapshotFile>,
    files: Vec<SnapshotFile>,
}

#[derive(Serialize)]
struct SnapshotFile {
    path: String,
    sha256: String,
}
