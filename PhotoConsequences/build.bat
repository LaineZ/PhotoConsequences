del /s /q ./bin
dotnet publish -c Release -p:Platform="x86" -r win-x86 /p:PublishTrimmed=true
dotnet publish -c Release -p:Platform="x64" -r win-x64 /p:PublishTrimmed=true
pause