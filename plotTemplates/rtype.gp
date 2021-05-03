load "config/default.gp"
set xrange [minX:maxX]
set yrange [minY:maxY]

rType(t) = (1-exp(-t))**(1.0/3.0)

plot for [file in files] file using 1:2 title "" w l ls 1, rType(x) w l ls 3