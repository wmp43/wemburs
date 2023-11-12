// Unit Tests
use crate::descriptive_statistics::errors::*;
use crate::descriptive_statistics::*;



#[cfg(test)]
mod tests {
    use super::*;


    mod mean_tests {
        use super::*;

        #[test]
        fn test_mean_empty_array() {
            let data: [f64; 0] = [];
            assert_eq!(mean(&data), Err(StatsError::EmptyDataSet));
        }

        #[test]
        fn test_mean_single_element() {
            let data = [3.0];
            assert_eq!(mean(&data), Ok(3.0));
        }

        #[test]
        fn test_mean_multiple_elements() {
            let data = [1.0, 2.0, 3.0, 4.0, 5.0];
            assert_eq!(mean(&data), Ok(3.0));
        }

        #[test]
        fn test_mean_with_nan() {
            let data = [1.0, 2.0, f64::NAN];
            assert_eq!(mean(&data), Err(StatsError::InvalidInputValue));
        }

        #[test]
        fn test_mean_with_infinity() {
            let data = [1.0, 2.0, f64::INFINITY];
            assert_eq!(mean(&data), Err(StatsError::InvalidInputValue));
        }

    }

    mod median_tests{
        use super::*;
        #[test]
        fn test_median_empty_array() {
            let data: [f64; 0] = [];
            assert_eq!(median(&data), Err(StatsError::EmptyDataSet));
        }

        #[test]
        fn test_median_single_element() {
            let data = [3.0];
            assert_eq!(median(&data), Ok(3.0));
        }

        #[test]
        fn test_median_multiple_elements() {
            let data = [5.0, 2.0, 3.0, 4.0, 1.0];
            assert_eq!(median(&data), Ok(3.0));
        }

        #[test]
        fn test_median_odd_len() {
            let data = [4.0, 2.0, 3.0, 4.0, 1.0];
            assert_eq!(median(&data), Ok(3.0));
        }

        #[test]
        fn test_median_even_len() {
            let data = [5.0, 2.0, 3.0, 4.0];
            assert_eq!(median(&data), Ok(3.5));
        }

        #[test]
        fn test_median_with_nan() {
            let data = [1.0, 2.0, f64::NAN];
            assert_eq!(median(&data), Err(StatsError::InvalidInputValue));
        }

        #[test]
        fn test_median_with_infinity() {
            let data = [1.0, 2.0, f64::INFINITY];
            assert_eq!(median(&data), Err(StatsError::InvalidInputValue));
        }
    }

    mod trimmed_mean_tests {
        use super::*;
        #[test]
        fn test_trimmed_mean_empty_array() {
            let data = [];
            assert_eq!(trimmed_mean(&data, 0.5), Err(StatsError::EmptyDataSet))
        }

        #[test]
        fn test_trimmed_mean_valid_no_trimming() {
            let data = [1.0, 2.0, 3.0, 4.0, 5.0];
            assert_eq!(trimmed_mean(&data, 0.0), Ok(3.0));
        }

        #[test]
        fn test_trimmed_mean_valid_moderate_trimming() {
            let data = [1.0, 2.0, 3.0, 4.0, 5.0];
            assert_eq!(trimmed_mean(&data, 0.1), Ok(3.0)); // Assumes trimming 10% (0.5 elements) from each end rounds to 0 elements
        }

        #[test]
        fn test_trimmed_mean_valid_max_trimming() {
            let data = [1.0, 2.0, 3.0, 4.0, 5.0];
            assert_eq!(trimmed_mean(&data, 0.5), Ok(3.0)); // Trimming 50% should leave the middle element
        }

        #[test]
        fn test_trimmed_mean_invalid_trim_percentage() {
            let data = [1.0, 2.0, 3.0, 4.0, 5.0];
            assert_eq!(trimmed_mean(&data, -0.1), Err(StatsError::InvalidInputValue));
            assert_eq!(trimmed_mean(&data, 1.1), Err(StatsError::InvalidInputValue));
        }
    }

    mod weighted_mean_tests {
        use super::*;

        #[test]
        fn test_weighted_mean_valid_input() {
            let data = [1.0, 2.0, 3.0];
            let weights = [0.5, 1.5, 2.0];
            assert_eq!(weighted_mean(&data, &weights), Ok(2.375));
        }

        #[test]
        fn test_weighted_mean_empty_arrays() {
            let data: [f64; 0] = [];
            let weights: [f64; 0] = [];
            assert_eq!(weighted_mean(&data, &weights), Err(StatsError::InvalidInputValue));
        }

        #[test]
        fn test_weighted_mean_mismatched_lengths() {
            let data = [1.0, 2.0];
            let weights = [1.0];
            assert_eq!(weighted_mean(&data, &weights), Err(StatsError::InvalidInputValue));
        }

        #[test]
        fn test_weighted_mean_data_with_nan() {
            let data = [1.0, f64::NAN];
            let weights = [1.0, 1.0];
            assert_eq!(weighted_mean(&data, &weights), Err(StatsError::InvalidInputValue));
        }

        #[test]
        fn test_weighted_mean_weights_with_nan() {
            let data = [1.0, 2.0];
            let weights = [1.0, f64::NAN];
            assert_eq!(weighted_mean(&data, &weights), Err(StatsError::InvalidInputValue));
        }

        #[test]
        fn test_weighted_mean_zero_sum_of_weights() {
            let data = [1.0, 2.0, 3.0];
            let weights = [0.0, 0.0, 0.0];
            assert_eq!(weighted_mean(&data, &weights), Err(StatsError::InvalidInputValue));
        }
    }

    mod variance_tests {
        use super::*;

        #[test]
        fn test_variance_valid_dataset() {
            let data = [1.0, 2.0, 3.0, 4.0, 5.0];
            let expected_variance = 2.5; // The sample variance of this dataset
            assert_eq!(variance(&data), Ok(expected_variance));
        }

        #[test]
        fn test_variance_empty_dataset() {
            let data: [f64; 0] = [];
            assert_eq!(variance(&data), Err(StatsError::EmptyDataSet));
        }

        #[test]
        fn test_variance_single_element() {
            let data = [3.0];
            assert_eq!(variance(&data), Err(StatsError::EmptyDataSet)); // Or another appropriate error for single element datasets
        }

        #[test]
        fn test_variance_with_nan() {
            let data = [1.0, f64::NAN, 3.0];
            assert_eq!(variance(&data), Err(StatsError::InvalidInputValue));
        }

        #[test]
        fn test_variance_with_infinity() {
            let data = [1.0, f64::INFINITY, 3.0];
            assert_eq!(variance(&data), Err(StatsError::InvalidInputValue));
        }
    }
}