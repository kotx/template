$project_name = Read-Host "Please enter the project name"

$configFiles = Get-ChildItem . * -Recurse
foreach ($file in $configFiles)
{
    (Get-Content $file.PSPath) |
    Foreach-Object { $_ -replace "template", $project_name } |
    Set-Content $file.PSPath
}

Remove-Item -Recurse .git
git init
