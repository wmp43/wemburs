// In descriptive_statistics/mod.rs
pub mod errors;
// At the top of your mod.rs or any other file where you need these modules
pub use crate::descriptive_statistics::errors::*;
use pyo3::types::PyDict;
//use statrs::statistics::{Statistics, Median, MeanN, VarianceN};
use pyo3::prelude::*;
use crate::utils::from_pyarray1;

#[macro_export] macro_rules! validate_statistical_input {
    // Basic array validation
    (basic, $data:expr) => {

        if $data.is_empty() {
            return Err(StatsError::EmptyDataSet.into());
        }

        if $data.iter().any(|val| val.is_nan() || val.is_infinite()) {
            return Err(StatsError::InvalidInputValue.into());
        }
    };
    // Trimmed mean specific validation
    (trimmed, $data:expr, $trim_percent:expr) => {
        // Invoke the basic validation first
        validate_statistical_input!(basic, $data);

        if $trim_percent < 0.0 || $trim_percent > 1.0 {
            return Err(StatsError::InvalidInputValue.into());
        }

        let n_to_trim = (($data.len() as f64) * $trim_percent) as usize;
        if $data.len() < 2 * n_to_trim {
            return Err(StatsError::InvalidInputValue.into());
        }
    };

    (weighted, $data:expr, $weights:expr) => {
        validate_statistical_input!(basic, $data);
        validate_statistical_input!(basic, $weights);

        if $data.len() != $weights.len() {
            return Err(StatsError::InconsistentLength.into());
        }

    }
}

// Rust native functions
pub fn median_rs(data: &[f64]) -> f64 {
    let mut data_copy = data.to_vec(); // Clone the data into a new Vec
    data_copy.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = data_copy.len() / 2;
    if data_copy.len() % 2 == 0 {
        (data_copy[mid - 1] + data_copy[mid]) / 2.0
    } else {
        data_copy[mid]
    }
}

pub fn mean_rs(data: &[f64]) -> f64 {
    let mut data_copy = data.to_vec();
    let sum: f64 = data_copy.iter().sum();
    sum / data_copy.len() as f64
}

pub fn variance_rs(data:  &[f64]) -> f64 {
    let mut data_copy = data.to_vec();
    let mean = mean_rs(data);
    let sum_of_squared_diffs: f64 = data_copy.iter()
        .map(|value| {
            let diff = value - mean;
            diff * diff
        })
        .sum();
    sum_of_squared_diffs / data_copy.len() as f64
}

pub fn percentile_rs(data: &[f64], percentile: f64) -> f64 {
    let mut data_copy = data.to_vec();
    data_copy.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if percentile == 0.0 {
        return *data_copy.first().unwrap();
    } else if percentile == 100.0 {
        return *data_copy.last().unwrap();
    }

    let length = data_copy.len() as f64;
    let rank = (percentile / 100.0) * (length - 1.0);
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;

    if lower == upper {
        data_copy[lower]
    } else {
        data_copy[lower] + (rank - lower as f64) * (data_copy[upper] - data_copy[lower])
    }
}


#[pyfunction]
pub fn mean(x: &PyAny) -> PyResult<f64> {
// This is where new utils function should go
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };

    validate_statistical_input!(basic, x_data); // data validation macro
    let mean_value: f64 = mean_rs(x_data);
    Ok(mean_value)
}

#[pyfunction]
pub fn trimmed_mean(x: &PyAny, trim_percent: f64) -> PyResult<f64> {
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(trimmed, &x_data, trim_percent);
    let n_to_trim = ((x_data.len() as f64) * trim_percent) as usize;
    let mut sorted_data = x_data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let trimmed_data = &sorted_data[n_to_trim..&x_data.len() - n_to_trim];
    let sum: f64 = trimmed_data.iter().sum();
    let trimmed_count = trimmed_data.len();
    Ok(sum / trimmed_count as f64)
}


#[pyfunction]
pub fn weighted_mean(x: &PyAny, y: &PyAny) -> PyResult<f64> {
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    let y_data = match from_pyarray1(y) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };

    validate_statistical_input!(weighted, &x_data, &y_data);

    let numerator_cum_sum: f64 = x_data.iter()
        .zip(y_data.iter()).map(|(&d, &w)| w * d).sum();

    let denominator_cum_sum: f64 = y_data.iter().sum();

    if denominator_cum_sum == 0.0 {
        return Err(StatsError::InvalidInputValue.into());
    }

    Ok(numerator_cum_sum / denominator_cum_sum)
}


#[pyfunction]
pub fn median(x: &PyAny) -> PyResult<f64> {
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, &x_data);
    let count = x_data.len();
    let mut sorted_data = x_data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid: usize = count / 2;
    if count % 2 == 0 {
        // Even len data
        let mid_val = (sorted_data[mid - 1] + sorted_data[mid]) / 2.0;
        Ok(mid_val)
    } else {
        // Odd len data
        let mid_val = sorted_data[mid];
        Ok(mid_val)
    }
}


#[pyfunction]
pub fn variance(x: &PyAny) -> PyResult<f64> {
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, &x_data);
    let mean_value: f64 = mean_rs(x_data);
    let count: usize = x_data.len();
    if count < 2 { return Err(StatsError::EmptyDataSet.into()); }
    let sum_sq_diff: f64 = x_data.iter().map(|&value| {
        let diff = value as f64 - mean_value;
        diff * diff
    }).sum();
    let variance = sum_sq_diff / ((count - 1) as f64);
    Ok(variance)
}


#[pyfunction]
pub fn trimmed_variance(x: &PyAny, trim_percent: f64) -> PyResult<f64> {
    // Trimmed Var, analagous to trimmed mean.
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(trimmed, &x_data, trim_percent);
    let count: usize = x_data.len();

    let n_to_trim = ((count as f64) * trim_percent) as usize;
    // Check that there's enough data to trim
    if count < 2 * n_to_trim { return Err(StatsError::InvalidInputValue.into()); }
    let mut sorted_data = x_data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut trimmed_data = &sorted_data[n_to_trim..count - n_to_trim];

    let trim_var = variance_rs(&mut trimmed_data);

    Ok(trim_var)
}


#[pyfunction]
pub fn median_absolute_deviation(x: &PyAny) -> PyResult<f64> {
    // Median absolute deviation
    // MAD = abs(x_i - median(x))
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, &x_data);
    let median = median_rs(x_data);
    let mut absolute_deviation: Vec<f64> = x_data.iter()
        .map(|&x| (x - median).abs())
        .collect();
    let mad: f64 = median_rs(&mut absolute_deviation);
    Ok(mad)
}


#[pyfunction]
pub fn iqr(x: &PyAny) -> PyResult<f64> {
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, &x_data);
    // need way to find 75th and 25th percentile
    if x_data.len() < 2 { return Err(StatsError::InvalidInputValue.into()); }
    let lower_quartile = percentile_rs(x_data, 25.0);
    let upper_quartile = percentile_rs(x_data, 75.0);
    Ok(upper_quartile - lower_quartile)
}


#[pyfunction]
pub fn range(x: &PyAny) -> PyResult<f64> {
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, &x_data);

    let min_val = x_data.iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .ok_or(StatsError::MinMaxError)?;

    let max_val = x_data.iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .ok_or(StatsError::MinMaxError)?;

    Ok((max_val - min_val).abs())
}


#[pyfunction]
pub fn covariance(x: &PyAny, y: &PyAny) -> PyResult<f64> {
    // Covariance of two PyArrays
    // Sum((x_i - x_bar) * (y_i - y_bar)) / n - 1
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    let y_data = match from_pyarray1(y) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(weighted, &x_data, &y_data);

    let n = x_data.len() as f64;
    if n < 2.0 { return Err(StatsError::InvalidInputValue.into()); }
    let x_mean: f64 = mean_rs(x_data);
    let y_mean: f64 = mean_rs(y_data);

    let cov_numerator: f64 = x_data.iter().zip(y_data.iter())
        .map(|(&x, &y)| (x - x_mean) * (y - y_mean))
        .sum();

    Ok(cov_numerator / (n - 1.0))
}


#[pyfunction]
pub fn correlation(x: &PyAny, y: &PyAny) -> PyResult<f64> {
    // crazy formula lol
    // I wonder what crazy bloke came up with this
    // he deserves a pint
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    let y_data = match from_pyarray1(y) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(weighted, &x_data, &y_data);

    let n = x_data.len() as f64;
    if n < 2.0 { return Err(StatsError::InvalidInputValue.into()); }


    let x_cumsum: f64 = x_data.iter().sum();
    let y_cumsum: f64 = y_data.iter().sum();
    let x_y_product: f64 = x_data.iter().zip(y_data.iter())
        .map(|(&x, &y)| x * y)
        .sum();

    let corr_numer: f64 = n * x_y_product - x_cumsum * y_cumsum;

    let x_squares_sum: f64 = x_data.iter()
        .map(|&x| x.powi(2))
        .sum();

    let y_squares_sum: f64 = y_data.iter()
        .map(|&y| y.powi(2))
        .sum();

    let x_part: f64 = n * x_squares_sum - x_cumsum.powi(2);
    let y_part: f64 = n * y_squares_sum - y_cumsum.powi(2);

    let corr_denom = (x_part * y_part).sqrt();

    if corr_denom == 0.0 { return Err(StatsError::ZeroVariance.into()); }

    Ok(corr_numer / corr_denom)
}


#[pyfunction]
pub fn skewness(x: &PyAny) -> PyResult<f64> {
    // Assumes normal distribution
    // returns f64
    // 0 is symmetric distribution
    // >0 denotes asymmetric tail extending toward positive vals
    // < 0 denotes asymmetric tail extending toward negative vals
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, &x_data);

    let n = x_data.len() as f64;
    if n < 2.0 { return Err(StatsError::InvalidInputValue.into()); }

    let mean: f64 = mean_rs(x_data);
    let var: f64 = variance_rs(x_data);
    let std: f64 = var.sqrt();
    if std == 0.0 { return Err(StatsError::ZeroVariance.into()); }

    let standardized_scores: f64 = x_data.iter()
        .map(|&x| ((x - mean)/std).powi(3))
        .sum();

    let skew: f64 = (n * standardized_scores) / ((n - 1.0) * (n - 2.0));
    Ok(skew)
}


#[pyfunction]
pub fn kurtosis(x: &PyAny) -> PyResult<f64> {
    // I'd rate this function 6.8 / 10.0
    // It would be cool if there were multiple methods for bias correction depending on distribution
    // maybe I could implement that
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, &x_data);
    let n = x_data.len() as f64;
    if n < 3.0 { return Err(StatsError::InvalidInputValue.into()); }

    let normalization_factor: f64 = (n * (n + 1.0)) / ((n - 1.0) * (n - 2.0) * (n - 3.0));

    let mean: f64 = mean_rs(x_data);
    let var: f64 = variance_rs(x_data);
    let std: f64 = var.sqrt();
    if std == 0.0 { return Err(StatsError::ZeroVariance.into()); }

    let sum_fourth_powers: f64 = x_data.iter()
        .map(|&x| ((x - mean) / std).powi(4))
        .sum();

    let bias_correction: f64 = 3.0 * (n - 1.0).powi(2) / ((n - 2.0) * (n - 3.0));

    Ok(normalization_factor * sum_fourth_powers - bias_correction)
}

#[pyfunction]
pub fn summary_statistics(x: &PyAny) -> PyResult<PyObject> {
    // This function should also be converted for dataframe methods
    let py = unsafe { Python::assume_gil_acquired() };
    let x_data = match from_pyarray1(x) {
        Ok(data) => data,
        Err(e) => return Err(StatsError::Conversion.into()),
    };
    validate_statistical_input!(basic, x_data);

    let mean = mean(x)?;
    let median = median(x)?;
    let range = range(x)?;
    let variance = variance(x)?;
    let iqr = iqr(x)?;
    let mad = median_absolute_deviation(x)?;
    let skew = skewness(x)?;
    let kurt = kurtosis(x)?;

    let summary = PyDict::new(py);
    summary.set_item("mean", mean)?;
    summary.set_item("median", median)?;
    summary.set_item("range", range)?;
    summary.set_item("variance", variance)?;
    summary.set_item("interquartile_range", iqr)?;
    summary.set_item("median_absolute_deviation", mad)?;
    summary.set_item("skewness", skew)?;
    summary.set_item("kurtosis", kurt)?;

    Ok(summary.into())
}





