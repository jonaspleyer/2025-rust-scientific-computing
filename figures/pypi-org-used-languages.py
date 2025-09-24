import matplotlib.pyplot as plt


def prepare_fig_ax():
    plt.rcParams.update(
        {
            "font.family": "Courier New",  # monospace font
            "font.size": 20,
            "axes.titlesize": 20,
            "axes.labelsize": 20,
            "xtick.labelsize": 20,
            "ytick.labelsize": 20,
            "legend.fontsize": 20,
            "figure.titlesize": 20,
        }
    )
    fig, ax = plt.subplots(figsize=(8, 6))
    fig.subplots_adjust(
        top=0.9,
        left=0.2,
        right=0.8,
        bottom=0.15,
    )
    ax.grid(True, which="major", linestyle="-", linewidth=0.75, alpha=0.25)
    ax.minorticks_off()
    ax.grid(True, which="minor", linestyle="-", linewidth=0.25, alpha=0.15)
    ax.set_axisbelow(True)
    return fig, ax


data = [
    ("Go", 46, "#00acd7"),
    ("Java", 186, "#c12f30"),
    ("Fortran", 248, "#744e97"),
    ("C", 2013, "#659bd3"),
    ("C++", 2403, "#00599d"),
    ("Rust", 3244, "#CE412B"),
]

fig, ax = prepare_fig_ax()

ax.barh(
    [d[0] for d in data],
    [d[1] for d in data],
    color=[d[2] for d in data],
    align="center",
)
ax.set_xlabel("(B) Number of Packages")
ax.set_title("Packages at pypi.org")

fig.savefig("paper/figures/pypi-org-used-languages.png")
fig.savefig("paper/figures/pypi-org-used-languages.pdf")
fig.savefig("paper/figures/pypi-org-used-languages.svg")
