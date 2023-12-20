// linear models mod.rs

// macro to ensure data makes sense for a linear regression model

// trait for linear models?

// Methods to fit and predict?

// or just a function?

// Define your macro for data validation here
use ndarray::{Array1, ArrayBase, Data, Ix1};

pub trait LinearModel {
    fn fit(&mut self, features: &Array2<f64>, target: &Array1<f64>);
    fn predict(&self, features: &Array2<f64>) -> Array1<f64>;
    fn score(&self, features: &Array2<f64>, metric: &str) -> f64;
    fn coef(&self) -> &Vec<f64>;
    //fn visualize(&self) -> visualization of linear model.
    // Don't know how this will work in practice
}
pub trait Scorer {
    fn score(&self, predictions: &Array1<f64>, actuals: &Array1<f64>) -> f64;
}

#[macro_export] macro_rules! validate_linear_model_input {
    ($features:expr, $target:expr) => {
        if $features.is_empty() {
            return Err(LinearModelError::EmptyDataSet.into());
        }
        if $features.ncols() < 1 || $features.ncols() > 100 {
            return Err(LinearModelError::InvalidFeatures.into());
        }
        if $features.genrows().into_iter().any(|row| row.len() != $features.ncols()) {
            return Err(LinearModelError::InconsistentLength.into());
        }
        if $features.iter().any(|&val| val.is_nan() || val.is_infinite()) {
            return Err(LinearModelError::InvalidInputValue.into());
        }
        if $target.is_empty() {
            return Err(LinearModelError::EmptyDataSet.into());
        }
        if $target.ndim() != 1 || $target.len() != $features.nrows() {
            return Err(LinearModelError::InconsistentLength.into());
        }
    };
}

pub struct LinearRegression {
    coef: Vec<f64>,
    scorer: Box<dyn Scorer>,
    // must build new error codes for if the scorer method inst valid
    // regression scorers for Logit
    // Logit Scorers for regressions
}
pub struct LogisticRegression {
    coef: Vec<f64>,
    scorer: Box<dyn Scorer>
}

struct MeanSquaredErrorScorer;


impl Scorer for MeanSquaredErrorScorer {
    fn score(&self, predictions: &Array1<f64>, actuals: &Array1<f64>) -> f64 {
        // zip method
        // validate_linear_model_input!(scorer, $items)
        let mse = predictions.iter()
            .zip(actuals.iter())
            .map(|(pred, actual)| (pred - actual).powi(2))
            .sum::<f64>() / predictions.len() as f64;

        Ok(mse)
    }
}
// Input:
// X = Matrix of input features (each row is a training example, each column is a feature)
// y = Vector of target values
// alpha = Learning rate
// iterations = Number of iterations for the gradient descent
//
// Output:
// beta = Vector of fitted coefficients (including intercept)
//
// Procedure:
// 1: Initialize beta (e.g., to zeros or small random numbers)
// 2: for iter in 1 to iterations do
// 3:     predictions = X * beta               // Calculate predictions for the current beta
// 4:     errors = predictions - y             // Compute the difference between predictions and actual values
// 5:     for j in 1 to length(beta) do
// 6:         gradient_j = sum(errors * X[:, j]) / length(y)  // Calculate the gradient for beta_j
// 7:         beta[j] = beta[j] - alpha * gradient_j          // Update beta_j
// 8:     end for
// 9: end for

impl LinearModel for LinearRegression {
    fn fit(&mut self, features: &Array2<f64>, target: &Array1<f64>) {
        let learning_rate = 0.01;
        let iterations = 1000;
        let m = features.nrows() as f64;

        self.coef = vec![0.0; features.ncols() + 1];

        for _ in 0..iterations {
            let predictions = self.predict(features);
            let errors = &predictions - target;
            let gradient = features.t().dot(&errors) / m;

            for (c, g) in self.coef.iter_mut().zip(gradient.iter()) {
                *c -= learning_rate * g;
            }
        }
    }

    fn predict(&self, features: &Array2<f64>) -> Array1<f64> {
        let mut predictions = Array1::zeros(features.nrows());

        for (i, feature_row) in features.outer_iter().enumerate() {
            let mut prediction = self.coef[0]; // Start with the intercept
            for (feature, &coef) in feature_row.iter().zip(&self.coef[1..]) {
                prediction += feature * coef;
            }
            predictions[i] = prediction;
        }
        predictions
    }

    fn score(&self, features: &Array2<f64>, actuals: &Array1<f64>) -> f64 {
        let predictions = self.predict(features);
        self.scorer.score(&predictions, actuals)
    }

    fn coef(&self) -> &Vec<f64> {
        &self.coef
    }
}