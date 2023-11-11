// Unit Tests
use crate::descriptive_statistics::mean;
use crate::descriptive_statistics::median;
use crate::descriptive_statistics::errors::*;



#[cfg(test)]
mod tests {
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