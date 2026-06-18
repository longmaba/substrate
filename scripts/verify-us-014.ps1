Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

cargo fmt --check
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

cargo test
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$benchOutput = cargo run --quiet -- bench fixtures\storage-agent-churn
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

$requiredFields = @(
    "substrate_stored_bytes: 10094",
    "substrate_delta_stored_bytes: 6566",
    "delta_dedup_ratio: 17.9638",
    "delta_encoding: sorted-unique-chunk-prefix-suffix-experiment"
)

foreach ($field in $requiredFields) {
    if (($benchOutput -join "`n") -notlike "*$field*") {
        throw "US-014 benchmark output missing expected field: $field"
    }
}

$benchOutput
