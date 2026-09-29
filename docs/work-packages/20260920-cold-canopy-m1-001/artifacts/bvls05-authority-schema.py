from pathlib import Path
import sys
sys.path.insert(0,'/workdir/openWEPP/tools')
from sc_contract_directory import ContractSet
p=Path('/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md')
x=ContractSet(p)
print('PASS owning LSE directory schema/profile validation')
