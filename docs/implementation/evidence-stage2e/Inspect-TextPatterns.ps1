param([Parameter(Mandatory=$true)][string]$OutputDirectory)
$ErrorActionPreference='Stop'
if(Test-Path -LiteralPath $OutputDirectory){throw 'Use a fresh output directory.'}
$out=New-Item -ItemType Directory -Path $OutputDirectory
$repo=(Get-Location).Path
. (Join-Path $repo 'tools/Invoke-OwnedProbe.ps1')
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
$binary=(Resolve-Path experiments/accesskit-iced-spike/target/release/accesskit-iced-spike.exe).Path
Invoke-OwnedProbe -FilePath $binary -StandardOutput (Join-Path $out.FullName 'stdout.txt') -StandardError (Join-Path $out.FullName 'stderr.txt') -Observe {
    param($process)
    $condition=[Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::ProcessIdProperty,$process.Id)
    $deadline=[Diagnostics.Stopwatch]::StartNew()
    $root=$null; $input=$null
    while($deadline.Elapsed.TotalSeconds -lt 10 -and $null -eq $input){
        $windows=[Windows.Automation.AutomationElement]::RootElement.FindAll([Windows.Automation.TreeScope]::Children,$condition)
        foreach($window in $windows){
            if($window.Current.Name -eq 'AccessKit iced spike'){
                $candidate=$window.FindFirst([Windows.Automation.TreeScope]::Descendants,[Windows.Automation.PropertyCondition]::new([Windows.Automation.AutomationElement]::AutomationIdProperty,'spike-3'))
                if($null -ne $candidate){$root=$window;$input=$candidate;break}
            }
        }
        if($null -eq $input){Start-Sleep -Milliseconds 50}
    }
    if($null -eq $input){throw 'Own input unavailable; cannot infer absent pattern from missing window.'}
    $text=$null; $value=$null
    $textAvailable=$input.TryGetCurrentPattern([Windows.Automation.TextPattern]::Pattern,[ref]$text)
    $valueAvailable=$input.TryGetCurrentPattern([Windows.Automation.ValuePattern]::Pattern,[ref]$value)
    $label=$input.GetCurrentPropertyValue([Windows.Automation.AutomationElement]::LabeledByProperty)
    $labelAvailable=($null -ne $label -and $label -ne [Windows.Automation.AutomationElement]::NotSupported)
    $result=[ordered]@{
        DateUTC=[DateTime]::UtcNow.ToString('o');OS=[Environment]::OSVersion.VersionString
        BinarySHA256=(Get-FileHash $binary -Algorithm SHA256).Hash
        Pid=$process.Id;RootHWND=$root.Current.NativeWindowHandle
        InputName=$input.Current.Name;InputControlType=$input.Current.ControlType.ProgrammaticName
        ValuePatternAvailable=$valueAvailable;TextPatternAvailable=$textAvailable
        LabeledByAvailable=$labelAvailable;AccessibleValue=if($valueAvailable){$value.Current.Value}else{$null}
        TextRangeGate=if($textAvailable){'Requires actual range tests; availability alone is insufficient'}else{'BLOCKED: no TextPattern on actual input'}
        LabelRelationshipGate=if($labelAvailable){'Requires target verification'}else{'FAIL: no LabeledBy relationship'}
        PhysicalKeyboard='Not run';Narrator='Not run'
    }
    $result|ConvertTo-Json -Depth 5|Set-Content (Join-Path $out.FullName 'observations.json') -Encoding utf8
    $close=$root.GetCurrentPattern([Windows.Automation.WindowPattern]::Pattern)
    $close.Close()
    if(!$process.WaitForExit(10000)){throw 'Own window did not close normally.'}
    [pscustomobject]@{ExitCode=$process.ExitCode;ChildRemaining=(!$process.HasExited)}|ConvertTo-Json|Set-Content (Join-Path $out.FullName 'cleanup.json') -Encoding utf8
    if($process.ExitCode -ne 0){throw 'Spike exited unsuccessfully.'}
}
