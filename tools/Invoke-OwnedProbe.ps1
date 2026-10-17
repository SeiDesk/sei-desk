# Test tooling only: own one exact Process through observation and cleanup.
function Invoke-OwnedProbe {
    param(
        [Parameter(Mandatory=$true)][string]$FilePath,
        [string[]]$ArgumentList = @(),
        [Parameter(Mandatory=$true)][string]$StandardOutput,
        [Parameter(Mandatory=$true)][string]$StandardError,
        [Parameter(Mandatory=$true)][scriptblock]$Observe
    )
    $ownedProcess = $null
    try {
        $start = @{
            FilePath=$FilePath; WindowStyle='Hidden'; PassThru=$true
            RedirectStandardOutput=$StandardOutput
            RedirectStandardError=$StandardError
            ErrorAction='Stop'
        }
        if ($ArgumentList.Count -gt 0) { $start.ArgumentList = $ArgumentList }
        $ownedProcess = Start-Process @start
        $null = $ownedProcess.Handle
        & $Observe $ownedProcess
    } finally {
        if ($null -ne $ownedProcess) {
            try {
                # Dispose closes a handle; it does not stop the owned process.
                # Never find/kill by a name or use a recursive process-tree kill.
                if (!$ownedProcess.HasExited) {
                    try { $ownedProcess.Kill() }
                    catch {
                        # Natural exit may race the check. Suppress only when
                        # the exact owned process is now confirmed terminated.
                        if (!$ownedProcess.HasExited) { throw }
                    }
                }
                $ownedProcess.WaitForExit()
            } finally { $ownedProcess.Dispose() }
        }
    }
}
