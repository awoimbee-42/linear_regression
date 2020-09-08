# ft_linear_regression

The first machine learning project at 42.

## Goal

Predict the value of a car according to its mileage (in km), using linear regression.

## How to use

```bash
# You can directly build
cargo build --release
# Or use the Makefile
make compile-local
# Or use the Makefile and docker
make compile-docker

# I recommend to then launch this command
make symlink
```

This project contains 3 executables:

* train: train the model, creates `thetas.csv` & `training_results.svg`
* estimate: estimate the price of a car, using `thetas.csv`
* accuracy: prints the MAE, RMSE, RAE, RRSE, but also what I call nMAE & nRMSE
