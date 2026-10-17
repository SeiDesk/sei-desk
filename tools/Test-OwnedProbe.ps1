param([Parameter(Mandatory=$true)][string]$OutputDirectory)
$ErrorActionPreference='Stop'
if (Test-Path -LiteralPath $OutputDirectory) { throw 'Use a fresh evidence directory.' }
$out=New-Item -ItemType Directory -Path $OutputDirectory
. (Join-Path $PSScriptRoot 'Invoke-OwnedProbe.ps1')
$shell=Join-Path $env:SystemRoot 'System32/WindowsPowerShell/v1.0/powershell.exe'
$results=@()
foreach ($case in @('ObserverThrow','NormalRelease','AfterExitThrow')) {
    $tag=[Guid]::NewGuid().ToString('N')
    $readyName="Local\SeiDeskProbeReady-$tag"
    $releaseName="Local\SeiDeskProbeRelease-$tag"
    $ready=[Threading.EventWaitHandle]::new($false,[Threading.EventResetMode]::ManualReset,$readyName)
    $release=[Threading.EventWaitHandle]::new($false,[Threading.EventResetMode]::ManualReset,$releaseName)
    # Child blocks on a parent-owned named event, not a timing sleep. Event
    # objects disappear when the parent/owned child close their handles.
    $code="`$r=[Threading.EventWaitHandle]::OpenExisting('$readyName'); `$g=[Threading.EventWaitHandle]::OpenExisting('$releaseName'); try { `$null=`$r.Set(); `$null=`$g.WaitOne() } finally { `$r.Dispose(); `$g.Dispose() }"
    $encoded=[Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($code))
    $context=[pscustomobject]@{Pid=0;ObservedFailure=$false;ExitCode=$null}
    try {
        try {
            Invoke-OwnedProbe -FilePath $shell -ArgumentList @('-NoProfile','-NonInteractive','-EncodedCommand',$encoded) -StandardOutput (Join-Path $out.FullName "$case-stdout.txt") -StandardError (Join-Path $out.FullName "$case-stderr.txt") -Observe {
                param($process)
                $context.Pid=$process.Id
                if (!$ready.WaitOne(10000)) { throw 'Fixture did not reach its event gate.' }
                if ($process.HasExited) { throw 'Fixture must be alive at observer entry.' }
                if ($case -eq 'ObserverThrow') { throw 'Injected metric observer failure' }
                $null=$release.Set()
                if (!$process.WaitForExit(10000)) { throw 'Fixture did not exit after release.' }
                $context.ExitCode=$process.ExitCode
                if ($case -eq 'AfterExitThrow') { throw 'Injected metric observer failure' }
            }
        } catch {
            if ($_.Exception.Message -ne 'Injected metric observer failure') { throw }
            $context.ObservedFailure=$true
        }
        if ($context.ObservedFailure -ne ($case -ne 'NormalRelease')) { throw 'Wrong exception outcome.' }
        if ($case -ne 'ObserverThrow' -and $context.ExitCode -ne 0) { throw 'Fixture failed to exit normally.' }
        if ($context.Pid -eq 0) { throw 'Owned fixture never started.' }
        if ($null -ne (Get-Process -Id $context.Pid -ErrorAction SilentlyContinue)) { throw 'Owned child remains after observation.' }
        $results += [pscustomobject]@{Case=$case;Status='Passed';OwnedPid=$context.Pid;ObserverFailurePropagated=$context.ObservedFailure;NormalExitCode=$context.ExitCode;ChildRemaining=$false}
    } finally {
        $null=$release.Set()
        $ready.Dispose(); $release.Dispose()
    }
}
[ordered]@{DateUTC=[DateTime]::UtcNow.ToString('o');OS=[Environment]::OSVersion.VersionString;Tests=$results} | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $out.FullName 'results.json') -Encoding utf8
$results | Format-Table
