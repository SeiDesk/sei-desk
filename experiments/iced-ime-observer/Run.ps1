param([Parameter(Mandatory=$true)][string]$EvidenceDirectory)
$ErrorActionPreference='Stop'
$repo=Split-Path (Split-Path $PSScriptRoot)
Set-Location -LiteralPath $repo
if(Test-Path $EvidenceDirectory){throw 'Use a fresh runtime directory.'}
$out=New-Item -ItemType Directory -Path $EvidenceDirectory
$binary=(Resolve-Path (Join-Path $PSScriptRoot 'target/iced-ime-observer.exe')).Path
$observations=Join-Path $out.FullName 'observations.jsonl'
. (Join-Path $repo 'tools/Invoke-OwnedProbe.ps1')
Invoke-OwnedProbe -FilePath $binary -ArgumentList @($observations) -StandardOutput (Join-Path $out.FullName 'stdout.txt') -StandardError (Join-Path $out.FullName 'stderr.txt') -Observe {
 param($process)
 [ordered]@{UTC=[DateTime]::UtcNow.ToString('o');OS=[Environment]::OSVersion.VersionString;BinarySHA256=(Get-FileHash $binary).Hash;Pid=$process.Id;BoundSeconds=600;InputMethod='Record actual driver separately; no synthetic IME events'}|ConvertTo-Json|Set-Content (Join-Path $out.FullName 'identity.json') -Encoding utf8
 if(!$process.WaitForExit(600000)){throw 'Owned harness reached its ten-minute bound; finally stops only this process.'}
 [pscustomobject]@{ExitCode=$process.ExitCode;ChildRemaining=(!$process.HasExited)}|ConvertTo-Json|Set-Content (Join-Path $out.FullName 'cleanup.json') -Encoding utf8
 if($process.ExitCode -ne 0){throw "Harness failed: $($process.ExitCode)"}
}
