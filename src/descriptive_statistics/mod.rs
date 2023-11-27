// In descriptive_statistics/mod.rs
pub mod errors;

#[cfg(test)]
mod unit_tests;

// At the top of your mod.rs or any other file where you need these modules
pub use crate::descriptive_statistics::errors::*;
use pyo3::types::PyDict;
use statrs::statistics::Data;
use statrs::statistics::{Statistics, Median, MeanN, VarianceN};
use pyo3::prelude::*;
use numpy::{PyArray1};



macro_rules! validate_statistical_input {
    // Basic array validation
    (basic, $data:expr) => {{
        if $data.is_empty() {
            return Err(StatsError::EmptyDataSet.into());
        }

        if $data.iter().any(|&val| val.is_nan() || val.is_infinite()) {
            return Err(StatsError::InvalidInputValue.into());
        }
    }};
    // Trimmed mean specific validation
    (trimmed, $data:expr, $trim_percent:expr) => {{
        // Invoke the basic validation first
        validate_statistical_input!(basic, $data);

        if $trim_percent < 0.0 || $trim_percent > 1.0 {
            return Err(StatsError::InvalidInputValue.into());
        }

        let n_to_trim = (($data.len() as f64) * $trim_percent) as usize;
        if $data.len() < 2 * n_to_trim {
            return Err(StatsError::InvalidInputValue.into());
        }
    }};

    (weighted, $data:expr, $weights:expr) => {
        validate_statistical_input!(basic, $data);
        validate_statistical_input!(basic, $weights);

        if $data.len() != $weights.len() {
            return Err(StatsError::InconsistentLength.into());
        }

    }
}

fn median_rs(data: &mut [f64]) -> f64 {
    let data = Data::new(data);
    let median = Median::from_data(&data).unwrap();
    median.value()
}

fn mean_rs(data: &mut [f64]) -> f64 {
    let data = Data::new(data);
    let mean = MeanN::from_data(&data).unwrap();
    mean.value()
}

fn variance_rs(data: &mut [f64]) -> f64 {
    let data = Data::new(data);
    let mean = VarianceN::from_data(&data).unwrap();
    mean.value()
}


#[pyfunction]
pub fn mean(data: &PyArray1<f64>) -> PyResult<f64> {
    let data_slice = data.as_slice()?; // convert to rust slice
    validate_statistical_input!(basic, &data_slice); // data validation macro
    let count = data_slice.len();
    let sum: f64 = data_slice.iter().sum();
    Ok(sum / count as f64)
}


// #[pyfunction]
// pub fn mode(data: &PyArray1<f64>) -> PyResult<f64> {
//     let data_slice = data.as_slice()?; // convert to rust slice
//     validate_statistical_input!(basic, &data_slice); // data validation macro
//     // figure out how to count most common values
//     Ok(())
// }


#[pyfunction]
pub fn trimmed_mean(data: &PyArray1<f64>, trim_percent: f64) -> PyResult<f64> {
    let data_slice = data.as_slice()?; // conversion to rust slice
    validate_statistical_input!(trimmed, &data_slice, trim_percent);
    let n_to_trim = ((data_slice.len() as f64) * trim_percent) as usize;
    let mut sorted_data = data_slice.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let trimmed_data = &sorted_data[n_to_trim..&data_slice.len() - n_to_trim];
    let sum: f64 = trimmed_data.iter().sum();
    let trimmed_count = trimmed_data.len();
    Ok(sum / trimmed_count as f64)
}


#[pyfunction]
pub fn weighted_mean(data: &PyArray1<f64>, weights: &PyArray1<f64>) -> PyResult<f64> {
    let data_slice = data.as_slice()?;
    let weights_slice = weights.as_slice()?;
    validate_statistical_input!(weighted, &data_slice, &weights_slice);
    let numerator_cum_sum: f64 = data_slice.iter()
        .zip(weights_slice.iter()).map(|(&d, &w)| w * d).sum();
    let denominator_cum_sum: f64 = weights_slice.iter().sum();
    if denominator_cum_sum == 0.0 {
        return Err(StatsError::InvalidInputValue.into());
    }
    Ok(numerator_cum_sum / denominator_cum_sum)
}


#[pyfunction]
pub fn median(data: &PyArray1<f64>) -> PyResult<f64> {
    let data_slice = data.as_slice()?;
    validate_statistical_input!(basic, &data_slice);
    let count = data_slice.len();
    let mut sorted_data = data_slice.to_vec();
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
pub fn variance(data: &PyArray1<f64>) -> PyResult<f64> {
    let data_slice = data.as_slice()?;
    validate_statistical_input!(basic, &data_slice);
    let mean_value: f64 = mean_rs(&mut data_slice);
    let count: &usize = &data_slice.len();
    if count < 2 { return Err(StatsError::EmptyDataSet.into()); }
    let mean = mean_rs( &mut data_slice);
    let sum_sq_diff: f64 = data_slice.iter().map(|&value| {
        let diff = value as f64 - mean_value;
        diff * diff
    }).sum();
    let variance = sum_sq_diff / (*count as f64 - 1.0);
    Ok(variance)
}


#[pyfunction]
pub fn trimmed_variance(data: &PyArray1<f64>, trim_percent: f64) -> PyResult<f64> {
    // Trimmed Var, analagous to trimmed mean.
    let data_slice = data.as_slice()?;
    validate_statistical_input!(trimmed, &data_slice, trim_percent);
    let count: usize = data_slice.len();

    let n_to_trim = ((count as f64) * trim_percent) as usize;
    // Check that there's enough data to trim
    if count < 2 * n_to_trim { return Err(StatsError::InvalidInputValue.into()); }
    let mut sorted_data = data_slice.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut trimmed_data = &sorted_data[n_to_trim..count - n_to_trim];

    let trim_var = variance_rs(&mut trimmed_data);

    Ok(trim_var)
}


#[pyfunction]
pub fn median_absolute_deviation(data: &PyArray1<f64>) -> PyResult<f64> {
    // Median absolute deviation
    // MAD = abs(x_i - median(x))
    let data_slice = data.as_slice()?;
    validate_statistical_input!(basic, &data_slice);
    let median = median_rs(&mut data_slice);
    let mut absolute_deviation: Vec<f64> = data_slice.iter()
        .map(|&x| (x - median).abs())
        .collect();
    let mad: f64 = median_rs(&mut absolute_deviation);
    Ok(mad)
}


#[pyfunction]
pub fn iqr(data: &PyArray1<f64>) -> PyResult<f64> {
    let data_slice = data.as_slice()?;
    validate_statistical_input!(basic, &data_slice);
    // need way to find 75th and 25th percentile
    if data_slice.len() < 2 { return Err(StatsError::InvalidInputValue.into()); }
    let mut sorted_data = data_slice.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let data_obj = Data::new(sorted_data);
    // Todo: Edit below to avoid unwrap() no panic! allowed
    let lower_quartile = data_obj.percentile(25.0).unwrap();
    let upper_quartile = data_obj.percentile(75.0).unwrap();
    Ok(upper_quartile - lower_quartile)
}


#[pyfunction]
pub fn range(data: &PyArray1<f64>) -> PyResult<f64> {
    let data_slice = data.as_slice()?;
    validate_statistical_input!(basic, &data_slice);

    let min_val = data_slice.iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .ok_or_else(|| StatsError::MinMaxError.into())?;

    let max_val = data_slice.iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .ok_or_else(|| StatsError::MinMaxError.into())?;

    Ok((max_val - min_val).abs())
}




#[pyfunction]
pub fn covariance(x: &PyArray1<f64>, y: &PyArray1<f64>) -> PyResult<f64> {
    // Covariance of two PyArrays
    // Sum((x_i - x_bar) * (y_i - y_bar)) / n - 1
    let x_data = x.as_slice()?;
    let y_data = y.as_slice()?;
    validate_statistical_input!(weighted, &x_data, &y_data);

    let n = x_data.len() as f64;
    if n < 2.0 { return Err(StatsError::InvalidInputValue.into()); }
    let x_mean: f64 = mean_rs(&mut x_data);
    let y_mean: f64 = mean_rs(&mut y_data);

    let cov_numerator: f64 = x_data.iter().zip(y_data.iter())
        .map(|(&x, &y)| (x - x_mean) * (y - y_mean))
        .sum();

    Ok(cov_numerator / (n - 1.0))
}


#[pyfunction]
pub fn correlation(x: &PyArray1<f64>, y: &PyArray1<f64>) -> PyResult<f64> {
    // crazy formula lol
    // I wonder what crazy bloke came up with this
    // he deserves a pint
    let x_data = x.as_slice()?;
    let y_data = y.as_slice()?;
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
pub fn skewness(x: &PyArray1<f64>) -> PyResult<f64> {
    // Assumes normal distribution
    // returns f64
    // 0 is symmetric distribution
    // >0 denotes asymmetric tail extending toward positive vals
    // < 0 denotes asymmetric tail extending toward negative vals
    let data = x.as_slice()?;
    validate_statistical_input!(basic, data);

    let n = data.len() as f64;
    if n < 2.0 { return Err(StatsError::InvalidInputValue.into()); }

    let mean: f64 = mean_rs(&mut data);
    let var: f64 = variance_rs(&mut data);
    let std: f64 = var.sqrt();
    if std == 0.0 { return Err(StatsError::ZeroVariance.into()); }

    let standardized_scores: f64 = data.iter()
        .map(|&x| ((x - mean)/std).powi(3))
        .sum();

    let skew: f64 = (n * standardized_scores) / ((n - 1.0) * (n - 2.0));
    Ok(skew)
}


#[pyfunction]
pub fn kurtosis(x: &PyArray1<f64>) -> PyResult<f64> {
    // I'd rate this function 6.8 / 10.0
    // It would be cool if there were multiple methods for bias correction depending on distribution
    // maybe I could implement that

    let data = x.as_slice()?;
    validate_statistical_input!(basic, &data);
    let n = data.len() as f64;
    if n < 3.0 { return Err(StatsError::InvalidInputValue.into()); }

    let normalization_factor: f64 = (n * (n + 1.0)) / ((n - 1.0) * (n - 2.0) * (n - 3.0));

    let mean: f64 = mean_rs(&mut data);
    let var: f64 = variance_rs(&mut data);
    let std: f64 = var.sqrt();
    if std == 0.0 { return Err(StatsError::ZeroVariance.into()); }

    let sum_fourth_powers: f64 = data.iter()
        .map(|&x| ((x - mean) / std).powi(4))
        .sum();

    let bias_correction: f64 = 3.0 * (n - 1.0).powi(2) / ((n - 2.0) * (n - 3.0));

    Ok(normalization_factor * sum_fourth_powers - bias_correction)
}

#[pyfunction]
pub fn summary_statistics(x: &PyArray1<f64>) -> PyResult<PyObject> {
    let py = unsafe { Python::assume_gil_acquired() };
    let data = x.as_slice()?;

    validate_statistical_input!(basic, data);

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



// Adding to pymodule





