param(
 [Parameter(Mandatory=$true)][string]$ArchivePath,
 [Parameter(Mandatory=$true)][string]$ManifestPath,
 [ValidatePattern('^[D-Z]$')][string]$BootDrive='D',
 [ValidatePattern('^[D-Z]$')][string]$GamesDrive='E',
 [switch]$VerifyOnly
)
$ErrorActionPreference='Stop'
$manifest=Get-Content -LiteralPath $ManifestPath -Raw | ConvertFrom-Json
if ($manifest.device -ne 'G350') {throw 'Expected G350 manifest'}
$boot=Get-Volume -DriveLetter $BootDrive
$games=Get-Volume -DriveLetter $GamesDrive
$bootPartition=Get-Partition -DriveLetter $BootDrive
$gamePartition=Get-Partition -DriveLetter $GamesDrive
$disk=Get-Disk -Number $bootPartition.DiskNumber
if ($boot.FileSystemLabel -ne 'PAN4ELC' -or $boot.FileSystem -ne 'FAT32' -or $games.FileSystemLabel -ne 'GAMES' -or $bootPartition.DiskNumber -ne $gamePartition.DiskNumber -or $disk.IsBoot -or $disk.IsSystem -or $disk.BusType -ne 'USB') {throw 'Expected connected USB G350 card with PAN4ELC and GAMES partitions'}
if ((Get-FileHash -LiteralPath $ArchivePath -Algorithm SHA256).Hash.ToLowerInvariant() -ne $manifest.update_sha256) {throw 'Update archive hash mismatch'}
$systemPath=$BootDrive+':\SYSTEM';$md5Path=$BootDrive+':\SYSTEM.md5';$slotPath=$GamesDrive+':\Slot'
$kernelPath=$BootDrive+':\KERNEL';$kernelMd5Path=$BootDrive+':\KERNEL.md5'
$hasKernel=$null -ne $manifest.PSObject.Properties['kernel_sha256']
$payload=@{}
if($null -ne $manifest.PSObject.Properties['payload_sha256']) {
 foreach($entry in $manifest.payload_sha256.PSObject.Properties) {
  if($entry.Name -notmatch '^Labels/PORTS/[A-Za-z0-9 ._()-]+\.png$'){throw 'Unexpected update payload path'}
  $payload[$entry.Name]=$entry.Value
 }
}
$payloadTargets=@($payload.Keys | ForEach-Object {[IO.Path]::GetFullPath((Join-Path $slotPath $_))})
function Slot-Metadata { @(Get-ChildItem -LiteralPath $slotPath -Recurse -File | Where-Object {$_.FullName -notin $payloadTargets} | Sort-Object FullName | ForEach-Object {$_.FullName+'|'+$_.Length+'|'+$_.LastWriteTimeUtc.Ticks}) }
$before=Slot-Metadata
Add-Type -Path (Join-Path $PSScriptRoot 'windows-card-io.cs')
if($hasKernel -and -not $VerifyOnly) {
 $currentKernel=[CardDirectRead]::Hash($kernelPath)
 if($currentKernel -ne $manifest.base_kernel_sha256 -and $currentKernel -ne $manifest.kernel_sha256){throw 'Different/custom G350 kernel; refusing to overwrite it'}
}
if (-not $VerifyOnly) {
 Add-Type -AssemblyName System.IO.Compression.FileSystem
 $zip=[System.IO.Compression.ZipFile]::OpenRead((Resolve-Path -LiteralPath $ArchivePath).Path)
 try {
  if ($zip.GetEntry('SYSTEM').Length -ne $manifest.system_size) {throw 'Unexpected SYSTEM size'}
  $pairs=@(@('SYSTEM',$systemPath),@('SYSTEM.md5',$md5Path))
  if($hasKernel) {
   if($zip.GetEntry('KERNEL').Length -ne $manifest.kernel_size){throw 'Unexpected KERNEL size'}
   $pairs+=,@('KERNEL',$kernelPath);$pairs+=,@('KERNEL.md5',$kernelMd5Path)
  }
  foreach($relative in $payload.Keys) {
   $target=Join-Path $slotPath $relative
   [IO.Directory]::CreateDirectory([IO.Path]::GetDirectoryName($target)) | Out-Null
   $entry=$zip.GetEntry('Slot/'+$relative);if($null -eq $entry){throw 'Missing label payload'}
   $src=$entry.Open();$dst=[IO.FileStream]::new($target,[IO.FileMode]::Create,[IO.FileAccess]::Write,[IO.FileShare]::None,1MB,[IO.FileOptions]::WriteThrough)
   try{$src.CopyTo($dst,1MB);$dst.Flush($true)}finally{$dst.Dispose();$src.Dispose()}
  }
  foreach($pair in $pairs) {
   $entry=$zip.GetEntry($pair[0]);if ($null -eq $entry) {throw 'Missing archive member'}
   $src=$entry.Open()
   $dst=[IO.FileStream]::new($pair[1],[IO.FileMode]::Open,[IO.FileAccess]::Write,[IO.FileShare]::None,1MB,[IO.FileOptions]::WriteThrough)
   try {$src.CopyTo($dst,1MB);$dst.SetLength($entry.Length);$dst.Flush($true)} finally {$dst.Dispose();$src.Dispose()}
  }
 } finally {$zip.Dispose()}
}
$hash=[CardDirectRead]::Hash($systemPath)
if ($hash -ne $manifest.system_sha256) {throw 'Uncached SD read failed verification. Leave card connected for repair.'}
if ([IO.File]::ReadAllText($md5Path).Trim() -ne ($manifest.system_md5+'  target/SYSTEM')) {throw 'SYSTEM.md5 mismatch'}
if($hasKernel) {
 if([CardDirectRead]::Hash($kernelPath) -ne $manifest.kernel_sha256){throw 'Uncached KERNEL verification failed'}
 if([IO.File]::ReadAllText($kernelMd5Path).Trim() -ne ($manifest.kernel_md5+'  target/KERNEL')){throw 'KERNEL.md5 mismatch'}
}
foreach($relative in $payload.Keys) {
 if([CardDirectRead]::Hash((Join-Path $slotPath $relative)) -ne $payload[$relative]){throw "Label verification failed: $relative"}
}
$after=Slot-Metadata
if (($before -join "`n") -ne ($after -join "`n")) {throw 'Slot metadata changed; inspect card before use'}
Write-Output "PASS: uncached firmware and artwork verification; other Slot files unchanged. SYSTEM SHA256 $hash"
