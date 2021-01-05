name="$1"
path="$2"
h5dump -a "$name" "$path" | grep "(0)" | tr -s " " | cut -d " " -f 3
