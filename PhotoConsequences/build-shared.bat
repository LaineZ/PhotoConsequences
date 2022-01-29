mkdir releases

rmdir /S /Q bin
rmdir /S /Q releases\win-x86-shared
rmdir /S /Q releases\win-x64-shared

dotnet publish -c Release -p:Platform="x64" -r win-x64 --self-contained false
dotnet publish -c Release -p:Platform="x86" -r win-x86 --self-contained false

mkdir releases\win-x86-shared
mkdir releases\win-x64-shared

copy bin\x86\Release\net5.0-windows\win-x86\publish\ releases\win-x86-shared
copy bin\x64\Release\net5.0-windows\win-x64\publish\ releases\win-x64-shared

del releases\win-x86-shared.zip
del releases\win-x64-shared.zip

powershell.exe -nologo -noprofile -command "Compress-Archive -Path releases\win-x86-shared -DestinationPath releases\win-x86-shared.zip"
powershell.exe -nologo -noprofile -command "Compress-Archive -Path releases\win-x64-shared -DestinationPath releases\win-x64-shared.zip"

pause