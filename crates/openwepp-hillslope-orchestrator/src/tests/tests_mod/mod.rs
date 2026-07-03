use std::sync::{Mutex, OnceLock};

mod direct_runtime;
mod direct_runtime_erosion_adjustments;
mod direct_runtime_erosion_operands;
mod direct_runtime_erosion_seed;
mod direct_runtime_r4il;
mod direct_runtime_r4mo;
mod direct_runtime_r4n;
mod direct_runtime_r4pqz;
mod direct_runtime_r5c;
mod direct_runtime_r5d;
mod direct_runtime_r7g_frost;
mod direct_runtime_r7g_snow;
mod direct_runtime_wave1_continuity;

fn direct_runtime_test_lock() -> &'static Mutex<()> {
    static DIRECT_RUNTIME_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    DIRECT_RUNTIME_TEST_LOCK.get_or_init(|| Mutex::new(()))
}
