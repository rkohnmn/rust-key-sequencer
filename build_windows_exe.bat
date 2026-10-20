@echo off
title Rust Key Sequencer - One-Click EXE Builder
echo ========================================================
echo   Rust Key ^& Mouse Sequencer - Windows .EXE Builder
echo ========================================================
echo.

where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [!] Rust / Cargo was not found on your PATH.
    echo     Install Rust from https://rustup.rs/ and re-run this script.
    pause
    exit /b 1
)

echo [*] Compiling optimized release executable...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    if not exist "dist" mkdir "dist"
    copy /Y "target\release\rust_key_sequencer.exe" "dist\rust_key_sequencer.exe" >nul
    echo.
    echo [SUCCESS] Compiled executable ready at:
    echo   - dist\rust_key_sequencer.exe
    echo   - target\release\rust_key_sequencer.exe
    echo.
    echo Launching rust_key_sequencer.exe...
    start "" "dist\rust_key_sequencer.exe"
) else (
    echo.
    echo [ERROR] Build failed. Check the compiler output above.
)
pause
