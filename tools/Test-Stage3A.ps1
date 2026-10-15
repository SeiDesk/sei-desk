param([Parameter(Mandatory=$true)][string]$OutputDirectory)
$ErrorActionPreference = 'Stop'
$repo = Split-Path $PSScriptRoot
Set-Location -LiteralPath $repo
if (Test-Path -LiteralPath $OutputDirectory) { throw 'Use a new output directory; prior evidence is never overwritten.' }
$out = New-Item -ItemType Directory -Path $OutputDirectory
$log = Join-Path $out.FullName 'quality-gates.txt'
$oldIncremental = $env:CARGO_INCREMENTAL
$oldFixture = $env:SEI_APPS_FIXTURE
try {
    # Process-local only, to avoid unsupported hard links on this workspace disk.
    $env:CARGO_INCREMENTAL = '0'
    $commands = @(
        @('fmt','--all','--check'),
        @('check','--workspace','--all-targets','--locked','--offline'),
        @('test','--workspace','--locked','--offline'),
        @('clippy','--workspace','--all-targets','--locked','--offline','--','-D','warnings'),
        @('build','--workspace','--release','--locked','--offline'),
        @('build','-p','sei-apps','--release','--examples','--locked','--offline')
    )
    foreach ($arguments in $commands) {
        "cargo $($arguments -join ' ')" | Add-Content $log -Encoding utf8
        # Native stderr contains normal Cargo progress, not just failures.
        $ErrorActionPreference = 'Continue'
        & cargo @arguments 2>&1 | ForEach-Object { $line = $_.ToString(); $line | Add-Content $log -Encoding utf8; $line }
        $result = $LASTEXITCODE
        $ErrorActionPreference = 'Stop'
        "exit=$result" | Add-Content $log -Encoding utf8
        if ($result -ne 0) { throw "Cargo failed: $result" }
    }
    $env:SEI_APPS_FIXTURE = (Resolve-Path target/release/examples/launch_fixture.exe).Path
    $ErrorActionPreference = 'Continue'
    & cargo test -p sei-apps --locked --offline -- --include-ignored 2>&1 | ForEach-Object { $line=$_.ToString(); $line | Add-Content (Join-Path $out.FullName 'windows-tests.txt') -Encoding utf8; $line }
    $result = $LASTEXITCODE
    $ErrorActionPreference = 'Stop'
    if ($result -ne 0) { throw "Fixture tests failed: $result" }
    $exe = (Resolve-Path target/release/examples/catalog_probe.exe).Path
    $stdout = Join-Path $out.FullName 'catalog-stdout.txt'
    $stderr = Join-Path $out.FullName 'catalog-stderr.txt'
    $process = Start-Process -FilePath $exe -WindowStyle Hidden -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
    $null = $process.Handle
    $clock = [Diagnostics.Stopwatch]::StartNew()
    $maxWorking = 0L; $maxPrivate = 0L; $cpu = 0.0
    try {
        while (!$process.HasExited) {
            $process.Refresh()
            $maxWorking = [Math]::Max($maxWorking,$process.WorkingSet64)
            $maxPrivate = [Math]::Max($maxPrivate,$process.PrivateMemorySize64)
            $cpu = $process.TotalProcessorTime.TotalSeconds
            if ($clock.Elapsed.TotalSeconds -gt 30) { $process.Kill(); $process.WaitForExit(); throw 'Owned read-only probe timed out' }
            Start-Sleep -Milliseconds 50
        }
        $process.WaitForExit()
        $exitCode = $process.ExitCode
        [ordered]@{
            DateUTC=[DateTime]::UtcNow.ToString('o'); OS=[Environment]::OSVersion.VersionString
            Build='release'; Rust=(& rustc -V | Out-String).Trim()
            ProbeSHA256=(Get-FileHash $exe -Algorithm SHA256).Hash
            FixtureSHA256=(Get-FileHash $env:SEI_APPS_FIXTURE -Algorithm SHA256).Hash
            DurationSeconds=$clock.Elapsed.TotalSeconds; SamplingMilliseconds=50
            IncludesTwoRefreshesAndTwoSecondIdleHold=$true
            MaxSampledWorkingSetBytes=$maxWorking; MaxSampledPrivateCommitBytes=$maxPrivate
            LastSampledCPUSeconds=$cpu; ExitCode=$exitCode; ChildRemaining=(!$process.HasExited)
        } | ConvertTo-Json | Set-Content (Join-Path $out.FullName 'resource-baseline.json') -Encoding utf8
        if ($exitCode -ne 0) { throw "Catalog probe failed: $exitCode" }
    } finally { $process.Dispose() }
    git diff --check
    if ($LASTEXITCODE -ne 0) { throw 'git diff --check failed' }
    'git diff --check exit=0' | Add-Content $log -Encoding utf8
} finally {
    $env:CARGO_INCREMENTAL = $oldIncremental
    $env:SEI_APPS_FIXTURE = $oldFixture
}
