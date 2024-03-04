#!/bin/sh
time build-wrapper-linux-x86-64 --out-dir build ./run-sonarqube-build-CWE415.sh
time sonar-scanner --define sonar.cfamily.build-wrapper-output=build
./run-sonarqube-download.sh