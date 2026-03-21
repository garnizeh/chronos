---
name: OS & Async Boundary Control
description: Workflow to implement system-level IO (like screen capture) without blocking the async runtime.
trigger: "Implement OS IO: [io_task]"
---
# Workflow: OS & Async Boundary

**Step 1: Identify IO Type**
- Determine if the underlying OS API or C-binding is blocking or natively asynchronous.

**Step 2: Thread Isolation**
- If the operation is blocking (e.g., standard screen capture APIs), wrap the execution inside a `tokio::task::spawn_blocking` block.
- Set up a `tokio::sync::mpsc` channel to transit the captured data back to the asynchronous engine.

**Step 3: Error Mapping**
- Map raw OS errors (like permission denied or memory limits) into the domain's custom `thiserror` enums.
- Ensure no `.unwrap()` is used during the OS interaction to prevent sudden panics.

**Step 4: Resource Cleanup**
- Verify that memory buffers (like captured image frames) are properly dropped or reused to avoid memory leaks.