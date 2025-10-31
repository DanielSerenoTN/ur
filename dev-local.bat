@echo off

echo Building URVIC Full Stack for Local Development...

REM Build Frontend
echo Building React Frontend...
cd urvic-front
call npm ci
call npm run build
cd ..

REM Copy to static folder
echo Copying frontend build to static folder...
if exist static rmdir /s /q static
mkdir static
xcopy urvic-front\dist\* static\ /s /e /y

REM Build Backend
echo Building Rust Backend...
cargo build --release

echo Full stack build completed!
echo Frontend files copied to .\static\
echo Backend binary at .\target\release\urvic-backend.exe
echo.
echo To run: .\target\release\urvic-backend.exe
echo Then visit: http://localhost:20090

pause
