for file in *.rs; do
    rustfmt --emit stdout $file > rustfmt/$file
done
