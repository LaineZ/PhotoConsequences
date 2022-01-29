mkdir releases

rmdir /S /Q bin
rmdir /S /Q releases\win-x86-bundle
rmdir /S /Q releases\win-x64-bundle

dotnet publish -c Release -p:Platform="x86" -r win-x86
dotnet publish -c Release -p:Platform="x64" -r win-x64

mkdir releases\win-x86-bundle
mkdir releases\win-x64-bundle

copy bin\x86\Release\net5.0-windows\win-x86\publish\ releases\win-x86-bundle
copy bin\x64\Release\net5.0-windows\win-x64\publish\ releases\win-x64-bundle

del releases\win-x86-bundle.zip
del releases\win-x64-bundle.zip

powershell.exe -nologo -noprofile -command "Compress-Archive -Path releases\win-x86-bundle -DestinationPath releases\win-x86-bundle.zip"
powershell.exe -nologo -noprofile -command "Compress-Archive -Path releases\win-x64-bundle -DestinationPath releases\win-x64-bundle.zip"

pause