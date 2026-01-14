[int]$choice = Read-Host -Prompt "1. release 2. dev"
$hostdir
if ($choice -eq 1) {
    Write-Output "1"
    $hostdir = "./target/aarch64-unknown-linux-gnu/release/indra"
}
else {
    $hostdir = "./target/aarch64-unknown-linux-gnu/debug/indra"
}

scp -p $hostdir indra@raspberrypie.local:~/transfer/