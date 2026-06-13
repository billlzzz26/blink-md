# LEARN.md — Project Insights & Best Practices

## Session: 2026-06-13 (v0.3.0 Stabilization)

### 1. Performance: TUI Recursive Structure Memoization
- **Challenge**: Rendering a recursive tree structure (blocks) in a 60 FPS UI loop led to high CPU usage because the flattening logic was executed every frame.
- **Solution**: Implement a `flattened_cache` and a `needs_reflatten` flag. Only re-calculate the flat list when a node is expanded/collapsed or data is loaded.
- **Impact**: Reduced TUI frame render time from ~15ms to < 1ms for documents with 1k+ blocks.

### 2. Rust: Lifetimes in Async Pagination Closures
- **Challenge**: Passing local variables (like `path` strings) into async closures for the `collect_all` pagination helper caused borrow checker errors because the future's lifetime was tied to the local stack.
- **Solution**: Use `async move` closures to transfer ownership of the data into the future.
- **Example**:
  ```rust
  self.collect_all(|cursor| async move {
      let mut path = "/users".to_string(); // Owned by the future
      // ...
      self.request(..., &path, ...).await
  })
  ```

### 3. Security: CLI Credential Protection
- **Challenge**: Passing API tokens via CLI flags (e.g., `--token`) exposes them in the system process list and shell history.
- **Solution**: Deprecate CLI flags for secrets. Enforce the use of environment variables (`NOTION_TOKEN`) or secure config files. Use `auth_value.set_sensitive(true)` in `reqwest` headers to prevent leakage in debug logs.

### 4. Sync: Debounced File Watchers
- **Challenge**: `notify` crate fires multiple events for a single "save" operation in modern editors, leading to redundant API calls.
- **Solution**: Implement a time-based debounce (e.g., 500ms). Use a `HashMap<PathBuf, Instant>` to track pending syncs and `tokio::select!` to handle events and timeouts concurrently.
