echo "[" > results-sonarqube.json
i=1
while true; do
    wget "https://sonarcloud.io/api/issues/search?projects=ingo-sonar_juliet-test-suite-c-cplusplus&impactSoftwareQualities=RELIABILITY&ps=500&p=$i" -O - | jq '.issues | map(. | select(.message == "Attempt to free released memory"))' > issue.json
    if [[ `wc -l issue.json` != "1 issue.json" ]]; then 
        if [ $i -ne 1 ]; then 
            echo "," >> results-sonarqube.json
        fi
        cat issue.json >> results-sonarqube.json
    else
        break;
    fi
    i=$(expr $i + 1)
done

echo "]" >> results-sonarqube.json

cat results-sonarqube.json | jq "[.[]]" > results-sonarqube2.json
mv results-sonarqube2.json results-sonarqube.json
rm issue.json
