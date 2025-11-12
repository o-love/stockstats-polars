from pathlib import Path
from typing import Union

import polars as pl
from polars.plugins import register_plugin_function

Number = Union[int, float]
By = Union[str, list[str], None]

PLUGIN_PATH = Path(__file__).parent

@pl.api.register_expr_namespace("stockstats")
class StockStatsNS:
    """
    Usage:
        df.with_columns(
            pl.col("close").stockstats.rsi(14).alias("rsi_14"),
            pl.col("close").stockstats.macd().struct.field("macd").alias("macd"),
        )
    The Expr this namespace is bound to is typically a price column (e.g., "close").
    """

    def __init__(self, expr: pl.Expr):
        self._x = expr  # usually close

    def rsi(self, timeframe: int = 14, *, by: By | None = None) -> pl.Expr:
        """
        Wilder-style RSI. Uses ewm smoothing with alpha = 1/n.
        """
        return register_plugin_function(
            plugin_path=PLUGIN_PATH,
            function_name="rsi",
            args=[self._x],
            kwargs={"timeframe": timeframe, "by": by},
        )
