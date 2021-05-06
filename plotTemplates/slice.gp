load "config/default.gp"
load "config/moreland.pal"
set xrange [minX:maxX]
set yrange [minY:maxY]
set cbrange [minC:maxC]
if (logPlot) {
    set logscale cb
}
else {
}
plot for [file in files] file using 1:2:3 w image