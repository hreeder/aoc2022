$NextDay = (Get-ChildItem src\bin).count + 1
Write-Host "Creating Day $NextDay"

$DayTemplate = (Get-Content -path _tpl.rs) -replace 'day1',"day$NextDay"
Out-File -FilePath src\bin\day$NextDay.rs -InputObject $DayTemplate
New-Item -Name "data\day$NextDay.txt" -ItemType File

$CargoAdd = @"

[[bin]]
name = "day$NextDay"
"@
Add-Content Cargo.toml $CargoAdd