import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('data/005930.KS.parquet')
#dg = pd.read_parquet('data/005930.KS.rsi.parquet')

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
rsi = df['rsi']
rsi_signal = df['rsi_signal']
#rsi2 = dg['rsi']

# Plot
with plt.style.context(["science", "nature"]):
    fig, axs = plt.subplots(3, 1, figsize=(8, 8), sharex=True, gridspec_kw={'height_ratios': [3, 1, 1]})
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
    axs[1].set_ylabel("Williams \%R")
    axs[1].set_ylim([-100, 0])
    axs[1].grid(True)

    axs[2].autoscale(tight=True)
    axs[2].plot(x, rsi)
    #axs[2].plot(x, rsi2, '--')
    axs[2].plot(x, rsi_signal, '-.')
    axs[2].axhline(y=30, color='r', linestyle='--')
    axs[2].axhline(y=70, color='r', linestyle='--')
    axs[2].set_xlabel("Date")
    axs[2].set_ylabel("RSI")
    axs[2].set_ylim([0, 100])
    axs[2].grid(True)

    fig.tight_layout()
    fig.savefig('figs/005930.KS.png', dpi=300, bbox_inches='tight')
