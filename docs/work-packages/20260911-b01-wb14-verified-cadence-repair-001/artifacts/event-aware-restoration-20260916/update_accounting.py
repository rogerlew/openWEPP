"""Refresh the charged wall-clock ledger and preserved nonzero receipt floor."""
import datetime,json
from pathlib import Path
A=Path(__file__).resolve().parent
now=datetime.datetime.now(datetime.timezone.utc)
start=datetime.datetime.fromisoformat("2026-09-16T15:12:00+00:00")
elapsed=(now-start).total_seconds()
failures=[]
for path in sorted(A.glob("*.json")):
    try: value=json.loads(path.read_text())
    except (ValueError, OSError): continue
    if isinstance(value,dict) and "argv" in value and value.get("exit_code",0)!=0:
        failures.append(dict(receipt=path.name,exit_code=value["exit_code"],elapsed_seconds=value.get("elapsed_seconds"),
          classification="expected red" if path.stem=="a002-prospective-red" else "retained failure"))
inventory=dict(recorded_utc=now.isoformat(),historical_floor=88,recorded_nonzero_commands=failures,
 additional_failed_temp_checks=2,cumulative_failure_floor=90+len(failures),
 limitation="Lower bound: early discovery/tool/temporary source uncertainty retained; no failure-count stop.")
budget=dict(recorded_utc=now.isoformat(),start_utc=start.isoformat(),deadline_utc="2026-09-16T19:12:00+00:00",
 carry_charged_seconds=51146.178324,ceiling_seconds=65546.178324,elapsed_seconds=elapsed,
 charged_seconds=51146.178324+elapsed,remaining_seconds=14400-elapsed,wait_deductions_seconds=0,
 prior_90_second_reservation_included_not_reclaimed=True,cumulative_failure_floor=inventory["cumulative_failure_floor"],
 physical_allowances={"authentic_capture":"1/1 exhausted","witness":"1/1 exhausted","corrected_recorder":"1/1 exhausted"},
 note="All wall time including builds, orchestration, review and publication counts once; later return/publication time continues accruing.")
for name,value in [("new-failure-receipts.json",inventory),("budget-current.json",budget)]:
    (A/name).write_text(json.dumps(value,indent=2)+"\n")
print(json.dumps(budget))

