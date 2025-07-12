# ADR‑000 — Packaging & Test Strategy for the Tool‑Set Repository

**Status**: Proposed  
**Date**: 2025‑07‑11  

---

## 1. Design & Technology Decision

The **tool‑set itself** (template its frontend, backend code, CDK constructs and docs) will be treated as a production‑grade software product.
We therefore adopt a multi‑layer test pyramid and a predictable packaging flow:

| Level                        | Scope                                                  | Framework                                     | Trigger             |
| ---------------------------- | ------------------------------------------------------ |-----------------------------------------------| ------------------- |
| **Lint / Format**            | All source files                                       | `npm run lint`, `cargo fmt`, ESLint, Prettier | PR                  |
| **Unit tests**               | Rust crates & Deno/TS libs                             | `cargo nextest`, `npm run test`               | PR                  |
| **Snapshot / Static tests**  | CDK constructs                                         | `aws-cdk/assertions`, `cdk-watchful snapshot` | PR                  |
| **Template smoke tests**     | Generate minimal app → compile & run its own tests     | Custom script                                 | PR                  |
| **End‑to‑end tool‑set test** | Full example app (PWA + Lambda) deployed to LocalStack | Playwright + k6                               | merge to main       |

Packaging outputs:

* **Rust crates** → published to GitHub Packages and Cargo registry on tag.
* **TypeScript packages** (`@toolset/*`) → published to GitHub Packages on tag.
* **Cookiecutter templates** → version‑tagged ZIP artifacts uploaded to the Release page.

Versioning: Semantic Versioning **per package**, with a mono‑tag `vX.Y.Z` for the template collection.

---

## 2. Intended Usage in Repository

1. **Local dev**

   ```bash
   npm run lint        # JS/TS & yaml
   cargo nextest -p all  # Rust
   npm run tes           # Deno libs
   ./scripts/test-cdk.sh # Synth + assertions
   ./scripts/test-template.sh web-basic
   ```
2. **PR checks** (GitHub Actions matrix)

   * Lint & unit tests across OSes (`ubuntu‑latest`, `macos‑latest`).
   * CDK snapshot test (`cdk synth` vs committed manifest).
   * Template smoke: generate *web-basic* and *full-stack* apps, run their unit tests.
3. **Nightly**

   * Run `template-e2e.yml`: spins up LocalStack, deploys generated app via CDK, runs Playwright E2E + k6 load.
   * Upload coverage reports to Codecov.
4. **Release**

   * Merge to `main` with `release:` prefix → version bump via `changeset`, tag, publish crates & npm, create GitHub Release with template zips.
     Requires the end-to-end tests to succeed.

---

## 3. Tool‑Set Support

| Component                               | Details                                                                                                 |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **`/scripts/test-template.sh`**         | Cookiecutter generate, `deno task build:ssg`, `cargo lambda build`, run Playwright unit tests headless. |
| **`/.github/workflows/toolset-ci.yml`** | Matrix job orchestrating the pyramid steps above.                                                       |
| **`/.changeset`**                       | Version files consumed by `changeset` CLI for coordinated releases.                                     |
| **`/iac/cdk/construct-tests/`**         | Jest tests using CDK Assertions; snapshot files checked‑in.                                             |
| **`/templates/_shared/ci`**             | Reusable GitHub Action snippets shared by template smoke tests.                                         |

---

### Consequences

* **Pros**: early detection of breaking template changes, reproducible package releases, confidence that generated apps compile & test under current dependencies.
* **Cons**: CI matrix is heavier (\~10‑15 min per PR); nightly job uses LocalStack Pro features (license cost).

### Alternatives Considered

* **No template smoke tests** – faster CI but risk of silent template rot.
* **In‑repo example app** instead of generating on the fly – simpler but doesn’t prove cookiecutter parameters.
