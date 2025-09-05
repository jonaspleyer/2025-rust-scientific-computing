import csv
import importlib
import numpy as np

pypi_mod = importlib.import_module("pypi-org-used-languages", package=None)

data = [
    # ["name", "has_deps", "is_dep", "total_downloads"],
    ["num", 6, 19633, 119630053],
    ["gnuplot", 4, 544, 585544],
    ["serde", 3, 650402, 628242900],
    ["cmake", 1, 7472, 92946595],
    ["mpi", 11, 122, 90339],
    ["rayon", 5, 78086, 238988757],
    ["ndarray", 16, 14545, 40638734],
    ["plotlib", 2, 85, 203549],
    ["rug", 13, 1921, 1377986],
    ["pyo3", 46, 12277, 97353000],
    ["tracing", 8, 221252, 345805880],
    ["argmin", 18, 576, 1251255],
    ["logging", 1, 1, 11410],
    ["arrow", 23, 8169, 28146509],
    ["wgpu", 29, 6356, 10860534],
    ["plotters", 24, 2625, 96692353],
    ["cargo-upgrades", 10, 3, 65641],
    ["honeycomb", 0, 10, 8101],
    ["maturin", 75, 76, 622927],
    ["salva2d", 14, 4, 14770],
    ["salva3d", 14, 4, 16044],
    ["bevy", 35, 15031, 2941320],
    ["polars", 25, 3516, 5432011],
    ["extendr-api", 11, 142, 180917],
    ["rapier3d", 23, 445, 688073],
    ["rapier2d", 23, 261, 552988],
    ["smartcore", 14, 624, 163519],
    ["opencl3", 4, 110, 269001],
    ["dioxus", 28, 1291, 658806],
    ["rapier3d-f64", 23, 43, 2050477],
    ["roqoqo", 27, 678, 215319],
    ["qoqo", 23, 233, 168792],
    ["burn", 12, 454, 343818],
    ["cudarc", 5, 372, 688009],
    ["hdt", 19, 20, 38035],
    ["struqture", 18, 264, 186353],
    ["faer", 46, 626, 558946],
    ["symbolica", 27, 48, 32589],
    ["lace", 30, 3, 16433],
    ["stochastic-rs", 49, 5, 53011],
    ["c3dio", 2, 18, 11907],
    ["zarrs", 53, 61, 81335],
    ["qhull", 4, 10, 8005],
    ["diffsol", 16, 55, 34307],
    ["rlst", 23, 24, 13100],
    ["ndelement", 12, 7, 7448],
    ["ndgrid", 12, 1, 6219],
    ["ninterp", 12, 25, 24679],
]

if __name__ == "__main__":
    res = []
    with open("crate-info-puller/crate-lists/deplist.csv", newline="") as csvfile:
        reader = csv.reader(csvfile, delimiter=",")
        for n, row in enumerate(reader):
            if n != 0:
                res.append(row)
    res = data

    names = [r[0] for r in res]
    has_deps = [int(r[1]) for r in res]
    is_deps = [int(r[2]) for r in res]
    total_downloads = [int(r[3]) for r in res]

    fig, ax = pypi_mod.prepare_fig_ax()

    c = np.array(total_downloads, dtype=float)
    c /= np.min(c)
    c = np.log(c)
    ax.scatter(has_deps, is_deps, c=c)

    # ax.set_xscale("log")
    ax.set_yscale("log")

    ax.set_xlabel("Has Dependencies")
    ax.set_ylabel("Is Dependency")

    # for name, deps, downloads in res:
    #     ax.annotate(name, (downloads, deps))

    fig.savefig("paper/figures/crates-io-deps-downloads-scatter.png")
    fig.savefig("paper/figures/crates-io-deps-downloads-scatter.pdf")
    fig.savefig("paper/figures/crates-io-deps-downloads-scatter.svg")
