param(
    [Parameter(Mandatory=$true,Position=0)][string]$Auftrag,
    [string]$Python = $env:SPRITE_AGENT_PYTHON,
    [string]$ComfyUrl = 'http://127.0.0.1:8188'
)
$ErrorActionPreference='Stop'
if (-not $Python) {
    $Python='L:/Comfy-Desktop/ComfyUI-Installs/ComfyUI/ComfyUI/.venv/Scripts/python.exe'
}
if (-not (Test-Path -LiteralPath $Python)) { throw 'Python fehlt. SPRITE_AGENT_PYTHON auf die vorhandene Python-Umgebung setzen.' }
Push-Location (Join-Path $PSScriptRoot '../..')
try {
    & $Python -m tools.sprite_agent request $Auftrag --url $ComfyUrl
    if ($LASTEXITCODE -ne 0) { throw 'Sprite-Auftrag abgebrochen; Meldung und Journal prüfen.' }
} finally { Pop-Location }
