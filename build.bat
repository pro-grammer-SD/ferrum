@echo off
echo Building Ferrum compiler...
cargo build --release

set "SRC=target\release\ferrum_compiler.exe"
set "DEST=dist\ferrum_compiler.exe"

if not exist dist (
    mkdir dist
)

copy /Y "%SRC%" "%DEST%"

echo Build complete! Executable copied to dist\ferrum_compiler.exe

:: Run the example
dist\ferrum_compiler.exe examples/demo.fm

echo Ran demo.fm example!
pause
