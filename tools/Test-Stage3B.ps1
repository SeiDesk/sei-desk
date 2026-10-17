param([Parameter(Mandatory=$true)][string]$OutputDirectory)
$ErrorActionPreference='Stop'
Set-Location -LiteralPath (Split-Path $PSScriptRoot)
if(Test-Path -LiteralPath $OutputDirectory) { throw 'Use a fresh evidence directory.' }
$out=New-Item -ItemType Directory -Path $OutputDirectory
# Preserve the prior runner; it checks the whole workspace, builds all examples,
# runs every sei-apps opt-in fixture test, and measures the Stage 3A baseline.
& (Join-Path $PSScriptRoot 'Test-Stage3A.ps1') -OutputDirectory (Join-Path $out.FullName 'suite')
$exe=(Resolve-Path target/release/examples/consumer_probe.exe).Path
. (Join-Path $PSScriptRoot 'Invoke-OwnedProbe.ps1')
Invoke-OwnedProbe -FilePath $exe -StandardOutput (Join-Path $out.FullName 'consumer-stdout.txt') -StandardError (Join-Path $out.FullName 'consumer-stderr.txt') -Observe {
    param($process)
    $clock=[Diagnostics.Stopwatch]::StartNew()
    $working=0L; $private=0L; $cpu=0.0
    while(!$process.HasExited) {
        $process.Refresh()
        $working=[Math]::Max($working,$process.WorkingSet64)
        $private=[Math]::Max($private,$process.PrivateMemorySize64)
        $cpu=$process.TotalProcessorTime.TotalSeconds
        if($clock.Elapsed.TotalSeconds -gt 30) { $process.Kill(); $process.WaitForExit(); throw 'Owned consumer probe timed out' }
        Start-Sleep -Milliseconds 50
    }
    $process.WaitForExit()
    $exitCode=$process.ExitCode
    $principal=[Security.Principal.WindowsPrincipal]::new([Security.Principal.WindowsIdentity]::GetCurrent())
    [ordered]@{
        DateUTC=[DateTime]::UtcNow.ToString('o'); OS=[Environment]::OSVersion.VersionString
        Build='release'; Rust=(& rustc -V | Out-String).Trim()
        AdministratorRoleInCurrentToken=$principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
        ProbeSHA256=(Get-FileHash $exe -Algorithm SHA256).Hash
        DurationSeconds=$clock.Elapsed.TotalSeconds; SamplingMilliseconds=50
        IncludesTwoRefreshesAndTwoSecondIdleHold=$true
        MaxSampledWorkingSetBytes=$working; MaxSampledPrivateCommitBytes=$private
        LastSampledCPUSeconds=$cpu; ExitCode=$exitCode; ChildRemaining=(!$process.HasExited)
    } | ConvertTo-Json | Set-Content (Join-Path $out.FullName 'consumer-resource-baseline.json') -Encoding utf8
    if($exitCode -ne 0) { throw "Consumer probe failed: $exitCode" }
}
