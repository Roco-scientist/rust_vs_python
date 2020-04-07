#[macro_use]
extern crate lazy_static;
extern crate regex;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;

lazy_static! {
    static ref DOLLAR_SEARCH: Regex = Regex::new(r"DOLLAR|USD|\$").unwrap();
}
/// Finds the multiplier within a couple of lines that dollar is mentioned
#[pyfunction]
pub fn find_multiplier(table_header: Vec<String>) -> u32 {
    // find the line within the header that contains the dollar term
    let line_index = table_header
        .iter()
        .enumerate()
        .find(|(_, line)| DOLLAR_SEARCH.is_match(&line.to_uppercase()))
        .map(|(x, _)| x);

    // if dollar is found within the index then find the number multiplier nearby
    if let Some(index) = line_index {
        table_header
            .iter()
            .skip(index - 1)
            .take(3)
            .find_map(|line| {
                [
                    ("THOUSAND", 1000),
                    ("MILLION", 1000000),
                    ("BILLION", 1000000000),
                ]
                .iter()
                .find_map(|(key, val)| {
                    if line.to_uppercase().contains(key) {
                        Some(*val)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(1)
    } else {
        1
    }
}

#[pymodule]
fn librust_vs_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(find_multiplier))?;

    Ok(())
}
