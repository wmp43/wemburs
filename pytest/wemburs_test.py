import wemburs as wmb
import numpy as np
import pytest

# Sample data for testing
test_data = np.array([1.2, 3.1, 5.6, 9.2, 5.4, 7.5, 1.1, 0.3, 4.0], dtype=np.float64)

# A tolerance for floating point comparison
tolerance = 1e-8

@pytest.mark.parametrize("func, expected", [
    (wmb.descriptive_statistics.mean, np.mean(test_data)),
    (wmb.descriptive_statistics.median, np.median(test_data)),

    # Add more functions and expected results as tuples
])
def test_statistical_functions(func, expected):
    result = func(test_data)
    assert np.isclose(result, expected, atol=tolerance), f"Expected {expected}, got {result}"

def test_range():
    expected = np.ptp(test_data)  # Peak-to-peak is a simple range function in NumPy
    result = wmb.descriptive_statistics.range(test_data)
    assert np.isclose(result, expected, atol=tolerance), f"Expected range {expected}, got {result}"

# Add more specific tests for other functions
def test_median_absolute_deviation():
    # Calculate the MAD manually or using numpy
    median = np.median(test_data)
    deviations = np.abs(test_data - median)
    expected = np.median(deviations)
    result = wmb.descriptive_statistics.median_absolute_deviation(test_data)
    assert np.isclose(result, expected, atol=tolerance), f"Expected MAD {expected}, got {result}"


@pytest.mark.parametrize("data, expected_skew", [
    (np.array([2, 8, 0, 4, 1, 9, 9, 0], dtype=np.float64), 0.0),  # symmetric data should have skewness ~0
    # More test cases or compute expected values using scipy.stats.skew
])
def test_skewness(data, expected_skew):
    result = wmb.descriptive_statistics.skewness(data)
    assert np.isclose(result, expected_skew, atol=tolerance), f"Expected skewness {expected_skew}, got {result}"

@pytest.mark.parametrize("data, expected_kurtosis", [
    (np.array([1, 2, 3, 4, 5], dtype=np.float64), -1.3),  # simple dataset
    # More test cases or compute expected values using scipy.stats.kurtosis
])
def test_kurtosis(data, expected_kurtosis):
    result = wmb.descriptive_statistics.kurtosis(data)
    assert np.isclose(result, expected_kurtosis, atol=tolerance), f"Expected kurtosis {expected_kurtosis}, got {result}"

@pytest.mark.parametrize("data, expected_variance", [
    (np.array([1, 2, 3, 4], dtype=np.float64), 1.25),
    (np.array([1.5, 2.5, 4.5], dtype=np.float64), 2.25),
    # More test cases
])
def test_variance(data, expected_variance):
    result = wmb.descriptive_statistics.variance(data)
    assert np.isclose(result, expected_variance, atol=tolerance), f"Expected variance {expected_variance}, got {result}"


# Add tests for error conditions
def test_empty_input():
    with pytest.raises(ValueError):
        wmb.descriptive_statistics.mean(np.array([]))


def test_invalid_input():
    with pytest.raises(ValueError):
        wmb.descriptive_statistics.mean(np.array([np.nan, 0, 10, 5]))

def test_non_numeric_input():
    with pytest.raises(TypeError):
        wmb.descriptive_statistics.mean(np.array(["a", "b", "c"]))


