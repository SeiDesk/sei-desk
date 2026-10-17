param([Parameter(Mandatory=$true)][string]$EvidenceDirectory)
$ErrorActionPreference='Stop'
$repo=Split-Path (Split-Path $PSScriptRoot)
Set-Location -LiteralPath $repo
if(Test-Path -LiteralPath $EvidenceDirectory){throw 'Use a fresh evidence directory.'}
$out=New-Item -ItemType Directory -Path $EvidenceDirectory
$target=New-Item -ItemType Directory -Force -Path (Join-Path $PSScriptRoot 'target')
$deps=(Resolve-Path experiments/accesskit-iced-spike/target/debug/deps).Path
$iced=Join-Path $deps 'libiced-5752bf559ff9ed40.rlib'
if(!(Test-Path $iced)){throw 'Build the unchanged pinned AccessKit spike with --locked --offline first; do not choose an arbitrary iced variant.'}
$native=Join-Path $env:USERPROFILE '.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows_x86_64_msvc-0.52.6/lib'
$source=Join-Path $PSScriptRoot 'src/main.rs'
$log=Join-Path $out.FullName 'build.txt'
function Run-Gate([string]$Program,[string[]]$Arguments){
    "$Program $($Arguments -join ' ')"|Add-Content $log
    & $Program @Arguments 2>&1|ForEach-Object{$_.ToString()|Add-Content $log}
    $code=$LASTEXITCODE
    "exit=$code"|Add-Content $log
    if($code -ne 0){throw "$Program failed: $code"}
}
Run-Gate rustfmt @('--edition','2024','--check',$source,(Join-Path $PSScriptRoot 'src/observe.rs'))
$common=@('--edition=2024',$source,'--extern',"iced=$iced",'-L',"dependency=$deps",'-L',"native=$native",'-D','warnings')
Run-Gate rustc ($common+@('--test','-o',(Join-Path $target.FullName 'observer-tests.exe')))
Run-Gate (Join-Path $target.FullName 'observer-tests.exe') @()
Run-Gate clippy-driver ($common+@('--test','-o',(Join-Path $target.FullName 'observer-clippy.exe'),'-D','clippy::all'))
Run-Gate rustc ($common+@('-O','-o',(Join-Path $target.FullName 'iced-ime-observer.exe')))
[ordered]@{UTC=[DateTime]::UtcNow.ToString('o');OS=[Environment]::OSVersion.VersionString;Rust=(& rustc -V|Out-String).Trim();BinarySHA256=(Get-FileHash (Join-Path $target.FullName 'iced-ime-observer.exe')).Hash;IcedRlibSHA256=(Get-FileHash $iced).Hash;LockSHA256=(Get-FileHash experiments/accesskit-iced-spike/Cargo.lock).Hash;NoManifestOrLockChange=$true}|ConvertTo-Json|Set-Content (Join-Path $out.FullName 'identity.json') -Encoding utf8
