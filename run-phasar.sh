rm -rf phasar
mkdir phasar
cd phasar
time find .. -name 'CWE415*.c*' | while read line; do clang -g -c -emit-llvm -I../testcasesupport $line; done
cargo run --manifest-path ../link-testcases/Cargo.toml -- --folder .

i=0
time (echo "[" && ls *.bc | while read line; do
    if [ "$i" -ne 0 ]; then
        echo ","
    fi
    i=1
    /home/ingo/git/type-2-phasar/build/type-2-phasar --analyses=double-free $line --result-format=sarif
done; echo "]") > ../results-phasar.json