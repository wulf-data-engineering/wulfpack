# ADR-004 — Protobuf‑over‑HTTP/1.1 as the Remote‑API Contract

**Status**: Proposed
**Date**: 2025‑07‑11
**Deciders**: Tool‑set maintainers

---

## 1. Design & Technology Decision

Client‑to‑service APIs should exchange messages encoded with Protocol Buffers (proto3) over plain HTTP/1.1. The tool set favors Protobuf because it yields compact payloads, strict typing, and painless forward‑compatibility. Other formats might make sense on a per‑endpoint basis when human readability, third‑party interoperability, or data‑warehouse integration outweighs binary efficiency.

| Aspect          | Choice                                                                                                   |
| --------------- | -------------------------------------------------------------------------------------------------------- |
| Wire format     | **`application/x‑protobuf`** preferred; `application/json` allowed depending on the `DATA_FORMAT` toggle |
| Transport       | HTTP/1.1 via Amazon API Gateway v2 (REST) for request/response; EventBridge & SQS for async              |
| Spec location   | `/proto/*.proto` files versioned in monorepo; linted with **Buf**                                        |
| Code generation | **Prost** for Rust (backend), **ts‑proto** for TypeScript/Deno (frontend & Tauri)                        |
| Versioning      | Backward‑compatible field numbering; additive changes preferred; breaking changes bump major SemVer      |
| Error envelope  | `ErrorProto { code: int32, message: string, details?: Any }`                                             |

**Why not gRPC?**
AWS Lambda lacks native gRPC support and we avoid the extra complexity of sidecar proxies. HTTP/1.1 works with all edge services (CloudFront, API Gateway), browser Fetch, and mobile WebView.

---

## 2. Intended Usage in Applications

1. **Define messages** in `/proto/foo_service.proto`; run `deno task proto:lint` (Buf) and `deno task proto:gen` to regenerate Rust and TS stubs.
2. **Implement handler** in Rust Lambda:

   ```rust
   async fn create_item(req: proto::CreateItemRequest) -> Result<proto::CreateItemResponse> { /* … */ }
   ```
3. **Call from web/Tauri**:

   ```ts
   import { CreateItemRequest, CreateItemResponse } from "@proto/foo_service";
   const res = await api.post<CreateItemResponse>("/v1/items", CreateItemRequest.create({ … }));
   ```
4. **Switch data format** locally by prefixing the dev task with an environment variable:

```bash
DATA_FORMAT=json deno task dev
```

The shared fetch wrapper reads `DATA_FORMAT`; if set to **json** it sends `Accept: application/json` and `Content‑Type: application/json`. The Rust Lambda detects these headers and transparently falls back to `serde_json` for deserialization/serialization.
5\. **Contract compatibility**: CI job runs `buf breaking --against main` to block incompatible field removals/renames.
6\. **Error handling**: clients map `ErrorProto.code` to typed exceptions; unknown codes surface as generic “UnexpectedError”.

Guidelines:

| Concern         | Guideline                                                                                      |
| --------------- | ---------------------------------------------------------------------------------------------- |
| Field evolution | Only *add* fields; mark deprecated ones with `[deprecated = true]`; never reuse field numbers. |
| Pagination      | Use `next_page_token` pattern; token is opaque to callers.                                     |
| Auth            | Pass Cognito JWT in `Authorization: Bearer` header; server returns `401` with `ErrorProto`.    |
| Observability   | Log request\_id, method, status, and `error_code` in structured `tracing` logs.                |

---

## 3. Tool‑Set Support

| Component                                           | Support details                                                                                                     |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| **`/proto`**                                        | Central schema directory; pre‑commit hook formats with `buf format`.                                                |
| **`deno_tasks`** (`deno.jsonc`)                     | `proto:lint`, `proto:gen`, `proto:check` tasks wrapping Buf CLI.                                                    |
| **`/libraries/shared/proto-defs`**                  | Generated Rust crate + Deno package published to internal registry for reuse.                                       |
| **CI step** (`buf‑check.yml`)                       | Runs `buf lint` and `buf breaking`; fails PR if violations found.                                                   |
| **API Gateway construct** (`/iac/cdk/proto-api.ts`) | Generates REST API resources from a proto‑annotated YAML; sets correct binary media types and Stage settings.       |
| **Gemini & Claude prompt packs**                    | Remind AI to use generated stubs (`import { FooRequest } …`) and to include `Content‑Type: application/x‑protobuf`. |

---

### Consequences

* **Pros**: compact messages, language‑neutral contracts, single schema source, easy JSON debugging path.
* **Cons**: no bidirectional streaming; binary payloads require custom tooling in Postman‑style debuggers.

### Alternatives Considered

* **gRPC over ALB** – would require containerizing services and maintaining sidecars; breaks Lambda‑first mindset.
* **Pure JSON/REST** – easier to sniff but larger payloads, looser contracts, and no forward‑compatibility guarantees.
