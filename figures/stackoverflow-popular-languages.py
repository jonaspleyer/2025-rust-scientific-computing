import matplotlib as mpl
import importlib

pypi_mod = importlib.import_module("pypi-org-used-languages", package=None)

fig, ax = pypi_mod.prepare_fig_ax()

groups = ["Learners", "Professionals", "Total"]
data_individual = [
    ("Fortran", [2, 0.9, 1.4], "#744e97"),
    ("Rust", [23.1, 14.5, 14.8], "#CE412B"),
    ("Go", [13.1, 17.4, 16.4], "#00acd7"),
    ("C", [48, 19.1, 22], "#659bd3"),
    ("C++", [44.6, 21.8, 23.5], "#00599d"),
    ("Java", [40.8, 29.6, 29.4], "#c12f30"),
]

data = []
for d in data_individual:
    for n, group in enumerate(groups):
        color = mpl.colors.to_rgba(d[2], alpha=(n + 1) / len(groups))
        data.append((f"{d[0]} ({group})", d[1][n], color))

labels = [
    "" if i % 3 != 1 else data_individual[int((i - 1) / 3)][0]
    for i, d in enumerate(data)
]
ax.barh(
    [d[0] for d in data],
    [d[1] for d in data],
    color=[mpl.colors.to_rgba(d[2]) for d in data],
    label=labels,
)
ax.set_yticklabels(labels)  # , horizontalalignment="left")

# handles, labels = ax.get_legend_handles_labels()
# print(handles[0])
handles = [
    mpl.patches.Rectangle(
        xy=(0, 0.6),
        width=0.9,
        height=0.8,
        angle=0,
        facecolor=mpl.colors.to_rgba("#000", alpha=(len(groups) - i) / len(groups)),
    )
    for i in range(len(groups))
]
ax.legend(
    handles,
    groups[::-1],
    ncol=3,
    frameon=False,
    loc="upper center",
    bbox_to_anchor=(0.5, 1.15),
)

# ax.set_title("Language Popularity (stackoverflow.co)")
ax.set_xlabel("(A) Language Usage [%]")

fig.savefig("paper/figures/stackoverflow-popular-languages.png")
fig.savefig("paper/figures/stackoverflow-popular-languages.pdf")
fig.savefig("paper/figures/stackoverflow-popular-languages.svg")
