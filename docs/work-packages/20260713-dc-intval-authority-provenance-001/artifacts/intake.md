# Intake And Focused Evidence

Status: `ACTIVE`

Evidence class: **Ran + Static**. Interactive focused output is supporting and
was not archived; the delegated exact release run supplies archived terminal
consumer evidence.

- start commit: `ed22f37bfef45eee4ae06eb7e08a2abc8561fc81`;
- current fixture, lock, two existing provenance hashes, and Git object at
  `9aa4c3d61549ab30da665a4dc109bab811522fe9` all equal
  `a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e`;
- static pre/post comparison proves the new guard discriminates missing schema
  and corrected provenance; unarchived interactive runs reported its red state
  and subsequent AUTH06 5/5 pass as supporting evidence;
- unarchived supporting runs also reported fixture lock, authority anti-evasion,
  and AUTH11 (2/2) passes;
- fixture JSON and lock bytes remained unchanged; and
- the scaffold-named HPHYS0227 target does not exist. Git history shows commit
  `a381702beca580fa10e71456a897f1a6a705a968` deleted it while the active required
  registry still names that path.

The failed initial lock command used the repository root with a relative lock
entry and verified no file. It was corrected by running from the fixture root;
the corrected command passed and no fixture was changed.
