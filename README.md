# rust_vs_python
Rust and python code to compare imported Rust code into Python vs native python <br>
Rust nightly may be needed for building because pyo3 is used<br>
pytest-benchmark also required for benchmarking<br>
Rust only code can be found in Roco-scientist/rust_vs_python_2<br>
<h3>Results</h1><br>
<h4>Within Python</h4>
Python: 21us<br>
Rust: 11us<br>
<h4>Only Rust</h4><br>
Rust: 7us<br>
```
cargo build --release
pytest
```
