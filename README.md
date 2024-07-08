# Execute Scanners on Juliet Test Suite + Compare Results

## Phasar CWE415 Linux
* run-phasar.sh

## Sonarqube CWE415 - Linux
* run-sonarqube.sh

## Sonarqube CWE89 - Windows
* `pacman -S make`
* Install LLVM for Windows including clang, add to path
* Run in Windows MSYS Terminal:
```bash
time /c/Users/T14/Sonar/build-wrapper-win-x86/build-wrapper-win-x86/build-wrapper --out-dir build make
time /c/Users/T14/Sonar/sonar-scanner-cli-5.0.1.3006-windows/sonar-scanner-5.0.1.3006-windows/bin/sonar-scanner.bat --define sonar.cfamily.build-wrapper-output=build
# ./run-sonarqube-download.sh (on linux)
rm -rf build
```
* Run in VS Studio 2022 Developer Command Prompt:
```batch
cd C:\Users\T14\git\juliet-test-suite-c-cplusplus\testcases\CWE114_Process_Control
/c/Users/T14/Sonar/build-wrapper-win-x86/build-wrapper-win-x86/build-wrapper --out-dir build compile_all.bat
sonar-scanner.bat --define sonar.cfamily.build-wrapper-output=build
REM ./run-sonarqube-download.sh (on linux)
rm -rf build
```

## Sonarqube CWE78 - Windows
* see https://docs.sonarsource.com/sonarqube/9.9/analyzing-source-code/languages/c-family/#using-build-wrapper
* Run in Developer Command Prompt:
```batch
cd C:\Users\T14\git\juliet-test-suite-c-cplusplus
<delete build folder>
<delete .scannerwork>
<edit wildcard in compile-all.bat or better delete all other source files>
<delete *.obj and *.lib recursively>
build-wrapper --out-dir build compile_all.bat
sonar-scanner
```