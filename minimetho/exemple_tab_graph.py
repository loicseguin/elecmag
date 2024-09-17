import numpy as np
import matplotlib
import matplotlib.pyplot as plt
from matplotlib.ticker import MultipleLocator


# Données simulées d'une expérience où on mesure la grandeur du champ
# électrique produit par une particule chargée en fonction de la distance.

k = 8.99e9
q = 42e-9
r = np.arange(0.2, 0.5, 0.03)
r += 0.003 * np.random.random(len(r))
E = k * q / r**2 / 1000
E += 0.6 * np.random.random(len(E))
dr = 0.004 + 0.02 * r
dE = 0.3 + 0.001 * E
x = 1/r**2
dx = 2 * dr / r * x

for ri, dri, Ei, dEi in zip(r, dr, E, dE):
    print(f"{ri:13.3}({dri:.3})  &  {Ei:13.3}({dEi:.3})  \\\\")


a, b = np.polyfit(x, E, 1)
# Droite min
amin, bmin = np.polyfit(x[[0, -1]] + np.array((-dx[0], dx[-1])),
                        a * x[[0, -1]] + b + np.array((dE[0], -dE[-1])),
                        1)
amax, bmax = np.polyfit(x[[0, -1]] + np.array((dx[0], -dx[-1])),
                        a * x[[0, -1]] + b + np.array((-dE[0], dE[-1])),
                        1)

# Bon graphique
# matplotlib.rc("xtick", top=True, direction="in")
# matplotlib.rc("ytick", right=True, direction="in")
fig, ax = plt.subplots(tight_layout=True)
ax.axline((0, b), slope=a, lw=0.75, color='0.1')
ax.axline((0, bmin), slope=amin, lw=0.75, color='0.5')
ax.axline((0, bmax), slope=amax, lw=0.75, color='0.5')
ax.errorbar(x, E, yerr=dE, xerr=dx, fmt='none')
ax.set_xlabel("$1/r^2$ (m${}^{-2}$)")
ax.set_ylabel("$E$ (kN/C)")
ax.grid(which="major", color="0.7")
ax.grid(which="minor", color="0.9")
ax.xaxis.set_minor_locator(MultipleLocator(1))
ax.yaxis.set_minor_locator(MultipleLocator(1))
ax.text(19.7, 6.2, f"$E = {a:.3f}".replace(".", ",") + "r^{-2} + " + f"{b:.3f}$".replace(".", ","))
fig.savefig("exemple_bon_graphique_er2.pdf")
plt.show()
