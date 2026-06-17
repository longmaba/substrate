# Storage Agent Churn Fixture

This deterministic fixture contains 25 synthetic revisions under `rev-00` through `rev-24`.
Each revision contains a small generated-looking Rust source tree. Most lines are stable; each revision changes a small handler value and one schema field tag.
It is used by `substrate bench fixtures/storage-agent-churn` to compare whole-file snapshot bytes with fixed-size chunk deduplication.
