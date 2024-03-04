rm -rf phasar
mkdir phasar
cd phasar
find -name '../CWE415*.c*' | while read line; do clang -g -S -emit-llvm $line; done
i=0
(echo "[" && ls *.ll | while read line; do 
    if [ "$i" -ne 0 ]; then
        echo ","
    fi
    i=1
    /home/ingo/git/type-2-phasar/build/type-2-phasar --analyses=double-free $line --result-format=sarif
done; echo "]") > phasar.json