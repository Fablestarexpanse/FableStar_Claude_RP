@echo off
cd /d "%~dp0"
echo Starting WorldWeaver in development mode...
npm run tauri dev
pause
