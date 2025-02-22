for file in *.rs; do
    rustfmt --quiet --emit stdout $file > rustfmt/$file
    cargo run --bin prettyplease $file > prettyplease/$file
done
