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

// Measures of Centrality
pub fn mean(data: &[f64]) -> Result<f64, StatsError> {
    let count = data.len();
    if count == 0 {
        return Err(StatsError::EmptyDataSet);
    }

    let sum: f64 = data.iter().try_fold(0.0, |acc, &val| {
        if val.is_infinite() || val.is_nan() {
            Err(StatsError::InvalidInputValue)
        } else {
            Ok(acc + val)
        }
    })?;

    Ok(sum / count as f64)
}


pub fn trimmed_mean(data: &[f64], trim_perc: f64) -> Result<f64, StatsError> {
    // Could offer trimming with IQR Functionality or other outlier functionality
    // currently should implelement with 10% trim on each end
    // 1. Make a copy, 2. sort, 3. Find num trim 4. return trimmed
    // Pitfalls: untrimmable length, Invalid values

    // checking if inputs are valid, trim_perc and array len
    let count = data.len();

    if trim_perc < 0.0 || trim_perc > 1.0 {
        return Err(StatsError::InvalidInputValue);
    }

    if count == 0 {
        return Err(StatsError::EmptyDataSet);
    }

    let n_to_trim = ((count as f64) * trim_perc) as usize;

    // Check that there's enough data to trim
    if count < 2 * n_to_trim {
        return Err(StatsError::InvalidInputValue);
    }

    // Check for invalid values
    if data.iter().any(|&val| val.is_nan() || val.is_infinite()) {
        return Err(StatsError::InvalidInputValue);
    }

    // Sort the data
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Trim the values
    let trimmed_data = &sorted_data[n_to_trim..count - n_to_trim];

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

    let data_count = data.len();
    let weight_count = weights.len();

    // invalid or mismatching len
    if weight_count != data_count || weight_count == 0 {
        // removing data_count == 0 as a boolean check made me feel smart lol
        return Err(StatsError::InvalidInputValue)
    }

    // invalid input weight or data
    if data.iter().any(|&val| val.is_nan() || val.is_infinite()) ||
        weights.iter().any(|&val| val.is_nan() || val.is_infinite()) {
        return Err(StatsError::InvalidInputValue);
    }

    let numerator_cum_sum: f64 = data.iter()
        .zip(weights.iter()).map(|(&d, &w)| w * d).sum();

    let denominator_cum_sum: f64 = weights.iter().sum();

    if denominator_cum_sum == 0.0 {
        return Err(StatsError::InvalidInputValue);
    }

    Ok(numerator_cum_sum / denominator_cum_sum)
}


pub fn median(data: &[f64]) -> Result<f64, StatsError> {
    let count = data.len();
    if count == 0 {
        return Err(StatsError::EmptyDataSet);
    }

    // Check for NaN or Infinity before sorting
    if data.iter().any(|&val| val.is_nan() || val.is_infinite()) {
        return Err(StatsError::InvalidInputValue);
    }

    println!("Original Array {:?} & Array Len {}", &data, count);

    // It's safe to sort now
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    println!("sorted array {:?}", sorted_data);

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

    let count = data.len();
    let mean_value = mean(data)?;

    if count < 2 {
        return Err(StatsError::EmptyDataSet);
    }

    // Check for invalid values
    if data.iter().any(|&val| val.is_nan() || val.is_infinite()) {
        return Err(StatsError::InvalidInputValue);
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

pub fn median_absolute_deviation(data: &[f64]) -> Result<f64, StatsError> {
    // some implementation for this ... never heard of this before but seems like it could be useful
    // L
}

pub fn trimmed_variance(data: &[f64]) -> Result<f64, StatsError> {
    // Trimmed Var, analagous to trimmed mean.
}



// median absolute deviation
// IQR -- percentile macro?
//
// Mean Absolute difference







