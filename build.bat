@echo off

:: Build release
cargo build --release

:: Run the program
.\dist\ferrum.exe run example.fm
