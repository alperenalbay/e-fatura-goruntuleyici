@echo off
setlocal
REM Loads the MSVC environment when Visual Studio Build Tools are installed.
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if exist "%VSWHERE%" (
  for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set "VSPATH=%%i"
)
if defined VSPATH if exist "%VSPATH%\VC\Auxiliary\Build\vcvars64.bat" (
  call "%VSPATH%\VC\Auxiliary\Build\vcvars64.bat" >nul
)
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
cd /d "%~dp0"
call npx tauri build %*
