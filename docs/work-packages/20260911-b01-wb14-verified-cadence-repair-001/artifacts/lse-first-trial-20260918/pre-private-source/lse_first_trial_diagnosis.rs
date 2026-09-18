//! Opt-in, test-support-only operand recording for one bounded LSE diagnosis.
//!
//! The recorder is observational: the solver never reads its state and this
//! module only allocates/serializes after the caller sets its private path.

use std::{cell::RefCell, fs, path::PathBuf};

#[derive(serde::Serialize)]
struct Trace {
    schema: &'static str,
    records: Vec<serde_json::Value>,
}

thread_local! {
    static ACTIVE: RefCell<Option<(PathBuf, Trace)>> = const { RefCell::new(None) };
}

pub fn begin_if_requested() {
    ACTIVE.with(|active| {
        if active.borrow().is_none() {
            if let Some(path) = std::env::var_os("OPENWEPP_LSE_FIRST_TRIAL_TRACE") {
                *active.borrow_mut() = Some((
                    PathBuf::from(path),
                    Trace {
                        schema: "openwepp.lse-first-trial-diagnosis.v1",
                        records: Vec::new(),
                    },
                ));
            }
        }
    });
}

pub fn enabled() -> bool {
    ACTIVE.with(|active| active.borrow().is_some())
}

pub fn record(kind: &'static str, build: impl FnOnce() -> serde_json::Value) {
    ACTIVE.with(|active| {
        if let Some((_, trace)) = active.borrow_mut().as_mut() {
            trace
                .records
                .push(serde_json::json!({"kind": kind, "value": build()}));
        }
    });
}

pub fn finish() {
    ACTIVE.with(|active| {
        let Some((path, trace)) = active.borrow_mut().take() else {
            return;
        };
        if let Ok(bytes) = serde_json::to_vec_pretty(&trace) {
            let temporary = path.with_extension("partial");
            if fs::write(&temporary, bytes).is_ok() {
                let _ = fs::rename(temporary, path);
            }
        }
    });
}
