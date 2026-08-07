# microbiorustr


**microbiorustr** is an R package that provides lightning-fast protein sequence metrics. It serves as a direct R interface to the Rust `seqmetrics` crate (part of the broader `microBioRust` ecosystem), bridging the performance and memory safety of Rust with the data science ecosystem of R using `extendr`.

## Features

* **Amino Acid Counts:** Calculates the exact occurrence of each amino acid in a protein sequence, returned as a sorted R `data.frame`
* **Amino Acid Percentages:** Calculates the relative frequency of each amino acid
* **Aromaticity:** Computes the aromaticity of a sequence (the relative frequency of Phe, Trp, and Tyr) based on Lobry (1994)
* **Instability Index:** Estimate the *in vivo* stability of a protein based on dipeptide weights. The weights data is compiled directly into the Rust binary.

---

## Installation

You can install the development version of `microbiorustr` from GitHub using the `remotes` or `devtools` package:

```r
# install.packages("devtools")
devtools::install_github("LCrossman/microBioRust", subdir = "microbiorust-r") 

## RUNNING on MacOSX - fix (works also if your R is inside a conda environment)
You may need to alter the path in the ~/.Renviron to include rust build flags
Provide the path to lib (PATH_TO_LIB below) which is above R/lib, so your R dylib is inside folder: PATH_TO_LIB/R/lib/

If you have no ~/.Renviron: you can add a new file just containing this line 
RUSTFLAGS="-C link-arg=-Wl,-rpath,PATH_TO_LIB -C link-arg=-Wl,-rpath,PATH_TO_LIB" 
