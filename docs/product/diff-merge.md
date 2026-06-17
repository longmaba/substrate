# Diff And Merge

## Contract

Substrate should prefer structural and semantic units over raw line ranges when
the language and tooling make that safe. Text diffs remain available as a
projection for human review and compatibility.

## Structural Diff

The intended default diff model is structural:

- Compare typed AST, IR, or semantic graph nodes when available.
- Normalize formatting churn out of review by construction.
- Keep text diff available for unsupported languages and compatibility output.
- Treat semantic equivalence claims conservatively; normalized structure is not
  the same as proven behavioral equivalence.

## Versioning Units

Substrate should eventually version product-relevant code entities, such as:

- Functions.
- Types.
- Public contracts.
- Dependency edges.
- Symbol-level metadata.

Files remain important for projection and compatibility, but they should not be
the only unit of history, fetch, diff, or merge.

## Merge Model

The target merge model is:

1. Identify the semantic entity or entities touched by competing changes.
2. Collect the intents and constraints behind each candidate.
3. Reconcile intents against a merged base where possible.
4. Generate or structurally reconcile a new candidate.
5. Admit the result only after the selected verification gates pass.

## Conflict Model

Conflicts should be framed around incompatible intent, contract, or verification
requirements rather than only overlapping lines. Text conflicts are still a
valid fallback when structural tooling cannot produce a reliable result.
