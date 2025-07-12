# ADR‑001 — Static SPA with Selective SSG for Web Deliverables

**Status**: Proposed
**Date**: 2025‑07‑11
**Deciders**: Tool‑set maintainers

---

## 1. Design & Technology Decision

We commit to building every web client as a **self‑contained Single‑Page Application (SPA) written in SvelteKit (Svelte + TypeScript)**.
Key characteristics:

| Aspect             | Choice                                                                                                                                           |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Rendering model    | **Static‑Site Generation (SSG)** for all routes that do not require per‑request personalization; **client‑side hydration** for dynamic behavior. |
| Routing            | SvelteKit’s file‑based router; browser history API.                                                                                              |
| Build output       | Pure static assets (HTML + JS + CSS + images) emitted by `@sveltejs/adapter-static`.                                                             |
| Deployment target  | Amazon S3 bucket + CloudFront (compression & caching); **optional Lambda\@Edge** for SPA fallback & redirects.                                   |
| Language & tooling | TypeScript (executed via **Deno** runtime), Vite, ESLint, Prettier.                                                                              |
| QA tooling         | **vitest** for unit tests (in `src/`), **Playwright** for UI/E2E tests (in `tests/`).                                                            |

### Lambda\@Edge responsibilities

A minimal Node 18 **Lambda\@Edge** (or CloudFront Function) is associated with the default CloudFront behavior to:

* Rewrite unknown SPA routes to `/index.html` for client‑side routing fallback
* Perform canonical redirects (e.g. `example.com → www.example.com`, trailing‑slash normalization)
* Inject security headers (HSTS, CSP, X‑Frame‑Options) without rebuilding the artifact

**What we mean by *Selective SSG*.** Only routes whose HTML output is identical for every user (e.g. marketing pages, documentation, legal pages) are prerendered at build time. Any route that depends on session‑specific or rapidly changing data is served as a regular SPA route and hydrates on the client. Developers opt‑in or out per route via SvelteKit’s `export const prerender = true | false` flag.

**Why not SSR?**
SSG delivers the maximum benefit from AWS’s low‑cost static hosting model while retaining good SEO and first‑paint performance.

---

## 2. Intended Usage in Applications

Developers starting a new project MUST:

1. **Generate the project** with `/templates/frontend` (see §3).
   The template ships with `adapter-static` pre‑configured and a canonical `svelte.config.ts`.
2. **Mark prerenderable routes** in `+page.server.ts` with `export const prerender = true;` to opt into SSG.
   Omit (or set to `false`) on pages that require per‑session data.
3. **Keep all stateful API calls** in `src/lib/api/**` modules using the shared Protobuf/JSON client.
   This separation avoids accidental fetches during `svelte-kit prerender`.
4. **Run `deno task preview:ssg`** locally; the task serves the `build/` folder via `vite preview`, showing exactly what CloudFront will serve.
5. \*\*Push to any branch → GitHub Action builds the static site and executes all tests but **does not deploy**.
   Push to **`main`** → Action builds, deploys the artifact to the **staging** S3/CloudFront stack, then runs smoke, UI, and E2E tests. If all checks pass, the *same artifact* is automatically promoted to **production** without rebuilding.

Rules of thumb:

| Situation                                     | Guidance                                                                                                          |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Public marketing page, docs, landing          | **Prerender**. Improves TTFB and SEO.                                                                             |
| Auth‑gated dashboard, personalised feed       | **Hydrate only**. Mark route non‑prerendered.                                                                     |
| Variant where *some* blocks are user‑specific | Use opaque placeholder components that load data on mount; leave page prerendered to retain SEO for static parts. |

---

## 3. Tool‑Set Support

| Tool‑set component                               | How it enforces / simplifies the decision                                                                                                                                                                                                                         |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`/templates/frontend`**                        | Cookiecutter template that scaffolds a SvelteKit app with: `adapter-static`, TypeScript strict mode, Playwright, vitest, and axe-playwright pre-wired. Unit tests are placed next to the files in `src/`, and Playwright tests are in `tests/`. |
| **`/libraries/frontend/eslint-config`**          | Central ESLint/Prettier/Tailwind config enforcing the coding standards.                                                                                                                                                                                           |
| **`deno.jsonc` & `/scripts`**                    | Shared **Deno tasks** (e.g., `deno task format`, `deno task lint`, `deno task build:ssg`) and a locked permissions manifest to standardize local & CI commands.                                                                                                   |
| **`/iac/cdk/static‑spa-stack.ts`**               | CDK construct exposing `StaticSpaStack(siteName, { domain, certificateArn })` which: 1) provisions the S3 bucket; 2) attaches a CloudFront distro with proper cache‑control & security headers; 3) adds an optional Lambda\@Edge for SPA fallback (`index.html`). |
| **CI/CD pipeline** (`.github/workflows/web.yml`) | `deno task ci:install` → `deno task build:ssg` → upload `build/` artifact → deploy to **staging** S3/CloudFront → run smoke, UI, and E2E tests → on success promote the same artifact to **production** and invalidate CloudFront caches.                         |
| **Gemini CLI prompt pack**                       | Contains “When generating UI components, assume a static‑site SvelteKit app using adapter‑static.” This keeps AI suggestions aligned.                                                                                                                             |
| **Docs** (`/docs/ssg-guidelines.md`)             | Explains prerender flags, cache invalidation, and route design patterns.                                                                                                                                                                                          |

Future enhancements (tracked separately):

* Incremental rebuilds via `svelte-kit sync` and CloudFront selective invalidation.
* Optional ISR (Incremental Static Regeneration) once supported natively in SvelteKit.

---

### Consequences

* Cold starts? None—the site is served as static files.
* Cost predictability: <\$1/month for S3 + CloudFront at prototype scale.
* Trade‑off: user‑specific content requires extra client fetches; server‑side rendering features (e.g., `load` with session data) are unavailable in prerendered routes.

### Alternatives considered

* **SSR on Lambda\@Edge** – increases complexity, erodes cost advantage, cold starts on every request.
* **Next.js / React** – heavier bundle size; ties to Node ecosystem instead of Svelte’s more minimal runtime.
