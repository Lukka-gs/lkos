# Native process smoke test. Run only on an interactive Windows desktop.
# Terminates only the LKOS processes started by this script.
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$executable = (Resolve-Path -LiteralPath (Join-Path $repoRoot 'src-tauri/target/release/lkos.exe')).Path
if (@(Get-Process -Name lkos -ErrorAction SilentlyContinue).Count -ne 0) {
    throw 'Close existing LKOS instances before running this test.'
}
$ownedProcesses = [Collections.Generic.List[Diagnostics.Process]]::new()
$settingsPath = Join-Path $env:APPDATA 'io.github.lukka-gs.lkos/settings.json'
function Start-TestInstance {
    $process = Start-Process -FilePath $executable -WorkingDirectory $repoRoot -WindowStyle Hidden -PassThru
    $ownedProcesses.Add($process)
    return $process
}
function Wait-ForWindow([Diagnostics.Process] $process) {
    $deadline = [DateTime]::UtcNow.AddSeconds(20)
    do {
        $process.Refresh()
        if ($process.HasExited) { throw 'Primary LKOS exited before creating its window.' }
        if ($process.MainWindowHandle -ne [IntPtr]::Zero) { return }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    throw 'LKOS did not create a window within 20 seconds.'
}
function Settings-Hash {
    if (-not (Test-Path -LiteralPath $settingsPath)) { throw 'Settings were not created.' }
    $sha = [Security.Cryptography.SHA256]::Create()
    try { return [Convert]::ToBase64String($sha.ComputeHash([IO.File]::ReadAllBytes($settingsPath))) }
    finally { $sha.Dispose() }
}
try {
    $primary = Start-TestInstance
    Wait-ForWindow $primary
    $initialHash = Settings-Hash
    for ($attempt = 0; $attempt -lt 3; $attempt++) {
        $secondary = Start-TestInstance
        if (-not $secondary.WaitForExit(10000)) { throw 'Duplicate LKOS did not exit.' }
        if ($secondary.ExitCode -ne 0) { throw "Duplicate exited with code $($secondary.ExitCode)." }
        $primary.Refresh()
        if ($primary.HasExited) { throw 'Original LKOS exited during duplicate launch.' }
        if ((Settings-Hash) -ne $initialHash) { throw 'Duplicate launch changed persisted settings.' }
    }
    # Simulate an unclean native-process exit; WebView2 child teardown is handled by Windows.
    Stop-Process -InputObject $primary -Force
    $primary.WaitForExit()
    $restarted = Start-TestInstance
    Wait-ForWindow $restarted
    if ((Settings-Hash) -ne $initialHash) { throw 'Restart changed persisted settings.' }
    Write-Output 'PASS: three duplicate launches exited successfully; original survived; restart after forced exit preserved settings.'
} finally {
    foreach ($process in $ownedProcesses) {
        $process.Refresh()
        if (-not $process.HasExited) {
            Stop-Process -InputObject $process -Force
            $process.WaitForExit()
        }
        $process.Dispose()
    }
}
