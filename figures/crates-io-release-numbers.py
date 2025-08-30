import csv
import importlib
import scipy as sp

pypi_mod = importlib.import_module("pypi-org-used-languages", package=None)

if __name__ == "__main__":
    res = []
    with open("crate-info-puller/crate-lists/releases.csv", newline="") as csvfile:
        reader = csv.reader(csvfile, delimiter=",")
        for n, row in enumerate(reader):
            if n != 0:
                res.append(row)

    x = [r[0] for r in res]
    y = [int(r[1]) for r in res]

    fig, ax = pypi_mod.prepare_fig_ax()

    ax.plot(x[1:], y[1:], color="#CE412B", label="Monthly Releases")

    ticks = ax.get_xticks()
    labels = ax.get_xticklabels()

    newticks = []
    newlabels = []
    for tick, label in zip(ticks, labels):
        split = label.get_text().split("/")
        if split[0] == "1" and int(split[1]) % 2 == 1:
            newticks.append(tick)
            newlabels.append(split[1])

    # Do linear fit
    x_fit = list(range(len(x) - 1))
    popt, pcov = sp.optimize.curve_fit(
        lambda x, a, b, c: a * x**2 + b * x + c, x_fit, y[1:], p0=(1, 0, 0)
    )
    a, b, c = popt
    da = pcov[0][0] ** 0.5
    db = pcov[1][1] ** 0.5
    dc = pcov[2][2] ** 0.5

    y_fit = [a * xi**2 + b * xi + c for xi in x_fit]
    # y_fit_lower = [(a - da) * xi**2 + (b - db) * xi + c - dc for xi in x_fit]
    # y_fit_upper = [(a + da) * xi**2 + (b + db) * xi + c + dc for xi in x_fit]

    ax.plot(x[1:], y_fit, color="#00599d", linestyle="--", label="Quadratic Fit")
    # ax.fill_between(x[1:], y_fit_lower, y_fit_upper, color="#00599d", alpha=0.2)

    ax.set_xticks(newticks, newlabels)

    ax.legend(
        ncol=2,
        frameon=False,
        loc="upper center",
        bbox_to_anchor=(0.5, 1.15),
    )

    fig.savefig("figures/crates-io-release-numbers.png")
    fig.savefig("figures/crates-io-release-numbers.pdf")
    fig.savefig("figures/crates-io-release-numbers.svg")
