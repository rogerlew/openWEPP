use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::{HillslopeCliError, HillslopeOutputTargets};

struct TransactionEntry {
    final_path: PathBuf,
    staged_path: PathBuf,
    backup_path: PathBuf,
    backed_up: bool,
    published: bool,
}

pub(super) struct HillslopeOutputTransaction {
    entries: Vec<TransactionEntry>,
    staged_targets: HillslopeOutputTargets,
    manifest_final_path: PathBuf,
    manifest_staged_path: PathBuf,
    manifest_backup_path: PathBuf,
    manifest_backed_up: bool,
    manifest_published: bool,
    committed: bool,
    #[cfg(test)]
    forced_publish_failure_index: Option<usize>,
}

impl HillslopeOutputTransaction {
    pub(super) fn new(
        final_targets: &HillslopeOutputTargets,
        manifest_final_path: PathBuf,
    ) -> Result<Self, HillslopeCliError> {
        if final_targets
            .wat_subhourly
            .as_ref()
            .is_some_and(|path| path.exists())
        {
            return Err(transaction_error(
                "WAT5-E-005 WAT5 output target already exists; no run output was modified",
            ));
        }

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| transaction_error(format!("transaction clock failed: {error}")))?
            .as_nanos();
        let token = format!("{}-{nonce}", std::process::id());
        let mut entries = Vec::new();
        let mut staged_by_final = BTreeMap::new();
        for final_path in target_paths(final_targets) {
            if final_path == &manifest_final_path {
                return Err(transaction_error(format!(
                    "manifest path aliases run output {}",
                    final_path.display()
                )));
            }
            if staged_by_final.contains_key(final_path) {
                return Err(transaction_error(format!(
                    "duplicate run output target {}",
                    final_path.display()
                )));
            }
            let staged_path = sibling_transaction_path(final_path, &token, "stage")?;
            let backup_path = sibling_transaction_path(final_path, &token, "backup")?;
            ensure_transaction_path_absent(&staged_path)?;
            ensure_transaction_path_absent(&backup_path)?;
            staged_by_final.insert(final_path.clone(), staged_path.clone());
            entries.push(TransactionEntry {
                final_path: final_path.clone(),
                staged_path,
                backup_path,
                backed_up: false,
                published: false,
            });
        }
        let manifest_staged_path = sibling_transaction_path(&manifest_final_path, &token, "stage")?;
        let manifest_backup_path =
            sibling_transaction_path(&manifest_final_path, &token, "backup")?;
        ensure_transaction_path_absent(&manifest_staged_path)?;
        ensure_transaction_path_absent(&manifest_backup_path)?;
        let staged_targets = map_targets(final_targets, &staged_by_final)?;
        Ok(Self {
            entries,
            staged_targets,
            manifest_final_path,
            manifest_staged_path,
            manifest_backup_path,
            manifest_backed_up: false,
            manifest_published: false,
            committed: false,
            #[cfg(test)]
            forced_publish_failure_index: None,
        })
    }

    pub(super) const fn staged_targets(&self) -> &HillslopeOutputTargets {
        &self.staged_targets
    }

    pub(super) fn manifest_staged_path(&self) -> &Path {
        &self.manifest_staged_path
    }

    pub(super) fn publish_outputs(&mut self) -> Result<(), HillslopeCliError> {
        for entry in &self.entries {
            if !entry.staged_path.is_file() {
                return Err(transaction_error(format!(
                    "staged output is missing or invalid at {}",
                    entry.staged_path.display()
                )));
            }
        }
        if let Err(error) = self.back_up_existing_targets() {
            self.rollback();
            return Err(error);
        }
        for index in 0..self.entries.len() {
            #[cfg(test)]
            if self.forced_publish_failure_index == Some(index) {
                self.rollback();
                return Err(transaction_error("forced output publication failure"));
            }
            let entry = &mut self.entries[index];
            if let Err(source) = publish_no_replace(&entry.staged_path, &entry.final_path) {
                let error =
                    transaction_io_error("publish staged output", &entry.final_path, &source);
                self.rollback();
                return Err(error);
            }
            entry.published = true;
        }
        Ok(())
    }

    pub(super) fn publish_manifest(&mut self) -> Result<(), HillslopeCliError> {
        if !self.manifest_staged_path.is_file() {
            self.rollback();
            return Err(transaction_error(format!(
                "staged manifest is missing or invalid at {}",
                self.manifest_staged_path.display()
            )));
        }
        if let Err(source) =
            publish_no_replace(&self.manifest_staged_path, &self.manifest_final_path)
        {
            let error = transaction_io_error(
                "publish completion manifest",
                &self.manifest_final_path,
                &source,
            );
            self.rollback();
            return Err(error);
        }
        self.manifest_published = true;
        self.committed = true;
        self.remove_backups_after_commit();
        Ok(())
    }

    fn back_up_existing_targets(&mut self) -> Result<(), HillslopeCliError> {
        for entry in &mut self.entries {
            if entry.final_path.exists() {
                fs::rename(&entry.final_path, &entry.backup_path).map_err(|source| {
                    transaction_io_error("back up existing output", &entry.final_path, &source)
                })?;
                entry.backed_up = true;
            }
        }
        if self.manifest_final_path.exists() {
            fs::rename(&self.manifest_final_path, &self.manifest_backup_path).map_err(
                |source| {
                    transaction_io_error(
                        "back up existing manifest",
                        &self.manifest_final_path,
                        &source,
                    )
                },
            )?;
            self.manifest_backed_up = true;
        }
        Ok(())
    }

    fn rollback(&mut self) {
        if self.manifest_published {
            let _ = fs::remove_file(&self.manifest_final_path);
            self.manifest_published = false;
        }
        for entry in self.entries.iter_mut().rev() {
            if entry.published {
                let _ = fs::remove_file(&entry.final_path);
                entry.published = false;
            }
        }
        if self.manifest_backed_up
            && fs::rename(&self.manifest_backup_path, &self.manifest_final_path).is_ok()
        {
            self.manifest_backed_up = false;
        }
        for entry in self.entries.iter_mut().rev() {
            if entry.backed_up && fs::rename(&entry.backup_path, &entry.final_path).is_ok() {
                entry.backed_up = false;
            }
        }
    }

    fn remove_backups_after_commit(&mut self) {
        if self.manifest_backed_up {
            let _ = fs::remove_file(&self.manifest_backup_path);
            self.manifest_backed_up = false;
        }
        for entry in &mut self.entries {
            if entry.backed_up {
                let _ = fs::remove_file(&entry.backup_path);
                entry.backed_up = false;
            }
        }
    }

    #[cfg(test)]
    fn force_publish_failure_at(&mut self, index: usize) {
        self.forced_publish_failure_index = Some(index);
    }
}

impl Drop for HillslopeOutputTransaction {
    fn drop(&mut self) {
        if !self.committed {
            self.rollback();
        }
        for entry in &self.entries {
            let _ = fs::remove_file(&entry.staged_path);
        }
        let _ = fs::remove_file(&self.manifest_staged_path);
    }
}

fn target_paths(targets: &HillslopeOutputTargets) -> Vec<&PathBuf> {
    std::iter::once(&targets.output_pass)
        .chain(std::iter::once(&targets.output_loss))
        .chain(targets.optional_outputs.iter())
        .chain(targets.laned_active_trace.iter())
        .collect()
}

fn map_targets(
    targets: &HillslopeOutputTargets,
    staged_by_final: &BTreeMap<PathBuf, PathBuf>,
) -> Result<HillslopeOutputTargets, HillslopeCliError> {
    let map = |path: &PathBuf| {
        staged_by_final.get(path).cloned().ok_or_else(|| {
            transaction_error(format!("missing staged mapping for {}", path.display()))
        })
    };
    let map_optional = |path: &Option<PathBuf>| path.as_ref().map(&map).transpose();
    Ok(HillslopeOutputTargets {
        output_pass: map(&targets.output_pass)?,
        output_loss: map(&targets.output_loss)?,
        optional_outputs: targets
            .optional_outputs
            .iter()
            .map(&map)
            .collect::<Result<Vec<_>, _>>()?,
        pass_parquet: map_optional(&targets.pass_parquet)?,
        wat: map_optional(&targets.wat)?,
        wat_subhourly: map_optional(&targets.wat_subhourly)?,
        laned_active_trace: map_optional(&targets.laned_active_trace)?,
        output_hillslope_id: targets.output_hillslope_id,
    })
}

fn sibling_transaction_path(
    final_path: &Path,
    token: &str,
    role: &str,
) -> Result<PathBuf, HillslopeCliError> {
    let file_name = final_path.file_name().ok_or_else(|| {
        transaction_error(format!(
            "output target has no file name: {}",
            final_path.display()
        ))
    })?;
    let parent = final_path.parent().unwrap_or_else(|| Path::new("."));
    Ok(parent.join(format!(
        ".{}.openwepp-{token}.{role}",
        file_name.to_string_lossy()
    )))
}

fn ensure_transaction_path_absent(path: &Path) -> Result<(), HillslopeCliError> {
    if path.exists() {
        return Err(transaction_error(format!(
            "transaction path already exists: {}",
            path.display()
        )));
    }
    Ok(())
}

fn publish_no_replace(staged_path: &Path, final_path: &Path) -> Result<(), std::io::Error> {
    fs::hard_link(staged_path, final_path)?;
    if let Err(source) = fs::remove_file(staged_path) {
        let _ = fs::remove_file(final_path);
        return Err(source);
    }
    Ok(())
}

fn transaction_io_error(action: &str, path: &Path, source: &std::io::Error) -> HillslopeCliError {
    transaction_error(format!("{action} at {} failed: {source}", path.display()))
}

fn transaction_error(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "output_transaction",
        detail: detail.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_directory(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("test clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "openwepp-output-transaction-{label}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create transaction directory");
        path
    }

    fn targets(root: &Path, with_wat5: bool) -> HillslopeOutputTargets {
        let wat = root.join("H1.wat.parquet");
        let pass_parquet = root.join("H1.pass.parquet");
        let wat_subhourly = with_wat5.then(|| root.join("H1.wat-subhourly.parquet"));
        let mut optional_outputs = vec![wat.clone(), pass_parquet.clone()];
        optional_outputs.extend(wat_subhourly.iter().cloned());
        HillslopeOutputTargets {
            output_pass: root.join("H1.hbp"),
            output_loss: root.join("H1.loss.json"),
            optional_outputs,
            pass_parquet: Some(pass_parquet),
            wat: Some(wat),
            wat_subhourly,
            laned_active_trace: None,
            output_hillslope_id: 1,
        }
    }

    fn write_all_staged(transaction: &HillslopeOutputTransaction, bytes: &[u8]) {
        for path in target_paths(transaction.staged_targets()) {
            fs::write(path, bytes).expect("write staged fixture");
        }
    }

    #[test]
    fn existing_wat5_rejects_before_sibling_outputs_are_touched() {
        let directory = test_directory("existing-wat5");
        let targets = targets(&directory, true);
        for path in target_paths(&targets) {
            fs::write(path, b"sentinel").expect("seed target");
        }
        let manifest = directory.join("manifest.json");
        fs::write(&manifest, b"manifest-sentinel").expect("seed manifest");
        let error = HillslopeOutputTransaction::new(&targets, manifest.clone())
            .err()
            .expect("existing WAT5 must reject transaction");
        assert!(error.to_string().contains("WAT5-E-005"));
        for path in target_paths(&targets) {
            assert_eq!(fs::read(path).expect("read sentinel"), b"sentinel");
        }
        assert_eq!(
            fs::read(manifest).expect("read manifest"),
            b"manifest-sentinel"
        );
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }

    #[test]
    fn mid_publication_failure_restores_the_complete_preexisting_set() {
        let directory = test_directory("mid-publish");
        let targets = targets(&directory, false);
        for path in target_paths(&targets) {
            fs::write(path, b"old").expect("seed target");
        }
        let manifest = directory.join("manifest.json");
        fs::write(&manifest, b"old-manifest").expect("seed manifest");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest.clone()).expect("transaction");
        write_all_staged(&transaction, b"new");
        transaction.force_publish_failure_at(2);
        transaction
            .publish_outputs()
            .expect_err("forced publication failure");
        for path in target_paths(&targets) {
            assert_eq!(fs::read(path).expect("read restored target"), b"old");
        }
        assert_eq!(fs::read(manifest).expect("read manifest"), b"old-manifest");
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }

    #[test]
    fn missing_manifest_rolls_back_published_outputs() {
        let directory = test_directory("missing-manifest");
        let targets = targets(&directory, false);
        for path in target_paths(&targets) {
            fs::write(path, b"old").expect("seed target");
        }
        let manifest = directory.join("manifest.json");
        fs::write(&manifest, b"old-manifest").expect("seed manifest");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest.clone()).expect("transaction");
        write_all_staged(&transaction, b"new");
        transaction
            .publish_outputs()
            .expect("publish staged outputs");
        transaction
            .publish_manifest()
            .expect_err("missing completion manifest must roll back");
        for path in target_paths(&targets) {
            assert_eq!(fs::read(path).expect("read restored target"), b"old");
        }
        assert_eq!(fs::read(manifest).expect("read manifest"), b"old-manifest");
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }

    #[test]
    fn success_commits_every_output_and_manifest_last() {
        let directory = test_directory("success");
        let targets = targets(&directory, false);
        let manifest = directory.join("manifest.json");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest.clone()).expect("transaction");
        write_all_staged(&transaction, b"new");
        transaction
            .publish_outputs()
            .expect("publish staged outputs");
        assert!(
            !manifest.exists(),
            "manifest is the final completion marker"
        );
        fs::write(transaction.manifest_staged_path(), b"new-manifest")
            .expect("write staged manifest");
        transaction.publish_manifest().expect("publish manifest");
        for path in target_paths(&targets) {
            assert_eq!(fs::read(path).expect("read committed target"), b"new");
        }
        assert_eq!(fs::read(manifest).expect("read manifest"), b"new-manifest");
        drop(transaction);
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }
}
