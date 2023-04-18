# Flip horizontaly, rotate 180
# Convert to ppm3, remove first and third line
cp $1 org/
name=$(echo "$1" | cut -f1 -d".")
convert $1 -flop $1
convert $1 -rotate 180 $1
convert $1 -compress none $name.ppm
mv $name.ppm $name.raw
tex=($name.raw)
sed -e '1d;3d' "$tex" > "$tex.tmp" && mv "$tex.tmp" "$tex"
rm $1
