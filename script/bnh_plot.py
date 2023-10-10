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
df_summary = pd.read_parquet(f'data/{symbol}_bnh_summary.parquet')

# Prepare Data to Plot
x = df['date'] # str
x = pd.to_datetime(x) # datetime
bnh_cr = df['bnh_cr']
bnh_vol = df['bnh_vol']
bnh_sr = df['bnh_sr']
bnh_dd = df['bnh_dd']

# Plot
with plt.style.context(["science", "nature"]):
    fig, axs = plt.subplots(5, 1, figsize=(6, 8), sharex=True, gridspec_kw={'height_ratios': [1,2,2,2,2]})
    fig.suptitle(f'Buy and Hold Strategy for {symbol}')

    values = df_summary.values
    for i in range(values.shape[1]):
        values[0][i] = f'{values[0][i]:.4f}'
    axs[0].table(cellText=values, colLabels=df_summary.columns, loc='center', cellLoc='center')
    axs[0].axis('off')

    axs[1].plot(x, bnh_cr)
    axs[1].axhline(y=1, color='k', linestyle='--')
    axs[1].set_ylabel('Cumulative Return')
    axs[1].grid(True)

    bnh_vol_nonzero = bnh_vol[bnh_vol != 0]
    vol_mean = np.mean(bnh_vol_nonzero)
    axs[2].plot(x, bnh_vol)
    axs[2].axhline(y=vol_mean, color='g', linestyle='--', label=f'Mean: {vol_mean:.2f}')
    axs[2].set_ylabel('Volatility (6 month)')
    axs[2].grid(True)
    axs[2].legend()

    bnh_sr_nonzero = bnh_sr[bnh_sr != 0]
    sr_mean = np.mean(bnh_sr_nonzero)
    axs[3].plot(x, bnh_sr)
    axs[3].axhline(y=sr_mean, color='g', linestyle='--', label=f'Mean: {sr_mean:.2f}')
    axs[3].axhline(y=0, color='k', linestyle='--')
    axs[3].set_ylabel('Sharpe Ratio (6 month)')
    axs[3].grid(True)
    axs[3].legend()

    # Underwater plot
    axs[4].plot(x, -bnh_dd * 100, color='r')
    axs[4].axhline(y=0, color='k', linestyle='--')
    axs[4].fill_between(x, -bnh_dd * 100, 0, color='r', alpha=0.3)
    axs[4].axhline(y=-np.max(bnh_dd) * 100, color='b', linestyle='--', label=f'MDD: {-np.max(bnh_dd) * 100:.2f}\%')
    axs[4].set_xlabel('Date')
    axs[4].set_ylabel('Drawdown (\%)')
    axs[4].grid(True)
    axs[4].legend()

    fig.tight_layout()
    # Remove margin between table and plot
    #fig.subplots_adjust(hspace=0)
    fig.align_ylabels()
    plt.savefig(f'figs/{symbol}_bnh.png', dpi=300)

