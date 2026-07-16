use std::env;
use std::error::Error;
use std::path::PathBuf;

use openwepp_assurance::V2Repository;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let repository_root = PathBuf::from(args.next().ok_or("missing repository root")?);
    let staging_root = PathBuf::from(args.next().ok_or("missing staging root")?);
    let report_id = args.next().ok_or("missing report ID")?;
    if args.next().is_some() {
        return Err("unexpected extra argument".into());
    }
    let report_id = report_id.into_string().map_err(|_| "report ID is not UTF-8")?;

    let repository = V2Repository::open(&repository_root)?;
    let roots = repository.review_roots(&report_id, &staging_root)?;
    println!("report_id={}", roots.report_id);
    println!("subject_root={}", roots.subject_root);
    println!(
        "finding_ledger_root={}",
        roots.finding_ledger_root.as_deref().unwrap_or("null")
    );
    println!(
        "approval_lock_root={}",
        roots.approval_lock_root.as_deref().unwrap_or("null")
    );
    println!(
        "release_transfer_root={}",
        roots.release_transfer_root.as_deref().unwrap_or("null")
    );
    Ok(())
}
