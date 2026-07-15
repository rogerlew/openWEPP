use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

use crate::error::{AssuranceError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Catalog,
    Schema,
    Tool,
    Method,
    Dossier,
    EvidenceManifest,
    EvidenceAsset,
    Narrative,
    Interpretation,
    Limitations,
    AuthoringRecord,
    AuthoringInput,
    AuthoringOutput,
    Review,
    Template,
    PublicOutput,
    Export,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    nodes: BTreeMap<String, Node>,
    contract_version: u32,
    schema_version: u32,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl DependencyGraph {
    pub(crate) const fn new(contract_version: u32, schema_version: u32) -> Self {
        Self {
            nodes: BTreeMap::new(),
            contract_version,
            schema_version,
        }
    }

    /// Adds a uniquely identified node.
    ///
    /// # Errors
    ///
    /// Returns an error when the node ID already exists.
    pub fn insert(&mut self, node: Node) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(AssuranceError::Invalid(format!(
                "duplicate graph node ID: {}",
                node.id
            )));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Verifies that all dependencies exist and the graph is acyclic.
    ///
    /// # Errors
    ///
    /// Returns an error for a missing dependency or cycle.
    pub fn validate(&self) -> Result<()> {
        self.validate_dependencies()?;
        let mut complete = BTreeSet::new();
        let mut active = BTreeSet::new();
        for id in self.nodes.keys() {
            self.visit(id, &mut active, &mut complete)?;
        }
        Ok(())
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub(crate) fn fingerprints_for(
        &self,
        repository_root: &Path,
        roots: &[String],
    ) -> Result<BTreeMap<String, String>> {
        let mut complete = BTreeMap::new();
        let mut active = BTreeSet::new();
        for id in roots {
            self.fingerprint_node(id, repository_root, &mut active, &mut complete)?;
        }
        Ok(complete)
    }

    fn validate_dependencies(&self) -> Result<()> {
        for node in self.nodes.values() {
            for dependency in &node.dependencies {
                if !self.nodes.contains_key(dependency) {
                    return Err(AssuranceError::Invalid(format!(
                        "graph node '{}' has missing dependency '{dependency}'",
                        node.id
                    )));
                }
            }
        }
        Ok(())
    }

    fn visit(
        &self,
        id: &str,
        active: &mut BTreeSet<String>,
        complete: &mut BTreeSet<String>,
    ) -> Result<()> {
        if complete.contains(id) {
            return Ok(());
        }
        if !active.insert(id.to_owned()) {
            return Err(AssuranceError::Invalid(format!(
                "dependency graph cycle includes '{id}'"
            )));
        }
        let node = self
            .nodes
            .get(id)
            .ok_or_else(|| AssuranceError::Invalid(format!("missing graph node '{id}'")))?;
        for dependency in &node.dependencies {
            self.visit(dependency, active, complete)?;
        }
        active.remove(id);
        complete.insert(id.to_owned());
        Ok(())
    }

    fn fingerprint_node(
        &self,
        id: &str,
        repository_root: &Path,
        active: &mut BTreeSet<String>,
        complete: &mut BTreeMap<String, String>,
    ) -> Result<String> {
        if let Some(digest) = complete.get(id) {
            return Ok(digest.clone());
        }
        if !active.insert(id.to_owned()) {
            return Err(AssuranceError::Invalid(format!(
                "dependency graph cycle includes '{id}'"
            )));
        }
        let node = self
            .nodes
            .get(id)
            .ok_or_else(|| AssuranceError::Invalid(format!("missing graph node '{id}'")))?;
        let mut hasher = Sha256::new();
        add_field(&mut hasher, b"openwepp-assurance-node-fingerprint-v2");
        add_field(&mut hasher, &self.contract_version.to_be_bytes());
        add_field(&mut hasher, &self.schema_version.to_be_bytes());
        add_field(&mut hasher, node.kind.label().as_bytes());
        add_field(&mut hasher, node.id.as_bytes());
        add_field(&mut hasher, node.path.to_string_lossy().as_bytes());
        for dependency in &node.dependencies {
            let digest = self.fingerprint_node(dependency, repository_root, active, complete)?;
            add_field(&mut hasher, dependency.as_bytes());
            add_field(&mut hasher, digest.as_bytes());
        }
        add_source_bytes(node, repository_root, &mut hasher)?;
        active.remove(id);
        let digest = format!("{:x}", hasher.finalize());
        complete.insert(id.to_owned(), digest.clone());
        Ok(digest)
    }
}

impl NodeKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Catalog => "catalog",
            Self::Schema => "schema",
            Self::Tool => "tool",
            Self::Method => "method",
            Self::Dossier => "dossier",
            Self::EvidenceManifest => "evidence-manifest",
            Self::EvidenceAsset => "evidence-asset",
            Self::Narrative => "narrative",
            Self::Interpretation => "interpretation",
            Self::Limitations => "limitations",
            Self::AuthoringRecord => "authoring-record",
            Self::AuthoringInput => "authoring-input",
            Self::AuthoringOutput => "authoring-output",
            Self::Review => "review",
            Self::Template => "template",
            Self::PublicOutput => "public-output",
            Self::Export => "export",
        }
    }
}

fn add_source_bytes(node: &Node, repository_root: &Path, hasher: &mut Sha256) -> Result<()> {
    if matches!(node.kind, NodeKind::PublicOutput | NodeKind::Export) {
        return Ok(());
    }
    let path = repository_root.join(&node.path);
    if path.is_file() {
        let mut file = File::open(&path).map_err(|error| AssuranceError::io(&path, error))?;
        let expected = file
            .metadata()
            .map_err(|error| AssuranceError::io(&path, error))?
            .len();
        hasher.update(expected.to_be_bytes());
        let mut observed = 0_u64;
        let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();
        loop {
            let count = file
                .read(&mut buffer)
                .map_err(|error| AssuranceError::io(&path, error))?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
            observed += count as u64;
        }
        if observed != expected {
            return Err(AssuranceError::Drift(format!(
                "graph input changed while fingerprinting: {}",
                node.path.display()
            )));
        }
    } else if node.dependencies.is_empty() {
        return Err(AssuranceError::Invalid(format!(
            "fingerprinted source node is not a file: {}",
            node.path.display()
        )));
    }
    Ok(())
}

fn add_field(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::path::PathBuf;

    use sha2::{Digest, Sha256};

    use super::{DependencyGraph, Node, NodeKind};

    #[test]
    fn rejects_missing_and_cyclic_dependencies() {
        let mut missing = DependencyGraph::default();
        missing
            .insert(Node {
                id: "output".into(),
                kind: NodeKind::PublicOutput,
                path: "out.md".into(),
                dependencies: vec!["absent".into()],
            })
            .expect("insert node");
        assert!(missing.validate().is_err());

        let mut cyclic = DependencyGraph::default();
        for (id, dependency) in [("a", "b"), ("b", "a")] {
            cyclic
                .insert(Node {
                    id: id.into(),
                    kind: NodeKind::Dossier,
                    path: format!("{id}.yaml").into(),
                    dependencies: vec![dependency.into()],
                })
                .expect("insert node");
        }
        assert!(cyclic.validate().is_err());
    }

    #[test]
    fn fingerprint_binds_versions_path_length_and_raw_streamed_bytes() {
        let root = scratch_root("fingerprint-contract");
        let path = root.join("source.bin");
        fs::write(&path, b"abc").expect("write fingerprint source");
        let mut graph = DependencyGraph::new(7, 11);
        graph
            .insert(Node {
                id: "asset".into(),
                kind: NodeKind::EvidenceAsset,
                path: PathBuf::from("source.bin"),
                dependencies: vec![],
            })
            .expect("insert source node");
        let observed = &graph
            .fingerprints_for(&root, &["asset".to_owned()])
            .expect("fingerprint source")["asset"];

        let mut expected = Sha256::new();
        for field in [
            b"openwepp-assurance-node-fingerprint-v2".as_slice(),
            7_u32.to_be_bytes().as_slice(),
            11_u32.to_be_bytes().as_slice(),
            b"evidence-asset".as_slice(),
            b"asset".as_slice(),
            b"source.bin".as_slice(),
        ] {
            expected.update((field.len() as u64).to_be_bytes());
            expected.update(field);
        }
        expected.update(3_u64.to_be_bytes());
        expected.update(b"abc");
        assert_eq!(observed, &format!("{:x}", expected.finalize()));

        let mut other_version = DependencyGraph::new(8, 11);
        other_version
            .insert(Node {
                id: "asset".into(),
                kind: NodeKind::EvidenceAsset,
                path: PathBuf::from("source.bin"),
                dependencies: vec![],
            })
            .expect("insert versioned node");
        assert_ne!(
            observed,
            &other_version
                .fingerprints_for(&root, &["asset".to_owned()])
                .expect("fingerprint other version")["asset"]
        );
        fs::remove_dir_all(root).expect("remove fingerprint scratch root");
    }

    #[test]
    fn fingerprints_large_evidence_without_a_whole_file_read() {
        let root = scratch_root("fingerprint-large");
        let path = root.join("large.bin");
        let file = File::create(&path).expect("create sparse evidence");
        file.set_len(8 * 1024 * 1024).expect("size sparse evidence");
        let mut graph = DependencyGraph::default();
        graph
            .insert(Node {
                id: "large".into(),
                kind: NodeKind::EvidenceAsset,
                path: PathBuf::from("large.bin"),
                dependencies: vec![],
            })
            .expect("insert large evidence node");
        assert_eq!(
            graph
                .fingerprints_for(&root, &["large".to_owned()])
                .expect("stream large evidence")["large"]
                .len(),
            64
        );
        fs::remove_dir_all(root).expect("remove large evidence scratch root");
    }

    fn scratch_root(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("{label}-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale graph scratch root");
        }
        fs::create_dir(&root).expect("create graph scratch root");
        root
    }
}
