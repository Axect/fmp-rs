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
rsi = df['rsi']
rsi_signal = df['rsi_signal']
macd = df['macd'] / 1000
macd_signal = df['macd_signal'] / 1000
adx = df['adx']
di_plus = df['di_plus']
di_minus = df['di_minus']
k = df['k']
d = df['d']
#rsi2 = dg['rsi']

# Plot
with plt.style.context(["science", "nature"]):
    fig, axs = plt.subplots(6, 1, figsize=(6, 8), sharex=True, gridspec_kw={'height_ratios': [4, 1, 1, 1, 1, 1]})
    axs[0].autoscale(tight=True)
    axs[0].plot(x, tp, label='Typical Price')
    axs[0].plot(x, sma, '--', label='SMA(20)')
    axs[0].plot(x, ema, '-.', label='EMA(20)')
    axs[0].plot(x, wma, ':', label='WMA(20)')
    # Tick angle
    plt.setp(axs[0].get_xticklabels(), rotation=30, horizontalalignment='right')
    axs[0].legend()
    axs[0].grid(True)
    axs[0].set_ylabel("Price")

    axs[1].autoscale(tight=True)
    axs[1].plot(x, rsi)
    axs[1].plot(x, rsi_signal, '-.')
    axs[1].axhline(y=30, color='r', linestyle='--')
    axs[1].axhline(y=70, color='r', linestyle='--')
    axs[1].set_ylabel("RSI")
    axs[1].set_ylim([10, 90])
    axs[1].grid(True)

    axs[2].autoscale(tight=True)
    #axs[2].plot(x, macd)
    #axs[2].plot(x, macd_signal, '-.')
    axs[2].axhline(y=0, color='k', linestyle='--')
    # Histogram for MACD
    macd_up = np.array(macd - macd_signal)
    macd_up[macd_up < 0] = np.nan
    macd_down = np.array(macd - macd_signal)
    macd_down[macd_down > 0] = np.nan
    axs[2].bar(x, macd_up, color='r', width=0.8)
    axs[2].bar(x, macd_down, color='b', width=0.8)
    axs[2].set_ylabel("MACD/1000")
    axs[2].grid(True)

    axs[3].autoscale(tight=True)
    axs[3].plot(x, adx, 'k')
    axs[3].axhline(y=20, color='b', linestyle='--')
    axs[3].axhline(y=25, color='r', linestyle='--')
    axs[3].set_ylabel("ADX")
    axs[3].set_ylim([0, 80])
    axs[3].grid(True)

    axs[4].autoscale(tight=True)
    di_up = np.array(di_plus - di_minus)
    di_up[di_up < 0] = np.nan
    di_down = np.array(di_plus - di_minus)
    di_down[di_down > 0] = np.nan
    axs[4].bar(x, di_up, color='r', width=0.8)
    axs[4].bar(x, di_down, color='b', width=0.8)
    axs[4].set_ylabel("DI+/-")
    axs[4].grid(True)

    axs[5].autoscale(tight=True)
    axs[5].plot(x, k, label='\%K')
    axs[5].plot(x, d, label='\%D')
    axs[5].axhline(y=20, color='b', linestyle='--')
    axs[5].axhline(y=80, color='r', linestyle='--')
    axs[5].legend()
    axs[5].set_xlabel("Date")
    axs[5].set_ylabel("Stochastic")
    axs[5].set_ylim([0, 100])
    axs[5].grid(True)

    fig.tight_layout()
    fig.savefig('figs/005930.KS.png', dpi=300, bbox_inches='tight')
