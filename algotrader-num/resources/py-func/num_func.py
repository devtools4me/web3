from statsmodels.tsa.stattools import coint
import statsmodels.api as sm
import pandas as pd
import numpy as np
import math
import json

def get_zscore(spread, z_score_window):
  df = pd.DataFrame(spread)
  mean = df.rolling(center=False, window=z_score_window).mean()
  std = df.rolling(center=False, window=z_score_window).std()
  x = df.rolling(center=False, window=1).mean()
  df["ZSCORE"] = (x - mean) / std
  return df["ZSCORE"]

def get_spread(series_1, series_2, hedge_ratio):
  spread = pd.Series(series_1) - (pd.Series(series_2) * hedge_ratio)
  return spread

def _spread_z_score(series_1, series_2, hedge_ratio, z_score_window):
  spread = get_spread(series_1, series_2, hedge_ratio)
  z_score = get_zscore(spread, z_score_window)
  return {
    "spread": spread.astype("string").fillna(np.nan).replace([np.nan], "").tolist(),
    "z_score": z_score.astype("string").fillna(np.nan).replace([np.nan], "").tolist()
  }

def get_spread_z_score(obj):
  return _spread_z_score(obj["series_1"], obj["series_2"], obj["hedge_ratio"], obj["z_score_window"])

def get_cointegration(obj):
  return _cointegration(obj["series_1"], obj["series_2"])

def get_cointegration_json(s):
  args = json.loads(s)
  res = get_cointegration(args)
  return json.dumps(res)

def _cointegration(series_1, series_2):
  coint_res = coint(series_1, series_2)
  coint_t = coint_res[0]
  p_value = coint_res[1]
  critical_value = coint_res[2][1]
  model = sm.OLS(series_1, series_2).fit()
  hedge_ratio = model.params[0]
  spread = get_spread(series_1, series_2, hedge_ratio)
  zero_crossings = len(np.where(np.diff(np.sign(spread)))[0])
  return {
    "p_value": p_value,
    "coint_t": coint_t,
    "c_value": critical_value,
    "hedge_ratio": hedge_ratio,
    "zero_crossings": zero_crossings
  }

def get_close_prices(prices):
  close_prices = []
  for price_values in prices:
    if math.isnan(price_values["close"]):
      return []
    close_prices.append(price_values["close"])
  return close_prices
