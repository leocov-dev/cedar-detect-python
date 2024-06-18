


## Local Development

Install `maturin` globally or in the local python interpreter

```shell
# globally
pipx install maturin

# locally (after making venv)
pip install maturin
```

Create a local development build
```shell
python -m venv .venv

maturin develop
```

Run tests

```shell
pytest
```