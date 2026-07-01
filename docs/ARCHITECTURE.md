# Adapter Architecture — overhaul for multi-platform extensibility

Status: Proposed (design doc / ADR) · Date: 2026-06-30 · Supersedes nothing

This document proposes how to restructure blink-md's conversion/sync core so
that adding a new platform is "write two small adapters and register them",
not "hand-write a thousand-line match and downcast `Box<dyn Any>`".

It is a design contract to agree on before cutting code. Implementation is
intended to land incrementally and non-breaking (see [Migration](#migration)).

---

## 1. Where we are today

```text
[ Source platform ] --FromPlatform--> [ UniversalDocument (IR) ] --ToPlatform--> [ Target platform ]
```

- IR hub — `src/ir/` (`UniversalDocument` = metadata + blocks + styles).
  Good. This is the right shape and we keep it.
- Adapters — `src/converter/{markdown,github_markdown,lark_sheets,notion,
  markdown_frontmatter}.rs` implement `FromPlatform`/`ToPlatform`
  (`src/converter/mod.rs`).
- Registry — `ConverterRegistry` wraps adapters as `dyn FromPlatformDyn`
  via `Box<dyn Any>` downcasting.

### Pain points the overhaul must fix

1. Heterogeneous `Input` associated type. File formats set
   `type Input = String`; Notion sets `type Input = PageWithBlocks`. Because
   the input type varies, the registry can only erase it to `Box<dyn Any>` and
   downcast at runtime — a type error becomes a runtime failure instead of a
   compile error.
2. No transform stage. Conversion is strictly `1 platform → IR → 1 platform`.
   There is nowhere to put format-agnostic operations (strip colors, downgrade
   unsupported blocks, redact, re-heading, …).
3. Lossy conversions are implicit. For a block kind the target does not
   support, `block_ir_to_notion` returns an error or drops data deep inside a
   match; the caller cannot ask "does this target support tables?" up front.
   (GFM tables specifically are now handled, but the general gap remains.)
4. Write path is create-only. `sync_cmd` always calls `create_page`. There is
   no notion of "diff the desired IR against what is live and apply the delta",
   so real sync (update/move/delete) is impossible.
5. Adapter boilerplate. Adding a platform means two large hand-written `match`
   functions (`block_to_ir` / `blocks_to_notion`).

---

## 2. Templates we are modeling on

| Layer | Template | What we take |
|---|---|---|
| Core conversion | Pandoc | One typed AST as the hub; a Reader per source and Writer per target; filters that transform the AST format-agnostically; extensions/capabilities make lossy conversion explicit. (Our IR + From/ToPlatform is already Pandoc-shaped — we formalize it.) |
| Write / live sync | Google Docs `documents.batchUpdate` | Represent a write as an ordered list of typed mutation ops applied atomically, computed from a diff — instead of imperative per-call CRUD with partial-failure risk. |
| API ergonomics | Stripe | Idempotency keys for retried writes; one consistent error envelope; cursor pagination (already have); dated API versioning per platform (Notion already does this). |

Why Pandoc specifically: it is the proven solution to exactly our problem
(N formats ↔ 1 AST) and its open/closed adapter model is what makes adding the
40th format cheap.

---

## 3. Proposed architecture

### 3.1 Split "file formats" from "live platforms"

The root cause of the `dyn Any` fragility is forcing two different things
through one trait. Separate them; both still meet at the IR hub.

```text
                        ┌──────────────── filters ───────────────┐
   bytes ─────Reader────► UniversalDocument ──(transform*)──► UniversalDocument ─────Writer────► bytes
   (md, html, csv, docx)      ▲                                    │                     (md, html, pdf, …)
                              │                                    │
   live API ──Source(async)──┘         diff(current, desired) ──► ChangeSet ──► Sink::apply(async) ──► live API
   (Notion, Lark, GDocs)                                          (batchUpdate-style)
```

- Reader / Writer — synchronous, total functions over bytes. Uniform
  signatures, with no associated `Input` type, so the registry holds
  homogeneous trait objects and the `Box<dyn Any>` disappears.

  ```rust
  pub trait Reader { fn read(&self, input: &[u8]) -> Result<UniversalDocument, ConvertError>; }
  pub trait Writer { fn write(&self, doc: &UniversalDocument) -> Result<Vec<u8>, ConvertError>; }
  ```

- Source / Sink — async, for live platforms that fetch/apply over an API
  client. The fetch shape (e.g. `PageWithBlocks`) stays a private detail of the
  adapter, not a public associated type leaking into the registry.

  ```rust
  #[async_trait] pub trait Source { async fn fetch(&self, client: &Client, id: &str) -> Result<UniversalDocument, _>; }
  #[async_trait] pub trait Sink   { async fn apply(&self, client: &Client, plan: &ChangeSet) -> Result<(), _>; }
  ```

### 3.2 Filters (the missing middle)

A filter is `fn(&mut UniversalDocument)` (or a small trait). A conversion is
`read → run filters → write`. This is where colour-stripping, block
downgrading, redaction, TOC injection, etc. live — once, for every format.

### 3.3 Capability descriptors — make loss explicit

Each Writer/Sink declares what it supports:

```rust
pub struct Capabilities { pub blocks: BlockKindSet, pub inline: InlineKindSet, pub lossless_roundtrip: bool }
```

The engine can then degrade gracefully on purpose (a built-in filter maps
unsupported kinds to the nearest supported one) and report what was lossy,
instead of silently dropping or erroring deep in a match.

### 3.4 Write path — `ChangeSet` (batchUpdate model)

```text
desired IR  ─┐
             ├─ diff ──► ChangeSet { ops: Vec<Op> }  ──► Sink::apply (atomic/ordered)
current IR  ─┘                  (Insert/Update/Move/Delete typed ops)
```

This turns `sync` from "always create" into real reconciliation, and the same
`ChangeSet` abstraction serves every live platform.

### 3.5 Error & ergonomics

- Generalize `ConverterError` → one `ConvertError` envelope (kind + context +
  optional source platform), Stripe-style consistent shape.
- Idempotency key on `Sink::apply` so retried syncs do not double-write.

---

## 4. Adding a new platform after the overhaul

1. Implement `Reader`/`Writer` (file format) or `Source`/`Sink` (live API).
2. Declare `Capabilities`.
3. `registry.register(...)`.

No registry edits, no `Any`, no touching other adapters. That is the whole
point.

---

## 5. Migration (incremental, non-breaking) {#migration}

The current `FromPlatform`/`ToPlatform` adapters keep working the entire time.

- M1 — IR contract: freeze/version the IR; add a doc test snapshot so the hub
  is a stable contract. (No behaviour change.)
- M2 — Reader/Writer traits: introduce the new traits; provide blanket impls
  bridging the existing `FromPlatform`/`ToPlatform` adapters whose `Input`/
  `Output` associated types are `String`, so file-format adapters move over
  with zero rewrites. Drop `Box<dyn Any>` from the registry for these.
- M3 — Filters: add the transform stage + 1–2 real filters (e.g. "strip default
  colours", "downgrade unsupported block").
- M4 — Source/Sink + Capabilities: move Notion to `Source`/`Sink`; add a
  `Capabilities` descriptor and a degradation filter; surface lossy reports.
- M5 — ChangeSet write path: `diff(current, desired) → ChangeSet`; teach `sync`
  to reconcile (update/move/delete), not just create.

Each Mx is its own small PR with tests.

---

## 6. Open questions

- `async_trait` vs hand-rolled futures for `Source`/`Sink` (MSRV 1.75).
- How granular should `ChangeSet` ops be per platform (Notion block append vs
  Google Docs index-based inserts)? Likely a small per-platform lowering step.
- Do we expose filters/capabilities on the CLI (`--lossy-report`, `--filter`)?

---

## 7. Recommendation

Adopt Pandoc's Reader/Writer/AST/filters model as the core, Google Docs
`batchUpdate`-style ChangeSets for the write path, and Stripe conventions for
error/idempotency/versioning. Land it via M1→M5 so `main` stays green and
shippable throughout.
