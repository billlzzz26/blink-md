# LEARN.md — Project Insights & Best Practices

## Session: 2026-06-18 (v0.3.1 CI/package/docs sync)

### 1. CI failures are user-facing failures
- Local tests passing does not mean the release is safe.
- GitHub Actions release/cross-platform jobs are part of the user experience because installers and release artifacts depend on them.
- If CI fails, fix the release path before polishing internal docs.

### 2. TUI recursive structure memoization
- Challenge: rendering a recursive tree structure in a 60 FPS UI loop can waste CPU if flattening runs every frame.
- Solution: cache flattened tree output and recompute only when expansion/collapse or data changes.
- Impact: keeps TUI responsive for large documents.

### 3. Rust lifetimes in async pagination closures
- Challenge: moving local owned values into async closures can trigger borrow checker errors if ownership is unclear.
- Solution: use `async move` closures and keep owned values inside the future.
- Example:
  ```rust
  self.collect_all(|cursor| async move {
      let mut path = "/users".to_string();
      self.request(..., &path, ...).await
  })
  ```

### 4. CLI credential protection
- Challenge: passing API tokens via CLI flags exposes them in process lists and shell history.
- Solution: use environment variables or secure config files. Mark sensitive values so debug logs do not leak them.

### 5. Debounced file watchers
- Challenge: `notify` can emit multiple events for one save operation, causing redundant API calls.
- Solution: debounce events with a short timeout and coalesce pending paths before sync.

### 6. Package hygiene
- Local agent state, secrets, and internal conductor docs must never enter `cargo package`.
- Add a CI/package gate that fails when forbidden paths appear.
