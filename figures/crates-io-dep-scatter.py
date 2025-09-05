import csv
import importlib
import numpy as np

pypi_mod = importlib.import_module("pypi-org-used-languages", package=None)


if __name__ == "__main__":
    res = []
    with open("crate-info-puller/crate-lists/deplist.csv", newline="") as csvfile:
        reader = csv.reader(csvfile, delimiter=",")
        for n, row in enumerate(reader):
            if n != 0:
                res.append(row)

    names = [r[0] for r in res]
    has_deps = [int(r[1]) for r in res]
    is_deps = [int(r[2]) for r in res]
    total_downloads = [int(r[3]) for r in res]

    fig, ax = pypi_mod.prepare_fig_ax()

    c = np.array(total_downloads, dtype=float)
    c /= np.min(c)
    c = np.log(c)
    ax.scatter(has_deps, is_deps, s=30 * c, c=c, alpha=0.7)

    # ax.set_xscale("log")
    ax.set_yscale("log")

    ax.set_xlabel("Has Dependencies")
    ax.set_ylabel("Is Dependency")

    # for name, deps, downloads in res:
    #     ax.annotate(name, (downloads, deps))

    fig.savefig("paper/figures/crates-io-deps-downloads-scatter.png")
    fig.savefig("paper/figures/crates-io-deps-downloads-scatter.pdf")
    fig.savefig("paper/figures/crates-io-deps-downloads-scatter.svg")
