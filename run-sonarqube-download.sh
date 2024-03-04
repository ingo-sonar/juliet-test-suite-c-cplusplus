echo "[" > issues.json
i=1
while true; do
    wget "https://sonarcloud.io/api/issues/search?projects=ingo-sonar_juliet-test-suite-c-cplusplus&impactSoftwareQualities=RELIABILITY&ps=500&p=$i" -O - | jq '.issues' > issue.json
    if [[ `wc -l issue.json` != "1 issue.json" ]]; then 
        if [ $i -ne 1 ]; then 
            echo "," >> issues.json
        fi
        cat issue.json >> issues.json
    else
        break;
    fi
    i=$(expr $i + 1)
done

echo "]" >> issues.json

cat issues.json | jq "[.[]]" > issues2.json
mv issues2.json issues.json
rm issue.json
