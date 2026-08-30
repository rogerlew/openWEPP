use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::transaction_spool::RunnerTransactionPrivateSpool;
use super::{HillslopeCliError, HillslopeOutputTargets};

struct TransactionEntry {
    final_path: PathBuf,
    staged_path: PathBuf,
    backup_path: PathBuf,
    replace_existing: bool,
    backed_up: bool,
    published: bool,
}

pub(super) struct HillslopeOutputTransaction {
    entries: Vec<TransactionEntry>,
    output_private_spool: RunnerTransactionPrivateSpool,
    manifest_private_spool: RunnerTransactionPrivateSpool,
    staged_targets: HillslopeOutputTargets,
    stage3_evidence_final_path: PathBuf,
    stage3_evidence_private_path: PathBuf,
    manifest_final_path: PathBuf,
    manifest_private_path: PathBuf,
    manifest_staged_path: PathBuf,
    manifest_backup_path: PathBuf,
    manifest_backed_up: bool,
    manifest_published: bool,
    committed: bool,
    #[cfg(test)]
    forced_publish_failure_index: Option<usize>,
    #[cfg(test)]
    forced_stage_unlink_failure_index: Option<usize>,
    #[cfg(test)]
    forced_rollback_remove_failure_index: Option<usize>,
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

        let (transaction_targets, stage3_evidence_final_path) =
            transaction_targets_with_stage3_evidence(final_targets, &manifest_final_path)?;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| transaction_error(format!("transaction clock failed: {error}")))?
            .as_nanos();
        let token = format!("{}-{nonce}", std::process::id());
        let mut entries = Vec::new();
        let mut staged_by_final = BTreeMap::new();
        let mut private_by_final = BTreeMap::new();
        let mut output_private_pairs = Vec::new();
        for final_path in target_paths(&transaction_targets) {
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
            let private_path = sibling_transaction_path(final_path, &token, "spool")?;
            let backup_path = sibling_transaction_path(final_path, &token, "backup")?;
            ensure_transaction_path_absent(&staged_path)?;
            ensure_transaction_path_absent(&private_path)?;
            ensure_transaction_path_absent(&backup_path)?;
            staged_by_final.insert(final_path.clone(), staged_path.clone());
            private_by_final.insert(final_path.clone(), private_path.clone());
            output_private_pairs.push((private_path, staged_path.clone()));
            entries.push(TransactionEntry {
                final_path: final_path.clone(),
                staged_path,
                backup_path,
                replace_existing: final_targets.wat_subhourly.as_ref() != Some(final_path),
                backed_up: false,
                published: false,
            });
        }
        let manifest_staged_path = sibling_transaction_path(&manifest_final_path, &token, "stage")?;
        let manifest_private_path =
            sibling_transaction_path(&manifest_final_path, &token, "spool")?;
        let manifest_backup_path =
            sibling_transaction_path(&manifest_final_path, &token, "backup")?;
        ensure_transaction_path_absent(&manifest_staged_path)?;
        ensure_transaction_path_absent(&manifest_private_path)?;
        ensure_transaction_path_absent(&manifest_backup_path)?;
        // The archive is a first-class transaction entry, but it is not an
        // ordinary configured output. Keep it out of the generic optional
        // output enumerator and expose its private path separately.
        let staged_targets = map_targets(final_targets, &private_by_final)?;
        let stage3_evidence_private_path = private_by_final
            .get(&stage3_evidence_final_path)
            .cloned()
            .ok_or_else(|| transaction_error("missing Stage-3 evidence private spool path"))?;
        let output_private_spool = RunnerTransactionPrivateSpool::new(output_private_pairs)
            .map_err(|source| {
                transaction_io_error("create private output spool", &manifest_final_path, &source)
            })?;
        let manifest_private_spool = RunnerTransactionPrivateSpool::new([(
            manifest_private_path.clone(),
            manifest_staged_path.clone(),
        )])
        .map_err(|source| {
            transaction_io_error(
                "create private manifest spool",
                &manifest_final_path,
                &source,
            )
        })?;
        Ok(Self {
            entries,
            output_private_spool,
            manifest_private_spool,
            staged_targets,
            stage3_evidence_final_path,
            stage3_evidence_private_path,
            manifest_final_path,
            manifest_private_path,
            manifest_staged_path,
            manifest_backup_path,
            manifest_backed_up: false,
            manifest_published: false,
            committed: false,
            #[cfg(test)]
            forced_publish_failure_index: None,
            #[cfg(test)]
            forced_stage_unlink_failure_index: None,
            #[cfg(test)]
            forced_rollback_remove_failure_index: None,
        })
    }

    pub(super) const fn staged_targets(&self) -> &HillslopeOutputTargets {
        &self.staged_targets
    }

    pub(super) fn manifest_private_path(&self) -> &Path {
        &self.manifest_private_path
    }

    pub(super) fn stage3_evidence_private_path(&self) -> &Path {
        &self.stage3_evidence_private_path
    }

    pub(super) fn stage3_evidence_final_path(&self) -> &Path {
        &self.stage3_evidence_final_path
    }

    pub(super) fn publish_outputs(&mut self) -> Result<(), HillslopeCliError> {
        self.output_private_spool
            .promote_complete()
            .map_err(|source| {
                transaction_io_error(
                    "promote private output spool",
                    &self.manifest_final_path,
                    &source,
                )
            })?;
        for entry in &self.entries {
            if !entry.staged_path.is_file() {
                return Err(transaction_error(format!(
                    "staged output is missing or invalid at {}",
                    entry.staged_path.display()
                )));
            }
        }
        if let Err(error) = self.back_up_existing_targets() {
            return Err(self.failure_with_rollback(error));
        }
        for index in 0..self.entries.len() {
            #[cfg(test)]
            if self.forced_publish_failure_index == Some(index) {
                let error = transaction_error("forced output publication failure");
                return Err(self.failure_with_rollback(error));
            }
            let entry = &self.entries[index];
            if let Err(source) = publish_link_no_replace(&entry.staged_path, &entry.final_path) {
                let error = transaction_io_error(
                    "link staged output",
                    &self.entries[index].final_path,
                    &source,
                );
                return Err(self.failure_with_rollback(error));
            }
            self.entries[index].published = true;
            #[cfg(test)]
            if self.forced_stage_unlink_failure_index == Some(index) {
                let error = transaction_error("forced staged-name removal failure");
                return Err(self.failure_with_rollback(error));
            }
            if let Err(source) = fs::remove_file(&self.entries[index].staged_path) {
                let error = transaction_io_error(
                    "remove published staging name",
                    &self.entries[index].staged_path,
                    &source,
                );
                return Err(self.failure_with_rollback(error));
            }
        }
        Ok(())
    }

    pub(super) fn publish_manifest(&mut self) -> Result<(), HillslopeCliError> {
        if let Err(source) = self.manifest_private_spool.promote_complete() {
            let error = transaction_io_error(
                "promote private manifest spool",
                &self.manifest_final_path,
                &source,
            );
            return Err(self.failure_with_rollback(error));
        }
        if !self.manifest_staged_path.is_file() {
            let error = transaction_error(format!(
                "staged manifest is missing or invalid at {}",
                self.manifest_staged_path.display()
            ));
            return Err(self.failure_with_rollback(error));
        }
        if let Err(source) =
            publish_link_no_replace(&self.manifest_staged_path, &self.manifest_final_path)
        {
            let error = transaction_io_error(
                "link completion manifest",
                &self.manifest_final_path,
                &source,
            );
            return Err(self.failure_with_rollback(error));
        }
        self.manifest_published = true;
        if let Err(source) = fs::remove_file(&self.manifest_staged_path) {
            let error = transaction_io_error(
                "remove published manifest staging name",
                &self.manifest_staged_path,
                &source,
            );
            return Err(self.failure_with_rollback(error));
        }
        self.committed = true;
        self.remove_backups_after_commit();
        Ok(())
    }

    pub(super) fn fail_and_rollback(&mut self, primary: HillslopeCliError) -> HillslopeCliError {
        self.failure_with_rollback(primary)
    }

    fn back_up_existing_targets(&mut self) -> Result<(), HillslopeCliError> {
        for entry in &self.entries {
            if !entry.replace_existing && entry.final_path.exists() {
                return Err(transaction_error(
                    "WAT5-E-005 WAT5 output target appeared during execution; no run output was modified",
                ));
            }
        }
        for entry in &mut self.entries {
            if entry.replace_existing && entry.final_path.exists() {
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

    fn rollback(&mut self) -> Result<(), HillslopeCliError> {
        let mut failures = Vec::new();
        if self.manifest_published {
            match fs::remove_file(&self.manifest_final_path) {
                Ok(()) => self.manifest_published = false,
                Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                    self.manifest_published = false;
                }
                Err(source) => failures.push(format!(
                    "remove manifest {}: {source}",
                    self.manifest_final_path.display()
                )),
            }
        }
        for (index, entry) in self.entries.iter_mut().enumerate().rev() {
            #[cfg(not(test))]
            let _ = index;
            if entry.published {
                #[cfg(test)]
                if self.forced_rollback_remove_failure_index == Some(index) {
                    failures.push(format!(
                        "forced removal failure at {}",
                        entry.final_path.display()
                    ));
                    continue;
                }
                match fs::remove_file(&entry.final_path) {
                    Ok(()) => entry.published = false,
                    Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                        entry.published = false;
                    }
                    Err(source) => failures.push(format!(
                        "remove output {}: {source}",
                        entry.final_path.display()
                    )),
                }
            }
        }
        if self.manifest_backed_up && !self.manifest_published {
            match fs::rename(&self.manifest_backup_path, &self.manifest_final_path) {
                Ok(()) => self.manifest_backed_up = false,
                Err(source) => failures.push(format!(
                    "restore manifest backup {}: {source}",
                    self.manifest_backup_path.display()
                )),
            }
        }
        for entry in self.entries.iter_mut().rev() {
            if entry.backed_up && !entry.published {
                match fs::rename(&entry.backup_path, &entry.final_path) {
                    Ok(()) => entry.backed_up = false,
                    Err(source) => failures.push(format!(
                        "restore output backup {}: {source}",
                        entry.backup_path.display()
                    )),
                }
            }
        }
        if failures.is_empty() {
            return Ok(());
        }
        let retained_backups = self
            .entries
            .iter()
            .filter(|entry| entry.backed_up)
            .map(|entry| entry.backup_path.display().to_string())
            .chain(
                self.manifest_backed_up
                    .then(|| self.manifest_backup_path.display().to_string()),
            )
            .collect::<Vec<_>>();
        Err(transaction_error(format!(
            "rollback incomplete; retained_backups={retained_backups:?}; failures={failures:?}"
        )))
    }

    fn failure_with_rollback(&mut self, primary: HillslopeCliError) -> HillslopeCliError {
        self.output_private_spool.discard();
        self.manifest_private_spool.discard();
        match self.rollback() {
            Ok(()) => primary,
            Err(rollback) => transaction_error(format!("{primary}; {rollback}")),
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

    #[cfg(test)]
    fn force_stage_unlink_failure_at(&mut self, index: usize) {
        self.forced_stage_unlink_failure_index = Some(index);
    }

    #[cfg(test)]
    fn force_rollback_remove_failure_at(&mut self, index: usize) {
        self.forced_rollback_remove_failure_index = Some(index);
    }
}

impl Drop for HillslopeOutputTransaction {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.rollback();
        }
        for entry in &self.entries {
            let _ = fs::remove_file(&entry.staged_path);
        }
        let _ = fs::remove_file(&self.manifest_private_path);
        let _ = fs::remove_file(&self.manifest_staged_path);
    }
}

fn transaction_targets_with_stage3_evidence(
    final_targets: &HillslopeOutputTargets,
    manifest_final_path: &Path,
) -> Result<(HillslopeOutputTargets, PathBuf), HillslopeCliError> {
    let stage3_evidence_final_path = stage3_evidence_companion_path(manifest_final_path)?;
    let mut transaction_targets = final_targets.clone();
    if target_paths(&transaction_targets)
        .iter()
        .any(|path| **path == stage3_evidence_final_path)
    {
        return Err(transaction_error(format!(
            "Stage-3 evidence archive aliases run output {}",
            stage3_evidence_final_path.display()
        )));
    }
    transaction_targets
        .optional_outputs
        .push(stage3_evidence_final_path.clone());
    Ok((transaction_targets, stage3_evidence_final_path))
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

fn stage3_evidence_companion_path(manifest_path: &Path) -> Result<PathBuf, HillslopeCliError> {
    let file_name = manifest_path.file_name().ok_or_else(|| {
        transaction_error(format!(
            "manifest path has no file name: {}",
            manifest_path.display()
        ))
    })?;
    let parent = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    Ok(parent.join(format!(
        "{}.stage3-v11-evidence-v1.bin",
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

fn publish_link_no_replace(staged_path: &Path, final_path: &Path) -> Result<(), std::io::Error> {
    fs::hard_link(staged_path, final_path)
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
        fs::write(transaction.stage3_evidence_private_path(), bytes)
            .expect("write Stage-3 evidence fixture");
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
        let evidence = transaction.stage3_evidence_final_path().to_path_buf();
        fs::write(&evidence, b"old-evidence").expect("seed evidence archive");
        write_all_staged(&transaction, b"new");
        transaction.force_publish_failure_at(2);
        transaction
            .publish_outputs()
            .expect_err("forced publication failure");
        for path in target_paths(&targets) {
            assert_eq!(fs::read(path).expect("read restored target"), b"old");
        }
        assert_eq!(
            fs::read(evidence).expect("read restored evidence archive"),
            b"old-evidence"
        );
        assert_eq!(fs::read(manifest).expect("read manifest"), b"old-manifest");
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }

    #[test]
    fn wat5_target_created_during_simulation_is_never_replaced() {
        let directory = test_directory("concurrent-wat5");
        let targets = targets(&directory, true);
        for path in target_paths(&targets) {
            if Some(path.as_path()) != targets.wat_subhourly.as_deref() {
                fs::write(path, b"old").expect("seed sibling target");
            }
        }
        let manifest = directory.join("manifest.json");
        fs::write(&manifest, b"old-manifest").expect("seed manifest");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest.clone()).expect("transaction");
        write_all_staged(&transaction, b"new");
        let wat5 = targets.wat_subhourly.as_ref().expect("WAT5 target");
        fs::write(wat5, b"concurrent-success").expect("publish concurrent WAT5");

        let error = transaction
            .publish_outputs()
            .expect_err("concurrent WAT5 must reject publication");
        assert!(error.to_string().contains("appeared during execution"));
        assert_eq!(
            fs::read(wat5).expect("read concurrent WAT5"),
            b"concurrent-success"
        );
        for path in target_paths(&targets) {
            if path != wat5 {
                assert_eq!(fs::read(path).expect("read sibling target"), b"old");
            }
        }
        assert_eq!(fs::read(manifest).expect("read manifest"), b"old-manifest");
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }

    #[test]
    fn staged_name_removal_failure_rolls_back_linked_output() {
        let directory = test_directory("unlink-failure");
        let targets = targets(&directory, false);
        for path in target_paths(&targets) {
            fs::write(path, b"old").expect("seed target");
        }
        let manifest = directory.join("manifest.json");
        fs::write(&manifest, b"old-manifest").expect("seed manifest");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest.clone()).expect("transaction");
        write_all_staged(&transaction, b"new");
        transaction.force_stage_unlink_failure_at(1);
        transaction
            .publish_outputs()
            .expect_err("forced staged-name removal failure");
        for path in target_paths(&targets) {
            assert_eq!(fs::read(path).expect("read restored target"), b"old");
        }
        assert_eq!(fs::read(&manifest).expect("read manifest"), b"old-manifest");
        drop(transaction);
        let hidden = fs::read_dir(&directory)
            .expect("read transaction directory")
            .map(|entry| entry.expect("directory entry").file_name())
            .filter(|name| name.to_string_lossy().contains(".openwepp-"))
            .collect::<Vec<_>>();
        assert!(hidden.is_empty(), "transaction leftovers: {hidden:?}");
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }

    #[test]
    fn rollback_cleanup_failure_is_surfaced_and_retains_backup_bytes() {
        let directory = test_directory("rollback-cleanup-failure");
        let targets = targets(&directory, false);
        for path in target_paths(&targets) {
            fs::write(path, b"old").expect("seed target");
        }
        let manifest = directory.join("manifest.json");
        fs::write(&manifest, b"old-manifest").expect("seed manifest");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest).expect("transaction");
        write_all_staged(&transaction, b"new");
        let retained_backup = transaction.entries[1].backup_path.clone();
        let affected_final = transaction.entries[1].final_path.clone();
        transaction.force_stage_unlink_failure_at(1);
        transaction.force_rollback_remove_failure_at(1);
        let error = transaction
            .publish_outputs()
            .expect_err("rollback cleanup injection must fail");
        assert!(error.to_string().contains("rollback incomplete"));
        assert!(error.to_string().contains("retained_backups"));
        assert_eq!(
            fs::read(&affected_final).expect("read linked new bytes"),
            b"new"
        );
        assert_eq!(
            fs::read(&retained_backup).expect("read retained old bytes"),
            b"old"
        );
        drop(transaction);
        assert_eq!(
            fs::read(&retained_backup).expect("backup survives Drop"),
            b"old"
        );
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
    fn manifest_construction_failure_surfaces_incomplete_rollback() {
        let directory = test_directory("manifest-construction-rollback");
        let targets = targets(&directory, false);
        for path in target_paths(&targets) {
            fs::write(path, b"old").expect("seed target");
        }
        let manifest = directory.join("manifest.json");
        fs::write(&manifest, b"old-manifest").expect("seed manifest");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest).expect("transaction");
        write_all_staged(&transaction, b"new");
        transaction
            .publish_outputs()
            .expect("publish staged outputs before manifest construction");
        let retained_backup = transaction.entries[0].backup_path.clone();
        transaction.force_rollback_remove_failure_at(0);

        let error = transaction.fail_and_rollback(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "manifest",
            detail: "forced manifest construction failure".to_string(),
        });

        let detail = error.to_string();
        assert!(detail.contains("forced manifest construction failure"));
        assert!(detail.contains("rollback incomplete"));
        assert!(detail.contains("retained_backups"));
        assert_eq!(
            fs::read(&retained_backup).expect("read retained old output"),
            b"old"
        );
        drop(transaction);
        assert_eq!(
            fs::read(&retained_backup).expect("backup survives Drop"),
            b"old"
        );
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }

    #[test]
    fn success_commits_every_output_and_manifest_last() {
        let directory = test_directory("success");
        let targets = targets(&directory, false);
        let manifest = directory.join("manifest.json");
        let mut transaction =
            HillslopeOutputTransaction::new(&targets, manifest.clone()).expect("transaction");
        let evidence = transaction.stage3_evidence_final_path().to_path_buf();
        write_all_staged(&transaction, b"new");
        transaction
            .publish_outputs()
            .expect("publish staged outputs");
        assert!(
            !manifest.exists(),
            "manifest is the final completion marker"
        );
        fs::write(transaction.manifest_private_path(), b"new-manifest")
            .expect("write staged manifest");
        transaction.publish_manifest().expect("publish manifest");
        for path in target_paths(&targets) {
            assert_eq!(fs::read(path).expect("read committed target"), b"new");
        }
        assert_eq!(
            fs::read(evidence).expect("read committed evidence archive"),
            b"new"
        );
        assert_eq!(fs::read(manifest).expect("read manifest"), b"new-manifest");
        drop(transaction);
        fs::remove_dir_all(directory).expect("remove transaction directory");
    }
}
