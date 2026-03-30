param(
    [Parameter(Mandatory = $true)]
    [string]$InputDir,

    [Parameter(Mandatory = $true)]
    [string]$OutputDir
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Release-ComObject {
    param([object]$ComObject)

    if ($null -ne $ComObject) {
        [void][System.Runtime.InteropServices.Marshal]::ReleaseComObject($ComObject)
    }
}

$resolvedInput = Resolve-Path -Path $InputDir
if (-not (Test-Path -Path $resolvedInput -PathType Container)) {
    throw "Input directory does not exist: $InputDir"
}

if (-not (Test-Path -Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

$powerPoint = $null
$presentations = $null

try {
    $powerPoint = New-Object -ComObject PowerPoint.Application
    $powerPoint.Visible = 1
    $presentations = $powerPoint.Presentations

    Get-ChildItem -Path $resolvedInput -Filter *.pptx | Sort-Object Name | ForEach-Object {
        $deck = $null
        $deckName = [System.IO.Path]::GetFileNameWithoutExtension($_.Name)
        $deckOutput = Join-Path $OutputDir $deckName

        if (-not (Test-Path -Path $deckOutput)) {
            New-Item -ItemType Directory -Path $deckOutput | Out-Null
        }

        try {
            $deck = $presentations.Open($_.FullName, $false, $true, $false)
            $deck.SaveAs($deckOutput, 18)
        }
        finally {
            if ($null -ne $deck) {
                $deck.Close()
                Release-ComObject $deck
            }
        }
    }
}
finally {
    if ($null -ne $presentations) {
        Release-ComObject $presentations
    }
    if ($null -ne $powerPoint) {
        $powerPoint.Quit()
        Release-ComObject $powerPoint
    }
    [GC]::Collect()
    [GC]::WaitForPendingFinalizers()
}
