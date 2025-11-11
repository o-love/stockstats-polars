Namespace extension of Polars with stock statistics/indicator calculation helper

Polars implementation of the **stockstats** idea: compute technical indicators inline on your price (ohlcv) DataFrame with simple access.


# Inspiration

This project is inspired by [**stockstats**](https://github.com/jealous/stockstats).
A Pandas dataframe implementation of stock indicators.

A big thanks and credit to Cedric Zhuang for his work on that library. 
Whose implementations I've based this library upon.

## Building the Python extension

The PyO3 binding lives in `python/stockstats-polars`, but you can drive it from the repo root:

This repo uses [uv](https://github.com/astral-sh/uv) to provide the Python tooling (see
`pyproject.toml`). Before running the helpers, sync the dev dependencies once:

```bash
uv sync --group dev
```

Then you can drive everything from the root:

```bash
# Build the Rust cdylib only (via Cargo workspace)
make py-build

# Install the module into your current Python environment (through uv-managed maturin)
make py-develop

# Produce a wheel/sdist with maturin
make py-wheel

# Run pytest suite (expects the module to be importable, e.g. after make py-develop)
make py-test
```
