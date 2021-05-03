load "config/default.gp"
set xrange [minX:maxX]
set yrange [minY:maxY]

dType(t) = ((1 + 7 / 4 * (t-startTimeAnalytical) / stroemgrenRadius * soundSpeed) ** (4.0 / 7.0))
# print(word(files, 1))
soundSpeed = 2
fit[startTimeAnalytical:] dType(x) word(files, 1) using 1:2 via soundSpeed
print("Sound speed:", soundSpeed * stroemgrenRadius / recombinationTime)

plot for [file in files] file using 1:2 title "" w l ls 1, dType(x) w l ls 3