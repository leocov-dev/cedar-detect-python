from pathlib import Path

from _pytest.fixtures import fixture

_unit_test_dir = Path(__file__).parent


@fixture(scope='session')
def data_path() -> Path:
    return _unit_test_dir / 'data'
