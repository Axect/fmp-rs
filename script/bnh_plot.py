import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import scienceplots
import argparse

args = argparse.ArgumentParser()
args.add_argument('--ticker', type=str, default='005930.KS')
args = args.parse_args()

symbol = args.ticker

# Import parquet file
df = pd.read_parquet(f'data/{symbol}_bnh.parquet')

# Prepare Data to Plot
x = df['date'] # str
x = pd.to_datetime(x) # datetime
bnh = df['bnh']
bnh_cr = df['bnh_cr']
bnh_vol = df['bnh_vol']
bnh_sr = df['bnh_sr']

# Plot
with plt.style.context(["science", "nature"]):
    fig, axs = plt.subplots(4, 1, figsize=(6, 8), sharex=True)
    fig.suptitle(f'Buy and Hold Strategy for {symbol}')

    axs[0].plot(x, bnh)
    axs[0].set_ylabel('Daily Return')
    axs[0].grid(True)

    axs[1].plot(x, bnh_cr)
    axs[1].axhline(y=1, color='r', linestyle='--')
    axs[1].set_ylabel('Cumulative Return')
    axs[1].grid(True)

    bnh_vol_nonzero = bnh_vol[bnh_vol != 0]
    vol_mean = np.mean(bnh_vol_nonzero)
    axs[2].plot(x, bnh_vol)
    axs[2].axhline(y=vol_mean, color='r', linestyle='--', label=f'Mean: {vol_mean:.2f}')
    axs[2].set_ylabel('Volatility (6 month)')
    axs[2].grid(True)
    axs[2].legend()

    bnh_sr_nonzero = bnh_sr[bnh_sr != 0]
    sr_mean = np.mean(bnh_sr_nonzero)
    axs[3].plot(x, bnh_sr)
    axs[3].axhline(y=sr_mean, color='r', linestyle='--', label=f'Mean: {sr_mean:.2f}')
    axs[3].set_xlabel('Date')
    axs[3].set_ylabel('Sharpe Ratio (6 month)')
    axs[3].grid(True)
    axs[3].legend()

    fig.tight_layout()
    fig.align_ylabels()
    plt.savefig(f'figs/{symbol}_bnh.png', dpi=300)

