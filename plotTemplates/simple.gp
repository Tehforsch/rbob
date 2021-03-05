set terminal pngcairo
load "default.gp"
plot for [file in filenames] file using 1:2 with lines