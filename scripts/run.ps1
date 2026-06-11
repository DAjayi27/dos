
$projectRoot = Split-Path -Parent $PSScriptRoot
$imagePath = Join-Path $projectRoot "target/arm_64_os/debug/bootimage-DOS.bin"


cargo run
