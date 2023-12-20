pub mod descriptive_statistics;
pub mod inferential_statistics;
pub mod linear_models;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
pub use descriptive_statistics::{mean, median, trimmed_mean, weighted_mean, variance,
                                 trimmed_variance, median_absolute_deviation, iqr, range,
                                 covariance, correlation, skewness, kurtosis, summary_statistics};

pub use inferential_statistics::{confidence_interval, kolmogorov_smirnov_test};


//
#[pymodule]
fn wemburs(py: Python, m: &PyModule) -> PyResult<()> {
    // descriptive statistics - yee yoo yaa
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(trimmed_mean, m)?)?;
    m.add_function(wrap_pyfunction!(weighted_mean, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    m.add_function(wrap_pyfunction!(variance, m)?)?;
    m.add_function(wrap_pyfunction!(trimmed_variance, m)?)?;
    m.add_function(wrap_pyfunction!(median_absolute_deviation, m)?)?;
    m.add_function(wrap_pyfunction!(iqr, m)?)?;
    m.add_function(wrap_pyfunction!(range, m)?)?;
    m.add_function(wrap_pyfunction!(covariance, m)?)?;
    m.add_function(wrap_pyfunction!(correlation, m)?)?;
    m.add_function(wrap_pyfunction!(skewness, m)?)?;
    m.add_function(wrap_pyfunction!(kurtosis, m)?)?;
    m.add_function(wrap_pyfunction!(summary_statistics, m)?)?;

    // inferential statistics - wee woo waa
    m.add_function(wrap_pyfunction!(confidence_interval, m)?)?;
    //m.add_function(wrap_pyfunction!(effect_size, m)?)?;
    m.add_function(wrap_pyfunction!(kolmogorov_smirnov_test, m)?)?;
    // m.add_function(wrap_pyfunction!(t_test, m)?)?;
    // m.add_function(wrap_pyfunction!(chi_squared_test, m)?)?;
    // m.add_function(wrap_pyfunction!(anova, m)?)?;
    // m.add_function(wrap_pyfunction!(mann_whitney_u_test, m)?)?;
    // m.add_function(wrap_pyfunction!(spearman_rank_correlation_test, m)?)?;
    // m.add_function(wrap_pyfunction!(fisher_exact_test, m)?)?;
    // m.add_function(wrap_pyfunction!(kruskal_wallis_test, m)?)?;
    // m.add_function(wrap_pyfunction!(wilcoxon_signed_rank_test, m)?)?;
    // m.add_function(wrap_pyfunction!(fisher_exact_test, m)?)?;
    // m.add_function(wrap_pyfunction!(power_analysis, m)?)?;
    Ok(())
}
    // Implement November 22nd
    //
    //
    //
    //

    //    Other Descriptive Functions to include
    //


    // outlier detection
    //    let od = PyModule::new(py, "outlier_detection")?;
    //    outlier_detection.add_function(wrap_pyfunction!(outlier_detection::median_absolute_deviation, od)?)?;


    // visualization
    //    let od = PyModule::new(py, "visualization")?;
    //    od.add_function(wrap_pyfunction!(visualization::box_plot, od)?)?;



