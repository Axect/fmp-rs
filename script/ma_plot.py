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

# Plot params
pparam = dict(
    xlabel = r'Date',
    ylabel = r'Price',
    xscale = 'linear',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(x, tp, label='Typical Price')
    ax.plot(x, sma, '--', label='SMA(20)')
    ax.plot(x, ema, '-.', label='EMA(20)')
    ax.plot(x, wma, ':', label='WMA(20)')
    # Tick angle
    plt.setp(ax.get_xticklabels(), rotation=30, horizontalalignment='right')
    ax.legend()
    fig.savefig('figs/ma.png', dpi=600, bbox_inches='tight')
