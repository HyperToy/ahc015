g++ a.cpp -o a

for i in `seq -f '%04g' 0 $1`
do  
    echo $i
    ./a < tools/in/$i.txt > tools/out/$i.txt
    cd ./tools
    echo $i >> scores.txt
    cargo run --release --bin vis in/$i.txt out/$i.txt >> scores.txt
    mv vis.html vis/$i.html
    cd ../
done