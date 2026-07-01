# Adapter Architecture — overhaul for multi-platform extensibility

Status: Proposed (design doc / ADR) · Date: 2026-06-30 · Supersedes nothing
Revised: 2026-07-01 — platform survey, addressing model, dialect-fork scope
(section 2, 3.0). Nothing in section 1 or the Pandoc/Stripe parts of section 2
changed; this revision replaces the write-path template and adds the pieces
that were missing.

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

## 2. Platform survey

Before picking a shape to build on, we looked at every platform actually in
scope: Notion, Lark/Feishu, GitHub Markdown, Obsidian, AppFlowy, Anytype, and
Craft.

### Addressing — who requires an ID up front

| Platform | Addressing | Note |
|---|---|---|
| Notion | ID-first (`page_id`/`block_id`) | no title lookup without a search call |
| Lark/Feishu | ID-first, plus an extra hop | wiki content needs `node_token` resolved to `document_id` before the first real lookup |
| AppFlowy | ID-first (UUID `view_id`) | a public API for third-party integration is effectively missing |
| Anytype | Stable persistent IDs, not content-addressed | objects form a graph via typed relations, not a page tree |
| Craft | block IDs / `rootBlockId` | real scoped API since 2025 (`connect.craft.do`), opt-in per connection |
| Obsidian | path/title-first | a vault is just a folder of `.md` files; the title is the handle |
| GitHub | path-first | `repo + path + ref`, content-addressable via blob SHA |

Every platform backed by a real database is ID-first at the wire level; only
the two file-based platforms (Obsidian, GitHub) are name-first, and only
because they are files, not because their API design is better. There is no
platform to copy for name-based addressing — Notion is not uniquely bad here,
it is the norm. The addressing layer has to be ours; see 3.0.

### Block model — who actually matches Markdown's shape

Notion, Lark Docx, AppFlowy, and Craft are all block trees, one block per
line — the same shape Markdown already has. Google Docs is a paragraph/run/
character-styling tree, closer to a word processor. That mismatch is why
Google Docs is dropped as the write-path template below, even though its
underlying idea (write as a diff, not imperative CRUD) is still worth keeping.

### Markdown forks by surface, not by platform

The same platform can fork markdown twice depending on what the content is
for. Lark is the clearest case:

- `lark_md` (chat/interactive cards): bold, italic, strikethrough, links,
  lists, code — no tables, no images, no dividers as standalone elements.
  Built for text typed fast and read once.
- Docx block API: full document conversion — tables, code blocks, columns,
  highlights all preserved. Built for persisted, structured content.

GitHub does not fork: one GFM dialect renders a commit message, a PR body, an
issue, and a README file the same way. That is evidence the fork is not
required, not evidence that chat and documents need separate treatment.

blink-md's adapters all sit on the persisted-document surface (Notion pages,
Lark Docx, Obsidian notes, GitHub files). Chat/card dialects such as
`lark_md` are out of scope by definition — nothing we convert is a chat
message.

### Templates we are modeling on

| Layer | Template | What we take |
|---|---|---|
| Core conversion | Pandoc | One typed AST as the hub; a Reader per source and Writer per target; filters that transform the AST format-agnostically; extensions/capabilities make lossy conversion explicit. (Our IR + From/ToPlatform is already Pandoc-shaped — we formalize it.) |
| Write / live sync | Lark's `batch_update` blocks endpoint | Represent a write as an ordered list of typed mutation ops applied atomically, computed from a diff, over a block tree that already matches our IR shape. (Google Docs `batchUpdate` has the same diff-to-ops idea, but its paragraph/run model does not match our block-per-line IR — Lark's endpoint gives the same reconciliation pattern without the model mismatch.) |
| API ergonomics | Stripe | Idempotency keys for retried writes; one consistent error envelope; cursor pagination (already have); dated API versioning per platform (Notion already does this). |

Why Pandoc specifically: it is the proven solution to exactly our problem
(N formats ↔ 1 AST) and its open/closed adapter model is what makes adding the
40th format cheap.

---

## 3. Proposed architecture

### 3.0 Addressing — names, not IDs

The local side needs nothing new: a directory of `.md` files already is the
vault/repo model, and the file's path or title is already the handle a person
types.

The remote side is ID-first everywhere (see the survey above), so blink-md
stores the resolved platform ID back into the file's own YAML frontmatter the
first time a page/doc is created remotely — extending the existing
`PropertyValue` system in `src/ir/frontmatter.rs` (Phase B/C) — instead of
keeping a separate index. This is git-diffable, travels with the file, and
needs no side cache to keep in sync; it is the same approach static site
generators (Hugo, Jekyll) and existing Obsidian-to-Notion sync plugins already
use for the same problem. A command that takes a path or title resolves it by
reading that file's own frontmatter first, and only falls back to a
search-by-title API call when the ID is not yet known (first sync, or a file
created outside blink-md).

### 3.1 Split "file formats" from "live platforms"

The root cause of the `dyn Any` fragility is forcing two different things
through one trait. Separate them; both still meet at the IR hub.

```text
                        ┌──────────────── filters ───────────────┐
   bytes ─────Reader────► UniversalDocument ──(transform*)──► UniversalDocument ─────Writer────► bytes
   (md, html, csv, docx)      ▲                                    │                     (md, html, pdf, …)
                              │                                    │
   live API ──Source(async)──┘         diff(current, desired) ──► ChangeSet ──► Sink::apply(async) ──► live API
   (Notion, Lark, GDocs)                                          (batch_update-style)
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

The clearest case: GitHub alerts (`> [!NOTE]`), Obsidian callouts
(`> [!note]`), and Notion callout blocks all collapse into one generic
`Callout { kind, folded, body }` IR node. Every alert/callout syntax is,
underneath, a blockquote with a marker line, so a Writer with no native
callout support still renders it as a plain blockquote showing the marker
text — nothing is silently dropped, only the styling degrades.

### 3.4 Write path — `ChangeSet` (diff/apply model)

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
- M3 — Filters: add the transform stage + 1–2 real filters (e.g. "strip
  default colours", "downgrade unsupported block"), including collapsing
  platform-specific callout/alert variants into the generic `Callout` node.
- M4 — Source/Sink + Capabilities: move Notion to `Source`/`Sink`; add a
  `Capabilities` descriptor and a degradation filter; surface lossy reports;
  store the resolved platform ID back into frontmatter on first create.
- M5 — ChangeSet write path: `diff(current, desired) → ChangeSet`; teach `sync`
  to reconcile (update/move/delete), not just create.

Each Mx is its own small PR with tests.

---

## 6. Open questions

- `async_trait` vs hand-rolled futures for `Source`/`Sink` (MSRV 1.75).
- How granular should `ChangeSet` ops be per platform (Notion block append vs
  Lark's batch_update op shape)? Likely a small per-platform lowering step.
- Do we expose filters/capabilities on the CLI (`--lossy-report`, `--filter`)?
- Frontmatter ID key shape: one `remote_id` + `remote_platform` pair, or
  namespaced keys per platform (`notion_page_id`, `lark_document_id`)?

---

## 7. Recommendation

Adopt Pandoc's Reader/Writer/AST/filters model as the core, a
Lark-`batch_update`-shaped `ChangeSet` for the write path, and Stripe
conventions for error/idempotency/versioning. Store resolved remote IDs in
each file's own frontmatter rather than a side index. Scope is the
persisted-document surface only (pages/files) — chat/card dialects such as
`lark_md` are explicitly out of scope. Land it via M1→M5 so `main` stays
green and shippable throughout.
