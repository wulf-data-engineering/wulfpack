# ADR-002 — Rust‑based AWS Lambda Functions as the Default Compute Unit

**Status**: Proposed
**Date**: 2025‑07‑11
**Deciders**: Tool‑set maintainers

---

## 1. Design & Technology Decision

Every piece of backend compute **SHALL be implemented as a Rust 2021 AWS Lambda Function** built with `cargo lambda`.
The function artifacts are compiled for the `provided.al2` custom runtime (or the future official Rust runtime when available) and deployed through AWS CDK.

| Aspect             | Choice                                                                                               |
| ------------------ | ---------------------------------------------------------------------------------------------------- |
| Language           | **Rust 1.79+** (2021 edition)                                                                        |
| Build command      | `cargo lambda build --release --arm64`                                                               |
| Package format     | ZIP artifact uploaded by CDK; switch to container image only if >50 MB                               |
| Runtime            | Custom runtime (`bootstrap`) on Amazon Linux 2                                                       |
| Invocation sources | HTTP API Gateway v2, EventBridge, SQS, DynamoDB Streams                                              |
| Observability      | `tracing` JSON logs → CloudWatch, X‑Ray tracing via `aws_lambda_powertools`                          |
| Configuration      | Environment variables resolved from **SSM Parameter Store**; secrets pulled from **Secrets Manager** |
| IAM                | One least‑privilege role per function, scoped to its data and AWS services                           |

**Why not always‑on containers (ECS/Fargate)?**
Lambda’s scale‑to‑zero billing and sub‑second cold start times (with Rust) align with our cost and ops goals; we will upgrade to containerized services only for workloads that exceed the 15‑minute limit or demand gRPC streaming.

---

## 2. Intended Usage in Applications

1. **Scaffold** a new Lambda with `/templates/backend` choosing *HTTP API* or *Event Listener*.
2. **Develop locally** with `cargo lambda watch -e events/example.json` for hot‑reload testing.
3. **Handler signature** example:

   ```rust
   async fn handler(evt: api::RequestProto, ctx: lambda_runtime::Context) -> Result<api::ResponseProto> { /* … */ }
   ```
4. **Logging**: `tracing::info!(request_id = %ctx.request_id(), "processing");`.
5. **Configuration**: `let settings = settings::Settings::from_env()?;` (reads SSM & Secrets).
6. **Testing**: unit + property tests via `cargo nextest`; integration tests run against LocalStack with `make local-integration`.
7. **CI flow**: commit → checks → on *main* branch artifact deploys to *staging* → smoke, UI, E2E → auto‑promote to *production*.

Ground rules:

| Concern             | Guideline                                                                                           |
| ------------------- | --------------------------------------------------------------------------------------------------- |
| **Function size**   | Keep ZIP < 20 MB; shared code goes to a Lambda layer.                                               |
| **Cold start**      | Target **arm64**, 256–512 MB memory; enable provisioned concurrency only on latency‑critical paths. |
| **Error handling**  | Return typed `ErrorProto { code, message }`; panic is caught by middleware and logged.              |
| **Execution limit** | < 10 s for interactive APIs; longer jobs are off‑loaded to Step Functions.                          |

---

## 3. Tool‑Set Support

| Component                                            | How it supports the decision                                                                                                                                                                                       |
| ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **`/templates/backend`**                             | Cookiecutter skeleton with Cargo workspace, `lambda_runtime`, `prost`, Tower middleware, CI stub.                                                                                                                  |
| **`/libraries/backend/common`**                      | Shared crates: `proto_defs`, `settings`, `tracing_setup`, `error_ext`.                                                                                                                                             |
| **`/iac/cdk/rust-function.ts`**                      | CDK construct `RustFunction(scope, id, { entry, events, env })` that compiles with Docker, packages, creates log group, IAM role, and enables X‑Ray.                                                               |
| **CI/CD pipeline** (`.github/workflows/backend.yml`) | Steps: `rustup` + cache → `cargo fmt`, `cargo clippy` → tests (`cargo nextest`) → `cargo audit`, `cargo deny` → build release → LocalStack integration tests → deploy to staging → promote to production on green. |
| **Gemini CLI prompt pack**                           | “Assume Rust Lambda, async/await, `tracing`, `tokio`” so AI code aligns with house style.                                                                                                                          |
| **Docs** (`/docs/rust-lambda-guidelines.md`)         | Memory tuning chart, cold‑start troubleshooting, best‑practice examples.                                                                                                                                           |

---

### Consequences

* **Pros**: memory‑safe, small artifacts (\~5–10 MB), fast cold starts (<150 ms median), single language for all compute paths.
* **Cons**: longer compile times vs. Go; Rust learning curve; custom runtime maintenance until AWS ships an official one.

### Alternatives Considered

* **Go Lambda** – faster compile but larger binaries and no built‑in Protobuf ergonomics.
* **ECS/Fargate microservices** – predictable latency but higher idle cost and more ops surface area.