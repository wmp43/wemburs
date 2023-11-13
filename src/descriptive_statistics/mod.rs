// In descriptive_statistics/mod.rs
pub mod errors;
pub mod dataframe;
pub mod visualizations;

#[cfg(test)]
mod unit_tests;

// At the top of your mod.rs or any other file where you need these modules
pub use crate::descriptive_statistics::errors::*;
pub use crate::descriptive_statistics::dataframe::*;
pub use crate::descriptive_statistics::visualizations::*;

use pyo3::prelude::*;
use numpy::PyArray;


// Helper Macros 
macro_rules! validate_statistical_input {
    // Basic array validation
    (basic, $data:expr) => {{
        if $data.is_empty() {
            return Err(StatsError::EmptyDataSet);
        }

        if $data.iter().any(|&val| val.is_nan() || val.is_infinite()) {
            return Err(StatsError::InvalidInputValue);
        }
    }};
    // Trimmed mean specific validation
    (trimmed, $data:expr, $trim_percent:expr) => {{
        // Invoke the basic validation first
        validate_statistical_input!(basic, $data);

        if $trim_percent < 0.0 || $trim_percent > 1.0 {
            return Err(StatsError::InvalidInputValue);
        }

        let n_to_trim = (($data.len() as f64) * $trim_percent) as usize;
        if $data.len() < 2 * n_to_trim {
            return Err(StatsError::InvalidInputValue);
        }
    }};

    (weighted, $data:expr, $weights:expr) => {
        validate_statistical_input!(basic, $data);
        validate_statistical_input!(basic, $weights);

        if $data.len() != $weights.len() {
            return Err(StatsError::InconsistentLength);
        }

    }
}


// Measures of Centrality
#[pyfunction]
pub fn mean(data: &[f64]) -> Result<f64, StatsError> {
    validate_statistical_input!(basic, &data);
    let count = data.len();
    let sum: f64 = data.iter().sum();
    Ok(sum / count as f64)
}

#[pyfunction]
pub fn trimmed_mean(data: &[f64], trim_percent: f64) -> Result<f64, StatsError> {
    // Could offer trimming with IQR Functionality or other outlier functionality
    // currently should implelement with 10% trim on each end
    // 1. Make a copy, 2. sort, 3. Find num trim 4. return trimmed
    // Pitfalls: untrimmable length, Invalid values

    // checking if inputs are valid, trim_perc and array len
    validate_statistical_input!(trimmed, &data, trim_percent);

    let n_to_trim = ((data.len() as f64) * trim_percent) as usize;
    // Check that there's enough data to trim

    // Sort the data
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Trim the values
    let trimmed_data = &sorted_data[n_to_trim..&data.len() - n_to_trim];

    // Calculate the mean of the trimmed data
    let sum: f64 = trimmed_data.iter().sum();
    let trimmed_count = trimmed_data.len();

    Ok(sum / trimmed_count as f64)
}


pub fn weighted_mean(data: &[f64], weights: &[f64]) -> Result<f64, StatsError> {
    // 1. get cum sum of numerator: w_i * d_i
    // 2. get cum sum of denom: w_i

    //pitfalls:
    // data or weight weight len 0,
    // data and weight non matching len,
    // data or weight value inf or nan
    validate_statistical_input!(weighted, data, weights);

    let numerator_cum_sum: f64 = data.iter()
        .zip(weights.iter()).map(|(&d, &w)| w * d).sum();

    let denominator_cum_sum: f64 = weights.iter().sum();

    if denominator_cum_sum == 0.0 {
        return Err(StatsError::InvalidInputValue);
    }

    Ok(numerator_cum_sum / denominator_cum_sum)
}


pub fn median(data: &[f64]) -> Result<f64, StatsError> {
    validate_statistical_input!(basic, &data);

    let count:i32 = &data.len();

    // It's safe to sort now
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mid = count / 2;
    if count % 2 == 0 {
        // Even len data
        let mid_val = (sorted_data[mid - 1] + sorted_data[mid]) / 2.0;
        println!("Middle Value: {}", mid_val);
        Ok(mid_val)
    } else {
        // Odd len data
        let mid_val = sorted_data[mid];
        println!("Middle Value: {}", mid_val);
        Ok(mid_val)
    }
}


// Measures of spread
pub fn variance(data: &[f64]) -> Result<f64, StatsError> {
    // (d_i * d_bar)**2 / count - 1
    // (d_i * d_bar)**2 is a cum sum
    validate_statistical_input!(basic, &data);
    let mean_value = mean(data)?;
    let count:i32 = &data.len();

    if count < 2 {
        return Err(StatsError::EmptyDataSet);
    }

    let mean = mean(data)?;

    // Proceed with variance calculation
    let sum_sq_diff: f64 = data.iter().map(|&value| {
        let diff = value - mean_value;
        diff * diff
    }).sum();

    let variance = sum_sq_diff / (count as f64 - 1.0);
    Ok(variance)
}

// pub fn median_absolute_deviation(data: &[f64]) -> Result<f64, StatsError> {
//     // some implementation for this ... never heard of this before but seems like it could be useful
//     // L
//     validate_array_input!(&data);
//
// }

pub fn trimmed_variance(data: &[f64], trim_percent: f64) -> Result<f64, StatsError> {
    // Trimmed Var, analagous to trimmed mean.
    validate_statistical_input!(trimmed, &data, trim_percent);
    let count:i32 = &data.len();

    let n_to_trim = ((count as f64) * trim_percent) as usize;
    // Check that there's enough data to trim
    if count < 2 * n_to_trim {
        return Err(StatsError::InvalidInputValue);
    }

    // Sort the data
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Trim the values
    let trimmed_data = &sorted_data[n_to_trim..count - n_to_trim];
    // calculate variance
    let trim_var = variance(trimmed_data)?;

    Ok(trim_var);
}



// median absolute deviation
// IQR -- percentile macro?
//
// Mean Absolute difference







