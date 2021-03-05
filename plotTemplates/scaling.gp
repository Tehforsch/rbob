set terminal pngcairo
set output "asd.png"
plot for [file in files] file using 1:2 title ""