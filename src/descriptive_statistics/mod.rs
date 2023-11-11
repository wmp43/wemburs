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

pub fn sorted_mean(data: &[f64], trim_n: usize) -> Result<f64, StatsError> {
    let count = data.len();

    // Check that there's enough data to trim
    if count < 2 * trim_n {
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
    let trimmed_data = &sorted_data[trim_n..count - trim_n];

    // Calculate the mean of the trimmed data
    let sum: f64 = trimmed_data.iter().sum();
    let trimmed_count = trimmed_data.len();

    Ok(sum / trimmed_count as f64)
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







