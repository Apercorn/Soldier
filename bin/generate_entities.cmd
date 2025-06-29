@echo off
setlocal enabledelayedexpansion

REM Change to the script's directory
cd /d "%~dp0.."

REM Read DATABASE_URL from crates/core/.env
for /f "usebackq tokens=1,2 delims==" %%A in ("crates/core/.env") do (
  if "%%A"=="DATABASE_URL" set DATABASE_URL=%%B
)

if not defined DATABASE_URL (
  echo DATABASE_URL not found in crates/core/.env
  exit /b 1
)

sea-orm-cli generate entity -u !DATABASE_URL! -o crates/core/src/entity
