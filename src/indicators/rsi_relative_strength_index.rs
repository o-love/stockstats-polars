use polars::prelude::{EWMOptions, *};
use std::ops::Neg;

/// Convenience alias to match the idea of `by: By | None` in Python.
pub type By = Vec<Expr>;

pub fn rsi(x: Expr, timeframe: usize, by: Option<By>) -> Expr {
    let x = x.cast(DataType::Float64);

    let delta = x.clone() - x.shift(lit(1));

    let pos = delta.clone().gt(lit(0.0)).fill_null(lit(false));
    let neg = delta.clone().lt(lit(0.0)).fill_null(lit(false));

    let gain = when(pos).then(delta.clone()).otherwise(lit(0.0));
    let loss = when(neg).then(delta.neg()).otherwise(lit(0.0));

    // Wilder's smoothing ~= EWM(alpha = 1/n)
    let ewm = EWMOptions {
        alpha: 1.0 / timeframe as f64,
        adjust: true,
        bias: false,
        ignore_nulls: true,
        min_periods: 1,
    };

    let avg_gain = gain.clone().ewm_mean(ewm.clone());
    let avg_loss = loss.clone().ewm_mean(ewm);

    // RSI = 100 - 100 / (1 + RS)
    let rs = avg_gain / avg_loss;
    let rsi = lit(100.0) - (lit(100.0) / (lit(1.0) + rs));

    match by {
        Some(keys) if !keys.is_empty() => rsi.over(keys),
        _ => rsi,
    }
}
