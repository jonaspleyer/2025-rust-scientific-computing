import matplotlib as mpl
import importlib

pypi_mod = importlib.import_module("pypi-org-used-languages", package=None)

fig1, ax1 = pypi_mod.prepare_fig_ax()
fig2, ax2 = pypi_mod.prepare_fig_ax()

years = [2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025]

# Name, Loved, Wanted, Color
languages = [
    (
        "Rust",
        [73.1, 78.9, 83.5, 86.1, 86.98, 86.73, 84.66, 82.2, 72.4],
        [6.6, 8.3, 9.5, 14.6, 14.09, 17.6, 30.56, 28.7, 29.2],
        "#CE412B",
    ),
    (
        "Go",
        [63.3, 65.6, 67.9, 62.3, 62.74, 64.58, 62.45, 67.7, 56.5],
        [13.5, 16.2, 15.0, 17.9, 14.54, 16.41, 20.59, 23.1, 23.4],
        "#00acd7",
    ),
    (
        "C",
        [41.7, None, 42.5, 33.1, 39.56, 39.68, 43.29, 47.4, 45.0],
        [6.4, 5.9, 5.0, 4.3, 4.52, 4.34, 11.51, 13.9, 14.5],
        "#659bd3",
    ),
    (
        "C++",
        [52.0, 46.7, 52.0, 43.4, 49.24, 48.39, 49.77, 53.1, 46.6],
        [11.8, 10.2, 9.1, 8.6, 8.8, 7.67, 16.35, 18.3, 16.7],
        "#00599d",
    ),
    (
        "Java",
        [50.5, 50.7, 53.4, 44.1, 47.15, 45.75, 44.11, 47.6, 41.8],
        [11.7, 10.5, 8.3, 8.8, 6.79, 5.6, 16.53, 17.9, 15.8],
        "#c12f30",
    ),
]

for name, loved, wanted, color in languages:
    ax1.plot(years, loved, label=name, color=color)
    ax2.plot(years, wanted, label=name, color=color)

labels = ["Most Loved Language", "Most Desired Language"]
for n, ax in enumerate([ax1, ax2]):
    ax.set_title(labels[n])
    ax.set_xlabel("Year")
    ax.set_ylabel("Developers [%]")
    ax.legend(
        ncol=1,
        frameon=False,
        loc="center",
        bbox_to_anchor=(1.17, 0.5),
    )

fig1.savefig("paper/figures/stackoverflow-loved-language.png")
fig1.savefig("paper/figures/stackoverflow-loved-language.pdf")
fig1.savefig("paper/figures/stackoverflow-loved-language.svg")

fig2.savefig("paper/figures/stackoverflow-desired-language.png")
fig2.savefig("paper/figures/stackoverflow-desired-language.pdf")
fig2.savefig("paper/figures/stackoverflow-desired-language.svg")
