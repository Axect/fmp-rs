import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('data/005930.KS.parquet')

# Prepare Data to Plot
x = df['date'] # str
x = pd.to_datetime(x) # datetime
tp = df['tp']
sma = df['sma']
ema = df['ema']
wma = df['wma']
dema = df['dema']
tema = df['tema']
williams = df['williams']

# Plot params
pparam = dict(
    xlabel = r'Date',
    ylabel = r'Price',
    xscale = 'linear',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, axs = plt.subplots(2, 1, figsize=(8, 6), sharex=True, gridspec_kw={'height_ratios': [3, 1]}, )
    axs[0].autoscale(tight=True)
    axs[0].plot(x, tp, label='Typical Price')
    axs[0].plot(x, sma, '--', label='SMA(20)')
    axs[0].plot(x, ema, '-.', label='EMA(20)')
    axs[0].plot(x, wma, ':', label='WMA(20)')
    axs[0].plot(x, dema, '--', label='DEMA(20)')
    axs[0].plot(x, tema, '-.', label='TEMA(20)')
    # Tick angle
    plt.setp(axs[0].get_xticklabels(), rotation=30, horizontalalignment='right')
    axs[0].legend()
    axs[0].grid(True)
    axs[0].set_ylabel("Price")

    axs[1].autoscale(tight=True)
    axs[1].plot(x, williams)
    axs[1].axhline(y=-20, color='r', linestyle='--')
    axs[1].axhline(y=-80, color='r', linestyle='--')
    axs[1].set_xlabel("Date")
    axs[1].set_ylabel("Williams \%R")
    axs[1].grid(True)

    fig.tight_layout()
    fig.savefig('figs/005930.KS.png', dpi=300, bbox_inches='tight')