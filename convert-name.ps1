$a = Get-Content './pkg/package.json' -raw | ConvertFrom-Json
$a.name = "@ryanpell/scanner-listener"
$a | ConvertTo-Json -depth 32| set-content './pkg/package.json'