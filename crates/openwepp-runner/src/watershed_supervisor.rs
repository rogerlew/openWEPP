use std::collections::BTreeSet;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, SystemTime};

use openwepp_input_contract::parsers::hbp::{
    HbpLatestEventPayload, HbpParseOptions, parse_hbp_from_path_with_latest_event_payload,
};
use serde_json::json;
use toml::Value;

use crate::api::RunnerLaunchRequest;
use crate::constants::HILLSLOPE_RUNFILE_SCHEMA_ID;
use crate::launch::build_hillslope_argv;
use crate::policy::SidecarPolicy;

#[derive(Debug, Clone)]
pub struct WatershedRunPlan {
    pub run_id: String,
    pub jobs: usize,
    pub hillslope_binary: PathBuf,
    pub hillslope_jobs: Vec<HillslopeJob>,
    pub expected_passes: Vec<PassInventoryExpectation>,
}

impl WatershedRunPlan {
    pub fn new(
        run_id: impl Into<String>,
        jobs: usize,
        hillslope_binary: PathBuf,
        mut hillslope_jobs: Vec<HillslopeJob>,
        mut expected_passes: Vec<PassInventoryExpectation>,
    ) -> Result<Self, String> {
        if jobs == 0 {
            return Err("CLIWAT-E-041 --jobs must be a positive integer".to_string());
        }
        if expected_passes.is_empty() {
            return Err(
                "CLIWAT-E-042 watershed run plan has no expected hillslope passes".to_string(),
            );
        }
        validate_unique_hillslope_ids(
            hillslope_jobs.iter().map(|job| job.hillslope_id),
            "hillslope jobs",
        )?;
        validate_unique_hillslope_ids(
            expected_passes
                .iter()
                .map(|expectation| expectation.hillslope_id),
            "pass inventory expectations",
        )?;

        hillslope_jobs.sort_by_key(|job| job.hillslope_id);
        expected_passes.sort_by_key(|expectation| expectation.hillslope_id);

        Ok(Self {
            run_id: run_id.into(),
            jobs,
            hillslope_binary,
            hillslope_jobs,
            expected_passes,
        })
    }

    pub fn execute_hillslope_jobs_serial(
        &self,
        sidecar_policy: SidecarPolicy,
        legacy_sidecar_discovery: bool,
    ) -> Result<(), String> {
        if self.jobs != 1 {
            return Err(
                "CLIWAT-E-041 serial watershed supervisor only supports --jobs 1".to_string(),
            );
        }
        self.execute_hillslope_jobs(sidecar_policy, legacy_sidecar_discovery)
            .map(|_| ())
    }

    pub fn execute_hillslope_jobs(
        &self,
        sidecar_policy: SidecarPolicy,
        legacy_sidecar_discovery: bool,
    ) -> Result<HillslopeWorkerPoolReport, String> {
        if self.hillslope_jobs.is_empty() {
            return Ok(HillslopeWorkerPoolReport {
                requested_jobs: self.jobs,
                worker_count: 0,
                launched_jobs: 0,
                completed_jobs: 0,
                skipped_jobs: 0,
                elapsed_ms: 0,
            });
        }
        if !self.hillslope_binary.is_file() {
            return Err(format!(
                "CLIWAT-E-042 hillslope binary does not exist: {}",
                self.hillslope_binary.display()
            ));
        }

        for job in &self.hillslope_jobs {
            job.prepare_generated_runfile()?;
        }

        HillslopeWorkerPool::new(self.jobs)?.execute(
            &self.hillslope_binary,
            &self.hillslope_jobs,
            sidecar_policy,
            legacy_sidecar_discovery,
        )
    }

    pub fn validate_pass_inventory(&self) -> Result<PassInventory, String> {
        PassInventory::validate(&self.expected_passes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopeWorkerPoolReport {
    pub requested_jobs: usize,
    pub worker_count: usize,
    pub launched_jobs: usize,
    pub completed_jobs: usize,
    pub skipped_jobs: usize,
    pub elapsed_ms: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HillslopeWorkerPool {
    worker_count: usize,
}

impl HillslopeWorkerPool {
    pub fn new(worker_count: usize) -> Result<Self, String> {
        if worker_count == 0 {
            return Err("CLIWAT-E-041 --jobs must be a positive integer".to_string());
        }
        Ok(Self { worker_count })
    }

    pub fn execute(
        &self,
        hillslope_binary: &Path,
        jobs: &[HillslopeJob],
        sidecar_policy: SidecarPolicy,
        legacy_sidecar_discovery: bool,
    ) -> Result<HillslopeWorkerPoolReport, String> {
        let started = Instant::now();
        let worker_count = self.worker_count.min(jobs.len()).max(1);
        let (sender, receiver) = mpsc::channel::<HillslopeJobCompletion>();
        let mut handles = Vec::with_capacity(jobs.len());
        let mut next_job_index = 0usize;
        let mut launched_jobs = 0usize;
        let mut completed_jobs = 0usize;
        let mut in_flight_jobs = 0usize;
        let mut first_failure: Option<String> = None;

        while in_flight_jobs < worker_count && next_job_index < jobs.len() {
            let spawn_result = spawn_hillslope_job(
                sender.clone(),
                &mut handles,
                jobs[next_job_index].clone(),
                hillslope_binary.to_path_buf(),
                sidecar_policy,
                legacy_sidecar_discovery,
                worker_count,
            );
            if let Err(error) = spawn_result {
                first_failure = Some(error);
                break;
            }
            next_job_index += 1;
            launched_jobs += 1;
            in_flight_jobs += 1;
        }

        while in_flight_jobs > 0 {
            let completion = match receiver.recv() {
                Ok(completion) => completion,
                Err(error) => {
                    first_failure = Some(format!(
                        "CLIWAT-E-043 hillslope worker pool lost a worker result: {error}"
                    ));
                    break;
                }
            };
            in_flight_jobs -= 1;
            completed_jobs += 1;
            if first_failure.is_none() {
                if let Err(error) = completion.result {
                    first_failure = Some(format!(
                        "hillslope job {} failed: {error}",
                        completion.hillslope_id
                    ));
                }
            }

            while first_failure.is_none()
                && in_flight_jobs < worker_count
                && next_job_index < jobs.len()
            {
                let spawn_result = spawn_hillslope_job(
                    sender.clone(),
                    &mut handles,
                    jobs[next_job_index].clone(),
                    hillslope_binary.to_path_buf(),
                    sidecar_policy,
                    legacy_sidecar_discovery,
                    worker_count,
                );
                if let Err(error) = spawn_result {
                    first_failure = Some(error);
                    break;
                }
                next_job_index += 1;
                launched_jobs += 1;
                in_flight_jobs += 1;
            }
        }
        drop(sender);

        for handle in handles {
            if handle.join().is_err() && first_failure.is_none() {
                first_failure =
                    Some("hillslope worker thread panicked after sending no result".to_string());
            }
        }

        let skipped_jobs = jobs.len().saturating_sub(launched_jobs);
        if let Some(failure) = first_failure {
            return Err(format!(
                "CLIWAT-E-043 hillslope worker pool failed; launched={launched_jobs}, completed={completed_jobs}, skipped_pending={skipped_jobs}; {failure}"
            ));
        }

        Ok(HillslopeWorkerPoolReport {
            requested_jobs: self.worker_count,
            worker_count,
            launched_jobs,
            completed_jobs,
            skipped_jobs,
            elapsed_ms: started.elapsed().as_millis(),
        })
    }
}

#[derive(Debug)]
struct HillslopeJobCompletion {
    hillslope_id: u32,
    result: Result<(), String>,
}

fn spawn_hillslope_job(
    sender: mpsc::Sender<HillslopeJobCompletion>,
    handles: &mut Vec<thread::JoinHandle<()>>,
    job: HillslopeJob,
    hillslope_binary: PathBuf,
    sidecar_policy: SidecarPolicy,
    legacy_sidecar_discovery: bool,
    worker_count: usize,
) -> Result<(), String> {
    let hillslope_id = job.hillslope_id;
    let handle = thread::Builder::new()
        .name(format!("openwepp-hillslope-H{hillslope_id}"))
        .spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                job.execute(
                    &hillslope_binary,
                    sidecar_policy,
                    legacy_sidecar_discovery,
                    worker_count,
                )
            }))
            .map_err(|_| format!("CLIWAT-E-043 hillslope job {hillslope_id} panicked"))
            .and_then(|inner| inner);
            let _ = sender.send(HillslopeJobCompletion {
                hillslope_id,
                result,
            });
        })
        .map_err(|error| {
            format!("CLIWAT-E-043 failed spawning hillslope worker {hillslope_id}: {error}")
        })?;
    handles.push(handle);
    Ok(())
}

#[derive(Debug, Clone)]
pub struct HillslopeJob {
    pub hillslope_id: u32,
    pub source_run_file_path: PathBuf,
    pub generated_run_file_path: PathBuf,
    pub output_root: PathBuf,
    pub expected_pass_file: PathBuf,
    pub expected_manifest_file: PathBuf,
    pub stdout_log_path: PathBuf,
    pub stderr_log_path: PathBuf,
    pub timing_path: PathBuf,
    pub freshness_marker_path: PathBuf,
}

impl HillslopeJob {
    fn prepare_generated_runfile(&self) -> Result<(), String> {
        fs::create_dir_all(&self.output_root).map_err(|error| {
            format!(
                "CLIWAT-E-044 failed creating hillslope job output root {}: {error}",
                self.output_root.display()
            )
        })?;
        self.remove_stale_outputs()?;
        let payload = fs::read_to_string(&self.source_run_file_path).map_err(|error| {
            format!(
                "CLIWAT-E-044 failed reading hillslope source runfile {}: {error}",
                self.source_run_file_path.display()
            )
        })?;
        let mut document = payload.parse::<Value>().map_err(|error| {
            format!(
                "CLIWAT-E-044 invalid TOML in hillslope source runfile {}: {error}",
                self.source_run_file_path.display()
            )
        })?;

        validate_hillslope_runfile_schema(&document, &self.source_run_file_path)?;
        rewrite_hillslope_runfile_inputs(&mut document, &self.source_run_file_path)?;
        rewrite_hillslope_runfile_outputs(&mut document, self)?;

        let generated = toml::to_string_pretty(&document).map_err(|error| {
            format!("CLIWAT-E-044 failed serializing generated hillslope runfile: {error}")
        })?;
        write_text_file(
            &self.generated_run_file_path,
            &generated,
            "generated hillslope runfile",
        )
    }

    fn remove_stale_outputs(&self) -> Result<(), String> {
        for (path, label) in [
            (&self.expected_pass_file, "stale generated pass file"),
            (
                &self.expected_manifest_file,
                "stale generated manifest file",
            ),
            (&self.timing_path, "stale generated timing file"),
            (
                &self.freshness_marker_path,
                "stale generated freshness marker",
            ),
            (&self.stdout_log_path, "stale generated stdout log"),
            (&self.stderr_log_path, "stale generated stderr log"),
        ] {
            remove_file_if_exists(path, label)?;
        }
        Ok(())
    }

    fn execute(
        &self,
        hillslope_binary: &Path,
        sidecar_policy: SidecarPolicy,
        legacy_sidecar_discovery: bool,
        worker_count: usize,
    ) -> Result<(), String> {
        let run_dir = self
            .source_run_file_path
            .parent()
            .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
        let request = RunnerLaunchRequest {
            hillslope_binary: hillslope_binary.to_path_buf(),
            run_dir,
            run_file: self.generated_run_file_path.clone(),
            output_dir: self.output_root.clone(),
            sidecar_policy,
            legacy_sidecar_discovery,
            manifest_path: Some(self.expected_manifest_file.clone()),
        };
        let argv = build_hillslope_argv(&request);

        let stdout_file = create_file(&self.stdout_log_path, "hillslope stdout log")?;
        let stderr_file = create_file(&self.stderr_log_path, "hillslope stderr log")?;
        write_text_file(
            &self.freshness_marker_path,
            "openwepp-watershed-hillslope-job-freshness-v1\n",
            "hillslope freshness marker",
        )?;
        let started = Instant::now();
        let status = Command::new(hillslope_binary)
            .args(&argv)
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .status()
            .map_err(|error| {
                format!(
                    "CLIWAT-E-043 failed launching hillslope job {} with {}: {error}",
                    self.hillslope_id,
                    hillslope_binary.display()
                )
            })?;
        let elapsed_ms = started.elapsed().as_millis();

        let timing = json!({
            "schema": "openwepp-watershed-hillslope-job-timing-v1",
            "hillslope_id": self.hillslope_id,
            "elapsed_ms": elapsed_ms,
            "status_code": status.code(),
            "success": status.success(),
            "worker_concurrency": worker_count,
            "failure_policy": "stop-launching-pending-jobs-after-first-hard-failure;wait-for-in-flight-jobs",
            "argv": argv,
            "stdout_log": self.stdout_log_path.display().to_string(),
            "stderr_log": self.stderr_log_path.display().to_string(),
            "expected_pass": self.expected_pass_file.display().to_string(),
            "expected_manifest": self.expected_manifest_file.display().to_string(),
            "freshness_marker": self.freshness_marker_path.display().to_string(),
        });
        let timing_text = serde_json::to_string_pretty(&timing)
            .map_err(|error| format!("CLIWAT-E-043 failed serializing job timing: {error}"))?;
        write_text_file(&self.timing_path, &timing_text, "hillslope timing record")?;

        if !status.success() {
            return Err(format!(
                "CLIWAT-E-043 hillslope job {} exited non-zero; stdout={}, stderr={}",
                self.hillslope_id,
                self.stdout_log_path.display(),
                self.stderr_log_path.display()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PassInventoryExpectation {
    pub hillslope_id: u32,
    pub pass_file_path: PathBuf,
    pub manifest_file_path: Option<PathBuf>,
    pub produced_by_job: bool,
    pub freshness_marker_path: Option<PathBuf>,
    pub timing_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct PassInventory {
    entries: Vec<PassInventoryEntry>,
}

impl PassInventory {
    fn validate(expectations: &[PassInventoryExpectation]) -> Result<Self, String> {
        let mut entries = Vec::with_capacity(expectations.len());
        for expectation in expectations {
            entries.push(PassInventoryEntry::validate(expectation)?);
        }
        Ok(Self { entries })
    }

    #[must_use]
    pub fn entries(&self) -> &[PassInventoryEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone)]
pub struct PassInventoryEntry {
    pub hillslope_id: u32,
    pub pass_file_path: PathBuf,
    pub manifest_file_path: Option<PathBuf>,
    pub pass_size_bytes: u64,
    pub nofe: u16,
    pub npart: u16,
    pub latest_event_payload: HbpLatestEventPayload,
    pub produced_by_job: bool,
}

impl PassInventoryEntry {
    fn validate(expectation: &PassInventoryExpectation) -> Result<Self, String> {
        let metadata = fs::metadata(&expectation.pass_file_path).map_err(|error| {
            format!(
                "CLIWAT-E-045 pass inventory missing hillslope {} pass file {}: {error}",
                expectation.hillslope_id,
                expectation.pass_file_path.display()
            )
        })?;
        if !metadata.is_file() {
            return Err(format!(
                "CLIWAT-E-045 pass inventory path for hillslope {} is not a file: {}",
                expectation.hillslope_id,
                expectation.pass_file_path.display()
            ));
        }
        if metadata.len() == 0 {
            return Err(format!(
                "CLIWAT-E-045 pass inventory path for hillslope {} is empty: {}",
                expectation.hillslope_id,
                expectation.pass_file_path.display()
            ));
        }
        validate_generated_freshness(expectation, &metadata)?;

        let (hbp, latest_event_payload) = parse_hbp_from_path_with_latest_event_payload(
            &expectation.pass_file_path,
            HbpParseOptions {
                expected_hillslope_id: Some(expectation.hillslope_id),
            },
        )
        .map_err(|error| {
            format!(
                "CLIWAT-E-045 failed parsing pass inventory file {} for hillslope {}: {error}",
                expectation.pass_file_path.display(),
                expectation.hillslope_id
            )
        })?;
        if hbp.npart == 0 {
            return Err(format!(
                "CLIWAT-E-045 pass inventory file {} reports npart=0 for hillslope {}",
                expectation.pass_file_path.display(),
                expectation.hillslope_id
            ));
        }
        let latest_event_payload = latest_event_payload.ok_or_else(|| {
            format!(
                "CLIWAT-E-045 pass inventory file {} for hillslope {} has no latest EventPayload; no canonical NoEvent authority is cited for watershed routing",
                expectation.pass_file_path.display(),
                expectation.hillslope_id
            )
        })?;
        validate_latest_event_vectors(
            expectation.hillslope_id,
            hbp.npart,
            &expectation.pass_file_path,
            &latest_event_payload,
        )?;

        Ok(Self {
            hillslope_id: expectation.hillslope_id,
            pass_file_path: expectation.pass_file_path.clone(),
            manifest_file_path: expectation.manifest_file_path.clone(),
            pass_size_bytes: metadata.len(),
            nofe: hbp.nofe,
            npart: hbp.npart,
            latest_event_payload,
            produced_by_job: expectation.produced_by_job,
        })
    }
}

fn validate_generated_freshness(
    expectation: &PassInventoryExpectation,
    pass_metadata: &fs::Metadata,
) -> Result<(), String> {
    if !expectation.produced_by_job {
        return Ok(());
    }
    let marker_path = expectation.freshness_marker_path.as_ref().ok_or_else(|| {
        format!(
            "CLIWAT-E-045 generated pass inventory for hillslope {} has no freshness marker path",
            expectation.hillslope_id
        )
    })?;
    let marker_metadata = fs::metadata(marker_path).map_err(|error| {
        format!(
            "CLIWAT-E-045 generated pass inventory for hillslope {} missing freshness marker {}: {error}",
            expectation.hillslope_id,
            marker_path.display()
        )
    })?;
    let marker_modified = modified_time(marker_path, &marker_metadata, "freshness marker")?;
    assert_file_fresh_after(
        expectation.hillslope_id,
        &expectation.pass_file_path,
        pass_metadata,
        marker_modified,
        "pass file",
    )?;

    let manifest_path = expectation.manifest_file_path.as_ref().ok_or_else(|| {
        format!(
            "CLIWAT-E-045 generated pass inventory for hillslope {} has no manifest path",
            expectation.hillslope_id
        )
    })?;
    let manifest_metadata = fs::metadata(manifest_path).map_err(|error| {
        format!(
            "CLIWAT-E-045 generated pass inventory for hillslope {} missing manifest {}: {error}",
            expectation.hillslope_id,
            manifest_path.display()
        )
    })?;
    assert_file_fresh_after(
        expectation.hillslope_id,
        manifest_path,
        &manifest_metadata,
        marker_modified,
        "manifest file",
    )?;

    let timing_path = expectation.timing_path.as_ref().ok_or_else(|| {
        format!(
            "CLIWAT-E-045 generated pass inventory for hillslope {} has no timing path",
            expectation.hillslope_id
        )
    })?;
    let timing_metadata = fs::metadata(timing_path).map_err(|error| {
        format!(
            "CLIWAT-E-045 generated pass inventory for hillslope {} missing timing record {}: {error}",
            expectation.hillslope_id,
            timing_path.display()
        )
    })?;
    assert_file_fresh_after(
        expectation.hillslope_id,
        timing_path,
        &timing_metadata,
        marker_modified,
        "timing record",
    )
}

fn assert_file_fresh_after(
    hillslope_id: u32,
    path: &Path,
    metadata: &fs::Metadata,
    marker_modified: SystemTime,
    label: &'static str,
) -> Result<(), String> {
    let modified = modified_time(path, metadata, label)?;
    if modified < marker_modified {
        return Err(format!(
            "CLIWAT-E-045 generated pass inventory {label} for hillslope {hillslope_id} is stale relative to freshness marker: {}",
            path.display()
        ));
    }
    Ok(())
}

fn modified_time(
    path: &Path,
    metadata: &fs::Metadata,
    label: &'static str,
) -> Result<SystemTime, String> {
    metadata.modified().map_err(|error| {
        format!(
            "CLIWAT-E-045 failed reading {label} modified time {}: {error}",
            path.display()
        )
    })
}

fn validate_unique_hillslope_ids(
    hillslope_ids: impl Iterator<Item = u32>,
    surface: &'static str,
) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for hillslope_id in hillslope_ids {
        if !seen.insert(hillslope_id) {
            return Err(format!(
                "CLIWAT-E-042 duplicate hillslope id {hillslope_id} in {surface}"
            ));
        }
    }
    Ok(())
}

fn validate_hillslope_runfile_schema(document: &Value, path: &Path) -> Result<(), String> {
    let schema = document
        .get("schema")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-044 hillslope source runfile {} missing string schema",
                path.display()
            )
        })?;
    if schema != HILLSLOPE_RUNFILE_SCHEMA_ID {
        return Err(format!(
            "CLIWAT-E-044 hillslope source runfile {} has unsupported schema '{}' (expected '{}')",
            path.display(),
            schema,
            HILLSLOPE_RUNFILE_SCHEMA_ID
        ));
    }
    Ok(())
}

fn rewrite_hillslope_runfile_inputs(
    document: &mut Value,
    source_run_file: &Path,
) -> Result<(), String> {
    let inputs = document
        .get_mut("inputs")
        .and_then(Value::as_table_mut)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-044 hillslope source runfile {} missing [inputs] table",
                source_run_file.display()
            )
        })?;
    for key in ["soil", "management", "slope", "climate"] {
        rewrite_required_input_path(inputs, source_run_file, key)?;
    }
    if inputs.contains_key("pmetpara") {
        rewrite_optional_input_path(inputs, source_run_file, "pmetpara")?;
    }
    Ok(())
}

fn rewrite_required_input_path(
    inputs: &mut toml::map::Map<String, Value>,
    source_run_file: &Path,
    key: &'static str,
) -> Result<(), String> {
    let value = inputs.get(key).and_then(Value::as_str).ok_or_else(|| {
        format!(
            "CLIWAT-E-044 hillslope source runfile {} missing string inputs.{key}",
            source_run_file.display()
        )
    })?;
    let resolved = resolve_runfile_relative_path(source_run_file, value);
    if !resolved.is_file() {
        return Err(format!(
            "CLIWAT-E-044 hillslope source runfile {} inputs.{key} path is not a file: {}",
            source_run_file.display(),
            resolved.display()
        ));
    }
    let resolved = fs::canonicalize(&resolved).map_err(|error| {
        format!(
            "CLIWAT-E-044 failed canonicalizing hillslope source runfile {} inputs.{key} path {}: {error}",
            source_run_file.display(),
            resolved.display()
        )
    })?;
    inputs.insert(
        key.to_string(),
        Value::String(resolved.display().to_string()),
    );
    Ok(())
}

fn rewrite_optional_input_path(
    inputs: &mut toml::map::Map<String, Value>,
    source_run_file: &Path,
    key: &'static str,
) -> Result<(), String> {
    let value = inputs.get(key).and_then(Value::as_str).ok_or_else(|| {
        format!(
            "CLIWAT-E-044 hillslope source runfile {} inputs.{key} must be a string when present",
            source_run_file.display()
        )
    })?;
    let resolved = resolve_runfile_relative_path(source_run_file, value);
    if !resolved.is_file() {
        return Err(format!(
            "CLIWAT-E-044 hillslope source runfile {} inputs.{key} path is not a file: {}",
            source_run_file.display(),
            resolved.display()
        ));
    }
    let resolved = fs::canonicalize(&resolved).map_err(|error| {
        format!(
            "CLIWAT-E-044 failed canonicalizing hillslope source runfile {} inputs.{key} path {}: {error}",
            source_run_file.display(),
            resolved.display()
        )
    })?;
    inputs.insert(
        key.to_string(),
        Value::String(resolved.display().to_string()),
    );
    Ok(())
}

fn rewrite_hillslope_runfile_outputs(
    document: &mut Value,
    job: &HillslopeJob,
) -> Result<(), String> {
    let outputs = document
        .get_mut("outputs")
        .and_then(Value::as_table_mut)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-044 hillslope source runfile {} missing [outputs] table",
                job.source_run_file_path.display()
            )
        })?;
    for key in ["pass", "loss"] {
        validate_output_string(outputs, &job.source_run_file_path, key)?;
    }
    outputs.insert(
        "pass".to_string(),
        Value::String(job.expected_pass_file.display().to_string()),
    );
    outputs.insert(
        "loss".to_string(),
        Value::String(
            job.output_root
                .join(format!("H{}.loss.json", job.hillslope_id))
                .display()
                .to_string(),
        ),
    );

    for (key, extension) in [
        ("pass_parquet", "pass.parquet"),
        ("wat", "wat.parquet"),
        ("soil", "soil.parquet"),
        ("plot", "plot.parquet"),
        ("ebe", "ebe.parquet"),
        ("element", "element.parquet"),
    ] {
        if outputs.contains_key(key) {
            validate_output_string(outputs, &job.source_run_file_path, key)?;
            outputs.insert(
                key.to_string(),
                Value::String(
                    job.output_root
                        .join(format!("H{}.{}", job.hillslope_id, extension))
                        .display()
                        .to_string(),
                ),
            );
        }
    }

    Ok(())
}

fn validate_output_string(
    outputs: &toml::map::Map<String, Value>,
    source_run_file: &Path,
    key: &'static str,
) -> Result<(), String> {
    if outputs.get(key).and_then(Value::as_str).is_none() {
        return Err(format!(
            "CLIWAT-E-044 hillslope source runfile {} outputs.{key} must be a string",
            source_run_file.display()
        ));
    }
    Ok(())
}

fn resolve_runfile_relative_path(run_file_path: &Path, candidate: &str) -> PathBuf {
    let candidate_path = PathBuf::from(candidate.trim());
    if candidate_path.is_absolute() {
        candidate_path
    } else {
        run_file_path
            .parent()
            .map(|parent| parent.join(&candidate_path))
            .unwrap_or(candidate_path)
    }
}

fn create_file(path: &Path, label: &'static str) -> Result<File, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "CLIWAT-E-043 failed creating parent directory for {label} {}: {error}",
                path.display()
            )
        })?;
    }
    File::create(path).map_err(|error| {
        format!(
            "CLIWAT-E-043 failed creating {label} {}: {error}",
            path.display()
        )
    })
}

fn remove_file_if_exists(path: &Path, label: &'static str) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "CLIWAT-E-044 failed removing {label} {}: {error}",
            path.display()
        )),
    }
}

fn write_text_file(path: &Path, text: &str, label: &'static str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "CLIWAT-E-044 failed creating parent directory for {label} {}: {error}",
                path.display()
            )
        })?;
    }
    fs::write(path, text).map_err(|error| {
        format!(
            "CLIWAT-E-044 failed writing {label} {}: {error}",
            path.display()
        )
    })
}

fn validate_latest_event_vectors(
    hillslope_id: u32,
    npart: u16,
    pass_file_path: &Path,
    payload: &HbpLatestEventPayload,
) -> Result<(), String> {
    let class_count = usize::from(npart);
    for (field, observed) in [
        ("particle_diameter_m", payload.particle_diameter_m.len()),
        (
            "sediment_concentration_kg_m3",
            payload.sediment_concentration_kg_m3.len(),
        ),
        (
            "particle_flow_fraction",
            payload.particle_flow_fraction.len(),
        ),
    ] {
        if observed != class_count {
            return Err(format!(
                "CLIWAT-E-045 pass inventory file {} has {field} length {observed}, expected npart={} for hillslope {}",
                pass_file_path.display(),
                class_count,
                hillslope_id
            ));
        }
    }

    // SC-INFILE-HBP-001 §8.5 (ADR-0036 D4): minor-1 hourly surfaces must
    // arrive as a 24-slot pair whose sediment timing is self-consistent
    // with the event's own concentration × volume mass
    // (`Σ S_h = Σ sedcon_i × Σ V_h`), failing closed on material
    // violation. Minor-0 payloads carry empty vectors and skip cleanly.
    let hourly_volume = &payload.hourly_runoff_volume_m3;
    let hourly_sediment = &payload.hourly_sediment_mass_kg;
    if !hourly_volume.is_empty() || !hourly_sediment.is_empty() {
        if hourly_volume.len() != 24 || hourly_sediment.len() != 24 {
            return Err(format!(
                "CLIWAT-E-046 pass inventory file {} hourly surfaces must be a 24-slot pair \
                 (volumes={}, sediment={}) for hillslope {}",
                pass_file_path.display(),
                hourly_volume.len(),
                hourly_sediment.len(),
                hillslope_id
            ));
        }
        let sediment_total_kg: f64 = hourly_sediment.iter().sum();
        // The sediment-side telescoping identity (SC-SED-001#INV-SED-014):
        // the hour-integrated exported mass equals detachment minus
        // deposition (zero-inflow single-OFE producers; E.3 adds the
        // inflow term with the multi-OFE handoff). This is deliberately
        // volume-free — a concentration x volume reconstruction would
        // embed the producer's efflen/slplen geometry in the intake gate.
        let exported_kg = payload.total_detachment_kg - payload.total_deposition_kg;
        let scale = sediment_total_kg.abs().max(exported_kg.abs()).max(1.0e-9);
        if (sediment_total_kg - exported_kg).abs() > 1.0e-6 * scale {
            return Err(format!(
                "CLIWAT-E-047 pass inventory file {} hourly sediment timing is inconsistent \
                 with the event mass: Σ S_h = {sediment_total_kg} kg vs \
                 tdet - tdep = {exported_kg} kg for hillslope {}",
                pass_file_path.display(),
                hillslope_id
            ));
        }
    }
    Ok(())
}
