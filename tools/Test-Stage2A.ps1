# Developer validation only. GUI code remains Rust + iced; this uses Windows'
# built-in UI Automation client and process counters, not a replacement frontend.
[CmdletBinding()]
param(
    [string]$BinaryPath = 'target/release/sei-desk.exe',
    [string]$OutputDirectory = 'docs/implementation/evidence-stage2a',
    [string]$ProbePath = 'target/release/examples/stage2a_probe.exe',
    [ValidateRange(1, 120)][int]$IdleSeconds = 30,
    [switch]$Keyboard,
    [switch]$ErrorFixture
)
$ErrorActionPreference = 'Stop'
if ($Keyboard -and $ErrorFixture) { throw 'Keyboard sequence and seeded error fixture must be separate runs.' }
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type -AssemblyName System.Drawing

function Get-OwnTree {
    param([System.Windows.Automation.AutomationElement]$Root, [System.Windows.Automation.TreeWalker]$Walker)
    $queue = New-Object System.Collections.Queue
    $queue.Enqueue(@($Root, 0))
    $result = @()
    while ($queue.Count -gt 0 -and $result.Count -lt 80) {
        $item = $queue.Dequeue()
        $node = $item[0]
        $depth = [int]$item[1]
        $current = $node.Current
        $result += [pscustomobject]@{
            Depth = $depth; Name = $current.Name; Type = $current.ControlType.ProgrammaticName
            Class = $current.ClassName; AutomationId = $current.AutomationId
            KeyboardFocusable = $current.IsKeyboardFocusable; HasKeyboardFocus = $current.HasKeyboardFocus
            Enabled = $current.IsEnabled; Patterns = @($node.GetSupportedPatterns() | ForEach-Object { $_.ProgrammaticName })
        }
        if ($depth -ge 6) { continue }
        $child = $Walker.GetFirstChild($node)
        while ($null -ne $child -and $queue.Count -lt 80) {
            $queue.Enqueue(@($child, $depth + 1))
            $child = $Walker.GetNextSibling($child)
        }
    }
    return $result
}

function Test-OwnFocus {
    param([int]$TargetId)
    $focused = [System.Windows.Automation.AutomationElement]::FocusedElement
    return $null -ne $focused -and $focused.Current.ProcessId -eq $TargetId
}

function Save-OwnWindow {
    param([System.Windows.Automation.AutomationElement]$Root, [int]$TargetId, [string]$Path)
    # Never capture another foreground app or a whole desktop screenshot.
    if (!(Test-OwnFocus $TargetId)) { return 'Not captured: target no longer owns keyboard focus' }
    if (!(Test-Path -LiteralPath $ProbePath)) { return 'Not captured: build the native test example first' }
    $boundsText = & $ProbePath $TargetId $Root.Current.NativeWindowHandle bounds 2>$null
    if ($LASTEXITCODE -ne 0) { return 'Not captured: native foreground/client-bounds guard refused' }
    $bounds = $boundsText | ConvertFrom-Json
    if ($bounds.Width -le 0 -or $bounds.Height -le 0) { return 'Not captured: invalid client bounds' }
    $bitmap = $null
    $graphics = $null
    try {
        $bitmap = New-Object System.Drawing.Bitmap ([int]$bounds.Width), ([int]$bounds.Height)
        $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
        $graphics.CopyFromScreen([int]$bounds.X, [int]$bounds.Y, 0, 0, $bitmap.Size)
        $bitmap.Save($Path, [System.Drawing.Imaging.ImageFormat]::Png)
        return 'Captured own foreground window'
    } catch { return "Not captured: $($_.Exception.Message)" }
    finally { if ($null -ne $graphics) { $graphics.Dispose() }; if ($null -ne $bitmap) { $bitmap.Dispose() } }
}

$binary = (Resolve-Path -LiteralPath $BinaryPath).Path
$output = [IO.Path]::GetFullPath($OutputDirectory)
if ((Test-Path -LiteralPath $output) -and @(Get-ChildItem -LiteralPath $output -Force).Count -gt 0) {
    throw 'Use an empty/new output directory; previous evidence is never overwritten.'
}
[IO.Directory]::CreateDirectory($output) | Out-Null
$result = [ordered]@{
    UTC = [DateTime]::UtcNow.ToString('o'); OS = [Environment]::OSVersion.VersionString
    Binary = [IO.Path]::GetFileName($binary); BinarySHA256 = (Get-FileHash $binary -Algorithm SHA256).Hash
    BuildMode = 'release (verify supplied binary path)'; Renderer = 'tiny-skia'
    RequestedIdleSeconds = $IdleSeconds; KeyboardRequested = [bool]$Keyboard
    ErrorFixture = [bool]$ErrorFixture
    UIAQuery = 'Not run'; KeyboardDelivery = 'Not run'; Close = 'Not run'
    # These scripts cannot certify a screen reader from window/child enumeration.
    ScreenReaderSpeech = 'Not run'; Conformance = 'Not established'
}
$child = $null
$failed = $false
try {
    $child = New-Object System.Diagnostics.Process
    $child.StartInfo.FileName = $binary
    $child.StartInfo.Arguments = '--validation --diagnostics'
    if ($ErrorFixture) { $child.StartInfo.Arguments += ' --error-fixture' }
    $child.StartInfo.UseShellExecute = $false
    $child.StartInfo.CreateNoWindow = $true
    $child.StartInfo.RedirectStandardOutput = $true
    $child.StartInfo.RedirectStandardError = $true
    if (!$child.Start()) { throw 'Could not start the validation app' }
    # Drain both pipes asynchronously; retain the Process started by this object
    # so ExitCode remains available, unlike the earlier Start-Process fixture.
    $stdoutTask = $child.StandardOutput.ReadToEndAsync()
    $stderrTask = $child.StandardError.ReadToEndAsync()
    $ready = [Diagnostics.Stopwatch]::StartNew()
    $root = $null
    do {
        if ($child.HasExited) { throw "App exited before a window appeared; code $($child.ExitCode)" }
        $condition = New-Object System.Windows.Automation.PropertyCondition ([System.Windows.Automation.AutomationElement]::ProcessIdProperty), $child.Id
        $windows = [System.Windows.Automation.AutomationElement]::RootElement.FindAll([System.Windows.Automation.TreeScope]::Children, $condition)
        foreach ($candidate in $windows) {
            if ($candidate.Current.ControlType -eq [System.Windows.Automation.ControlType]::Window -and $candidate.Current.Name -match '^Sei-Desk . Stage 2A validation$') {
                $root = $candidate
                break
            }
        }
        if ($null -ne $root) { break }
        Start-Sleep -Milliseconds 100
    } while ($ready.Elapsed.TotalSeconds -lt 15)
    if ($null -eq $root) { throw 'No titled validation window for the exact child PID within 15 seconds' }
    $windowHandle = [IntPtr]$root.Current.NativeWindowHandle
    $result.ValidationHandleSource = 'Exact child PID + UIA Window type + title; checked against iced diagnostic after exit'
    Start-Sleep -Milliseconds 500
    if ($root.Current.ProcessId -ne $child.Id -or $root.Current.Name -notmatch '^Sei-Desk . Stage 2A validation$') {
        throw "Incorrect UIA root: '$($root.Current.Name)' / $($root.Current.ClassName)"
    }
    $raw = @(Get-OwnTree $root ([System.Windows.Automation.TreeWalker]::RawViewWalker))
    $control = @(Get-OwnTree $root ([System.Windows.Automation.TreeWalker]::ControlViewWalker))
    ConvertTo-Json -InputObject $raw -Depth 6 | Set-Content (Join-Path $output 'uia-raw.json') -Encoding UTF8
    ConvertTo-Json -InputObject $control -Depth 6 | Set-Content (Join-Path $output 'uia-control.json') -Encoding UTF8
    $result.UIAQuery = 'Executed'
    $result.UIAClientEditCount = @($control | Where-Object { $_.Type -eq 'ControlType.Edit' }).Count
    $result.UIAClientCheckboxCount = @($control | Where-Object { $_.Type -eq 'ControlType.CheckBox' }).Count
    $result.UIAValidateButtonCount = @($control | Where-Object { $_.Type -eq 'ControlType.Button' -and $_.Name -eq 'Validate' }).Count

    if ($Keyboard) {
        if (!(Test-Path -LiteralPath $ProbePath) -or !(Test-OwnFocus $child.Id)) {
            $result.KeyboardDelivery = 'Blocked: target did not acquire keyboard focus'
        } else {
            $result.KeyboardDelivery = 'Native sequence dispatched; inspect app diagnostics for actual delivery'
            $steps = @(
                @{ Key = 'tab'; Name = 'validate_focus' },
                @{ Key = 'enter'; Name = 'empty_error' },
                @{ Key = 'escape'; Name = 'dismiss_error' },
                @{ Key = 'tab'; Name = 'motion_focus' },
                @{ Key = 'space'; Name = 'toggle_motion' },
                @{ Key = 'back'; Name = 'back_validate' },
                @{ Key = 'back'; Name = 'back_input' },
                @{ Key = 'sample'; Name = 'edit' },
                @{ Key = 'caret'; Name = 'edit_at_caret' },
                @{ Key = 'tab'; Name = 'validate_again' },
                @{ Key = 'enter'; Name = 'valid' }
            )
            foreach ($step in $steps) {
                # UIA PID plus native exact-HWND/title guards before fixed input only.
                if (!(Test-OwnFocus $child.Id)) { $result.KeyboardDelivery = 'Interrupted: focus changed; remaining keys not sent'; break }
                $ErrorActionPreference = 'Continue'
                try { & $ProbePath $child.Id $windowHandle.ToInt64() $step.Key 2>&1 | Add-Content (Join-Path $output 'keyboard-probe.txt') }
                finally { $ErrorActionPreference = 'Stop' }
                $probeExit = $LASTEXITCODE
                if ($probeExit -ne 0) { $result.KeyboardDelivery = 'Blocked: native foreground/input probe refused; remaining keys not sent'; break }
                Start-Sleep -Milliseconds 200
                if ($step.Name -eq 'empty_error') {
                    $errorTree = @(Get-OwnTree $root ([System.Windows.Automation.TreeWalker]::ControlViewWalker))
                    ConvertTo-Json -InputObject $errorTree -Depth 6 | Set-Content (Join-Path $output 'uia-error.json') -Encoding UTF8
                    $result.ErrorScreenshot = Save-OwnWindow $root $child.Id (Join-Path $output 'error-focus.png')
                }
            }
        }
    }
    $result.FinalScreenshot = Save-OwnWindow $root $child.Id (Join-Path $output 'window.png')
    Start-Sleep -Seconds 3
    $child.Refresh()
    $cpuStart = $child.TotalProcessorTime.TotalSeconds
    $result.WorkingSetStartBytes = $child.WorkingSet64
    $result.PrivateCommitStartBytes = $child.PrivateMemorySize64
    $result.TargetOwnsKeyboardFocusAtSampleStart = Test-OwnFocus $child.Id
    $watch = [Diagnostics.Stopwatch]::StartNew()
    Start-Sleep -Seconds $IdleSeconds
    $child.Refresh()
    $watch.Stop()
    $result.ObservedIdleSeconds = $watch.Elapsed.TotalSeconds
    $result.CPUSeconds = $child.TotalProcessorTime.TotalSeconds - $cpuStart
    $result.SingleCoreCPUPercent = 100 * $result.CPUSeconds / $result.ObservedIdleSeconds
    $result.WorkingSetEndBytes = $child.WorkingSet64
    $result.PrivateCommitEndBytes = $child.PrivateMemorySize64
    $result.PeakWorkingSetBytes = $child.PeakWorkingSet64
    $result.TargetOwnsKeyboardFocusAtSampleEnd = Test-OwnFocus $child.Id
} catch {
    $failed = $true
    $result.Failure = $_.Exception.Message
} finally {
    if ($null -ne $child) {
        if (!$child.HasExited) {
            $closeSent = $false
            try {
                if ($null -ne $root -and $root.Current.ProcessId -eq $child.Id) {
                    $windowPattern = $root.GetCurrentPattern([System.Windows.Automation.WindowPattern]::Pattern)
                    $windowPattern.Close()
                    $closeSent = $true
                }
            } catch { $result.ClosePatternFailure = $_.Exception.Message }
            if (!$closeSent) {
                # Fallback still targets only this owned process; it is not a UIA pass.
                try { $closeSent = $child.CloseMainWindow() } catch { $result.CloseMessageFailure = $_.Exception.Message }
            }
            if ($closeSent -and $child.WaitForExit(10000)) { $result.Close = 'WM_CLOSE: process exited' }
            else {
                $result.Close = 'Failed graceful exit: exact owned child forcibly stopped'
                $failed = $true
                if (!$child.HasExited) { $child.Kill(); $child.WaitForExit() }
            }
        }
        $child.WaitForExit()
        $child.Refresh()
        $result.ExitCode = $child.ExitCode
        $result.ChildRemaining = !$child.HasExited
        if ($null -ne $stdoutTask) { $stdoutTask.Result | Set-Content (Join-Path $output 'app-stdout.txt') -Encoding UTF8 }
        if ($null -ne $stderrTask) { $stderrTask.Result | Set-Content (Join-Path $output 'app-stderr.txt') -Encoding UTF8 }
        $child.Dispose()
    }
    $trace = Get-Content (Join-Path $output 'app-stderr.txt') -Raw -ErrorAction SilentlyContinue
    if ($null -ne $windowHandle) {
        $result.IcedHandleConfirmed = $trace -match ("(?m)^validation_hwnd=" + $windowHandle.ToInt64() + '\r?$')
        if (!$result.IcedHandleConfirmed) { $failed = $true; $result.HandleFailure = 'Selected root did not match the iced-owned diagnostic HWND' }
    }
    if ($result.KeyboardDelivery -like 'Native sequence dispatched*') {
        $result.KeyboardStateTraceObserved = $trace -match 'focus=Validate feedback=Empty' -and $trace -match 'focus=Motion.*local_reduce=true' -and $trace -match 'focus=Validate feedback=Valid'
        if (!$result.KeyboardStateTraceObserved) { $result.KeyboardDelivery = 'Failed: expected app state transitions were not observed'; $failed = $true }
    }
    ConvertTo-Json -InputObject $result -Depth 6 | Set-Content (Join-Path $output 'run.json') -Encoding UTF8
    ConvertTo-Json -InputObject $result -Depth 6
}
if ($failed) { exit 1 }
