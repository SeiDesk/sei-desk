# Developer client, not application GUI code. Run with Windows PowerShell 5.1.
[CmdletBinding()]
param(
    [string]$BinaryPath = 'experiments/accesskit-iced-spike/target/release/accesskit-iced-spike.exe',
    [string]$OutputDirectory = 'docs/implementation/evidence-stage2c/run-01',
    [ValidateRange(1,120)][int]$IdleSeconds = 30
)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class SpikeClient {
  [StructLayout(LayoutKind.Sequential)] public struct RECT {public int L,T,R,B;}
  [StructLayout(LayoutKind.Sequential)] public struct POINT {public int X,Y;}
  [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h,out uint p);
  [DllImport("user32.dll")] static extern bool GetClientRect(IntPtr h,out RECT r);
  [DllImport("user32.dll")] static extern bool ClientToScreen(IntPtr h,ref POINT p);
  [DllImport("user32.dll")] static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] static extern bool PostMessage(IntPtr h,uint m,IntPtr w,IntPtr l);
  public static int[] Bounds(IntPtr h,int pid) {
    uint p;GetWindowThreadProcessId(h,out p);if(p!=pid)throw new Exception("wrong PID");
    RECT r;POINT o=new POINT();if(!GetClientRect(h,out r)||!ClientToScreen(h,ref o))throw new Exception("no client bounds");
    return new int[]{o.X,o.Y,r.R-r.L,r.B-r.T};
  }
  public static bool OwnForeground(IntPtr h) {return GetForegroundWindow()==h;}
  public static void Close(IntPtr h,int pid) {Bounds(h,pid);if(!PostMessage(h,0x10,IntPtr.Zero,IntPtr.Zero))throw new Exception("close failed");}
}
'@
$output = [IO.Path]::GetFullPath((Join-Path (Get-Location) $OutputDirectory))
if ((Test-Path $output) -and @(Get-ChildItem -Force $output).Count -gt 0) { throw 'Output must be new/empty' }
New-Item -ItemType Directory -Force -Path $output | Out-Null
$binary = (Resolve-Path $BinaryPath).Path
$result = [ordered]@{ UTC=[DateTime]::UtcNow.ToString('o'); OS=[Environment]::OSVersion.VersionString; BinarySHA256=(Get-FileHash $binary).Hash; BuildMode='release'; Narrator='Not run'; PhysicalKeyboard='Not run'; TextSelection='Not advertised'; LiveAnnouncement='Not advertised'; Checks=@() }
$child=$null; $root=$null; $windowHandle=[IntPtr]::Zero; $stdoutTask=$null; $stderrTask=$null; $failed=$false
function Check([bool]$Condition,[string]$Name) {
    $result.Checks += [pscustomobject]@{ Name=$Name; Passed=$Condition }
    if (!$Condition) { throw "Check failed: $Name" }
}
function Wait-Condition([scriptblock]$Condition) {
    $watch=[Diagnostics.Stopwatch]::StartNew()
    do { if (& $Condition) { return $true }; Start-Sleep -Milliseconds 50 } while($watch.Elapsed.TotalSeconds -lt 3)
    return $false
}
function Exception-Details($Exception) {
    $chain=@();$current=$Exception
    while($null -ne $current){$chain += [pscustomobject]@{Type=$current.GetType().FullName;HResult=$current.HResult;Message=$current.Message};$current=$current.InnerException}
    return $chain
}
function Find-Control([string]$Name,[System.Windows.Automation.ControlType]$Type) {
    $conditions = [System.Windows.Automation.Condition[]]@(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::NameProperty,$Name),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty,$Type))
    $condition=[System.Windows.Automation.AndCondition]::new($conditions)
    $matches=$root.FindAll([System.Windows.Automation.TreeScope]::Descendants,$condition)
    foreach($match in $matches){if($match.Current.AutomationId.StartsWith('spike-')){return $match}}
    return $null
}
function Save-Tree([string]$Name,[System.Windows.Automation.TreeWalker]$Walker) {
    $queue=[Collections.Queue]::new(); $queue.Enqueue(@($root,0)); $nodes=@()
    while($queue.Count -gt 0 -and $nodes.Count -lt 50) {
        $item=$queue.Dequeue();$node=$item[0];$depth=$item[1];$current=$node.Current
        if ($current.ProcessId -ne $child.Id) { throw 'Unexpected tree process' }
        $rect=$current.BoundingRectangle
        $nodes += [pscustomobject]@{ Depth=$depth; Name=$current.Name; AutomationId=$current.AutomationId; Type=$current.ControlType.ProgrammaticName; PID=$current.ProcessId; HWND=$current.NativeWindowHandle; Enabled=$current.IsEnabled; Focusable=$current.IsKeyboardFocusable; Focused=$current.HasKeyboardFocus; Bounds=@($rect.X,$rect.Y,$rect.Width,$rect.Height); Patterns=@($node.GetSupportedPatterns() | ForEach-Object {$_.ProgrammaticName}); RuntimeId=@($node.GetRuntimeId()) }
        if($depth -ge 6){continue};$descendant=$Walker.GetFirstChild($node)
        while($null -ne $descendant -and $queue.Count -lt 50){$queue.Enqueue(@($descendant,($depth+1)));$descendant=$Walker.GetNextSibling($descendant)}
    }
    ConvertTo-Json -InputObject $nodes -Depth 5 | Set-Content (Join-Path $output "$Name.json") -Encoding UTF8
}
function Save-Image([string]$Name) {
    if(![SpikeClient]::OwnForeground($windowHandle)){return 'Blocked: not foreground'}
    $bounds=[SpikeClient]::Bounds($windowHandle,$child.Id)
    $bitmap=[Drawing.Bitmap]::new($bounds[2],$bounds[3]);$graphics=[Drawing.Graphics]::FromImage($bitmap)
    try{$graphics.CopyFromScreen($bounds[0],$bounds[1],0,0,$bitmap.Size);$bitmap.Save((Join-Path $output "$Name.png"),[Drawing.Imaging.ImageFormat]::Png)}finally{$graphics.Dispose();$bitmap.Dispose()}
    return 'Captured own client'
}
try {
    $info=[Diagnostics.ProcessStartInfo]::new($binary);$info.UseShellExecute=$false;$info.CreateNoWindow=$true;$info.RedirectStandardOutput=$true;$info.RedirectStandardError=$true
    $child=[Diagnostics.Process]::Start($info);$stdoutTask=$child.StandardOutput.ReadToEndAsync();$stderrTask=$child.StandardError.ReadToEndAsync()
    $pidCondition=[System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ProcessIdProperty,$child.Id)
    $found=Wait-Condition {
        if($child.HasExited){throw 'Application exited before discovery'}
        $candidates=[System.Windows.Automation.AutomationElement]::RootElement.FindAll([System.Windows.Automation.TreeScope]::Children,$pidCondition)
        foreach($candidate in $candidates){if($candidate.Current.Name -eq 'AccessKit iced spike' -and $candidate.Current.ControlType -eq [System.Windows.Automation.ControlType]::Window){$script:root=$candidate;return $true}}
        return $false
    }
    Check $found 'Correct PID/title/Window root discovered'
    $windowHandle=[IntPtr]$root.Current.NativeWindowHandle
    $result.PID=$child.Id;$result.HWND=$windowHandle.ToInt64()
    [SpikeClient]::Bounds($windowHandle,$child.Id) | Out-Null
    Save-Tree 'initial-raw' ([System.Windows.Automation.TreeWalker]::RawViewWalker)
    Save-Tree 'initial-control' ([System.Windows.Automation.TreeWalker]::ControlViewWalker)
    $sampleControl=Find-Control 'Sample text' ([System.Windows.Automation.ControlType]::Edit)
    $validate=Find-Control 'Validate' ([System.Windows.Automation.ControlType]::Button)
    $motion=Find-Control 'Prefer reduced motion' ([System.Windows.Automation.ControlType]::CheckBox)
    $close=Find-Control 'Close' ([System.Windows.Automation.ControlType]::Button)
    Check ($null -ne $sampleControl -and $null -ne $validate -and $null -ne $motion -and $null -ne $close) 'Required names and roles discovered'
    $client=[SpikeClient]::Bounds($windowHandle,$child.Id)
    foreach($control in @($sampleControl,$validate,$motion,$close)) {
        $rect=$control.Current.BoundingRectangle
        Check ($control.Current.IsEnabled -and $control.Current.IsKeyboardFocusable) ('Enabled/focusable: '+$control.Current.Name)
        Check ($rect.Width -gt 0 -and $rect.Height -gt 0 -and $rect.X -ge $client[0] -and $rect.Y -ge $client[1] -and $rect.Right -le $client[0]+$client[2]+1 -and $rect.Bottom -le $client[1]+$client[3]+1) ('Bounds inside actual client: '+$control.Current.Name)
    }
    $value=$sampleControl.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    Check (!$value.Current.IsReadOnly -and $value.Current.Value -eq '') 'Editable initially empty ValuePattern'
    # The AccessKit model has LabelledBy, but 0.35.1 does not map UIA LabeledBy.
    # Record this provider limitation separately from the verified accessible name.
    $result.UIALabeledBy=if($null -ne $sampleControl.Current.LabeledBy){$sampleControl.Current.LabeledBy.Current.Name}else{'Not exposed by this provider'}
    $value.SetValue('sample')
    Check (Wait-Condition {$value.Current.Value -eq 'sample'}) 'ValuePattern.SetValue synchronized'
    $result.ValueScreenshot=Save-Image 'value'
    $validate.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
    Start-Sleep -Milliseconds 200
    Check ($null -eq (Find-Control 'Dismiss error' ([System.Windows.Automation.ControlType]::Button))) 'Nonempty validation has no error action'
    $value.SetValue('');Check (Wait-Condition {$value.Current.Value -eq ''}) 'Value cleared'
    # UIA may explicitly request focus on the invoked element before Click.
    # Validate the initiating control's focus, rather than assuming Input retains it.
    $validate.SetFocus()
    Check (Wait-Condition {$validate.Current.HasKeyboardFocus}) 'UIA focus request reaches real Validate focus'
    $validate.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
    Check (Wait-Condition {$null -ne (Find-Control 'Dismiss error' ([System.Windows.Automation.ControlType]::Button))}) 'Invoke Validate exposes error action'
    $dismiss=Find-Control 'Dismiss error' ([System.Windows.Automation.ControlType]::Button)
    Check ($null -ne (Find-Control 'Enter some text before validating.' ([System.Windows.Automation.ControlType]::Text))) 'Error text exposed'
    Check ($validate.Current.HasKeyboardFocus) 'Error preserves initiating Validate focus'
    Save-Tree 'error-raw' ([System.Windows.Automation.TreeWalker]::RawViewWalker)
    Save-Tree 'error-control' ([System.Windows.Automation.TreeWalker]::ControlViewWalker)
    $result.ErrorScreenshot=Save-Image 'error'
    $dismissPattern=$dismiss.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)
    $dismissPattern.Invoke()
    Check (Wait-Condition {$null -eq (Find-Control 'Dismiss error' ([System.Windows.Automation.ControlType]::Button))}) 'Dismiss Invoke removes action'
    Check ($null -eq (Find-Control 'Enter some text before validating.' ([System.Windows.Automation.ControlType]::Text))) 'Dismiss removes error text'
    $staleRejected=$false
    try{$dismissPattern.Invoke()}catch{$result.StaleDismissException=Exception-Details $_.Exception;$staleRejected=@($result.StaleDismissException | Where-Object {$_.Type -eq 'System.Windows.Automation.ElementNotAvailableException' -and $_.HResult -eq -2147220991}).Count -gt 0}
    Check $staleRejected 'Retained removed Dismiss returns UIA_E_ELEMENTNOTAVAILABLE'
    $toggle=$motion.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    $before=$toggle.Current.ToggleState;$toggle.Toggle()
    Check (Wait-Condition {$toggle.Current.ToggleState -ne $before}) 'TogglePattern updates checked state'
    $result.ToggleScreenshot=Save-Image 'checked'
    Save-Tree 'checked-control' ([System.Windows.Automation.TreeWalker]::ControlViewWalker)
    $toggle.Toggle();Check (Wait-Condition {$toggle.Current.ToggleState -eq $before}) 'Toggle restores checked state'
    $transform=$root.GetCurrentPattern([System.Windows.Automation.TransformPattern]::Pattern)
    $original=$root.Current.BoundingRectangle;$originalInput=$sampleControl.Current.BoundingRectangle
    $transform.Resize($original.Width+120,$original.Height+80)
    Check (Wait-Condition {$sampleControl.Current.BoundingRectangle.Width -gt $originalInput.Width+100}) 'Resize updates real input layout bounds'
    Save-Tree 'resized-control' ([System.Windows.Automation.TreeWalker]::ControlViewWalker)
    $result.ResizeScreenshot=Save-Image 'resized'
    $transform.Resize($original.Width,$original.Height)
    Check (Wait-Condition {[Math]::Abs($sampleControl.Current.BoundingRectangle.Width-$originalInput.Width) -lt 2}) 'Restore size restores input bounds'
    $transform.Move($original.X+20,$original.Y+20)
    Check (Wait-Condition {[Math]::Abs($sampleControl.Current.BoundingRectangle.X-$originalInput.X-20) -lt 2}) 'Move updates screen-space bounds'
    $transform.Move($original.X,$original.Y)
    Check (Wait-Condition {[Math]::Abs($sampleControl.Current.BoundingRectangle.X-$originalInput.X) -lt 2}) 'Restore position restores bounds'
    Save-Tree 'final-raw' ([System.Windows.Automation.TreeWalker]::RawViewWalker)
    Save-Tree 'final-control' ([System.Windows.Automation.TreeWalker]::ControlViewWalker)
    Start-Sleep -Seconds 3;$child.Refresh();$cpu=$child.TotalProcessorTime.TotalSeconds;$watch=[Diagnostics.Stopwatch]::StartNew()
    Start-Sleep -Seconds $IdleSeconds;$child.Refresh();$watch.Stop()
    $result.IdleSeconds=$watch.Elapsed.TotalSeconds;$result.CPUSeconds=$child.TotalProcessorTime.TotalSeconds-$cpu;$result.SingleCoreCPUPercent=100*$result.CPUSeconds/$result.IdleSeconds
    $result.WorkingSetBytes=$child.WorkingSet64;$result.PrivateCommitBytes=$child.PrivateMemorySize64;$result.PeakWorkingSetBytes=$child.PeakWorkingSet64
    $close.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern).Invoke()
    Check ($child.WaitForExit(10000)) 'Close Invoke exits process'
    $staleRejected=$false;try{$value.SetValue('late')}catch{$result.StaleClosedException=Exception-Details $_.Exception;$staleRejected=@($result.StaleClosedException | Where-Object {$_.Type -eq 'System.Windows.Automation.ElementNotAvailableException' -and $_.HResult -eq -2147220991}).Count -gt 0}
    Check $staleRejected 'Closed-window input returns UIA_E_ELEMENTNOTAVAILABLE'
} catch {$failed=$true;$result.Failure=$_.Exception.Message} finally {
    if($null -ne $child){
        if(!$child.HasExited){try{if($windowHandle -ne [IntPtr]::Zero){[SpikeClient]::Close($windowHandle,$child.Id)}}catch{$result.CleanupException=$_.Exception.Message};if(!$child.WaitForExit(10000)){$failed=$true;$result.ForcedStop=$true;$child.Kill();$child.WaitForExit()}}
        $child.WaitForExit();$result.ExitCode=$child.ExitCode;$result.ChildRemaining=!$child.HasExited
        if($null -ne $stdoutTask){[IO.File]::WriteAllText((Join-Path $output 'stdout.txt'),$stdoutTask.Result)}
        if($null -ne $stderrTask){[IO.File]::WriteAllText((Join-Path $output 'stderr.txt'),$stderrTask.Result)}
        $trace=$stderrTask.Result;$result.PreShowAttachmentConfirmed=$trace -match ('provider_attached_before_show hwnd='+$windowHandle.ToInt64()+'\r?\n');$result.ProviderDetached=$trace -match 'provider_detached'
        if(!$result.PreShowAttachmentConfirmed -or !$result.ProviderDetached -or $result.ExitCode -ne 0){$failed=$true}
        $child.Dispose()
    }
    $result.UIAActionsPassed=!$failed
    $result.AccessibilityGate='Not passed: Narrator/physical keyboard not run; no live or text-selection claim'
    ConvertTo-Json -InputObject $result -Depth 6 | Set-Content (Join-Path $output 'run.json') -Encoding UTF8
    ConvertTo-Json -InputObject $result -Depth 6
}
if($failed){exit 1}
