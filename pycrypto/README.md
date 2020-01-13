# Python-rust binding with pyo3 and maturin

## Prerequisites

Uses the py03 crate, which requires nightly rust. It can be used for both rust packages for python and for embedding python into a rust app.

(Below is using a python3 virtualenv...)

Install maturin:
```bash
(.venv) $ pip install maturin
```
Build locally:
```
(.venv) $ maturin develop
```
Test:
```
python test/test.py
```

## References

[py03 docs](https://docs.rs/pyo3/0.8.5/pyo3/index.html)

[py03 repo with examples](https://github.com/PyO3/pyo3)

[maturin docs](https://docs.rs/maturin/0.7.7/maturin/)