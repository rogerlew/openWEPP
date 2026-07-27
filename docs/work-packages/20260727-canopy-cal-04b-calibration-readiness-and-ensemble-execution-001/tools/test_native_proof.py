import importlib.util
from pathlib import Path


def load_native_proof():
    path = Path(__file__).with_name("native-proof.py")
    spec = importlib.util.spec_from_file_location("cal04b_native_proof", path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_typed_temperature_threshold_order_error_forms():
    native_proof = load_native_proof()

    assert native_proof.has_typed_temperature_threshold_order_error(
        "lower threshold must be less than upper threshold for temperature"
    )
    assert native_proof.has_typed_temperature_threshold_order_error(
        "invalid plants[0].phenology: minimum_temperature_inactive_c must be "
        "less than minimum_temperature_unconstrained_c"
    )
    assert not native_proof.has_typed_temperature_threshold_order_error(
        "minimum_temperature_inactive_c is non-finite"
    )
