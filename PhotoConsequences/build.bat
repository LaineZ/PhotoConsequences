rmdir /S /Q bin
dotnet publish -c Release -p:Platform="x86" -r win-x86
dotnet publish -c Release -p:Platform="x64" -r win-x64
pause