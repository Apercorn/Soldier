# Response & Error Handling

This document covers how to send responses in every interaction scenario
and how to propagate errors back to the user.

---

## Responding to Interactions

### 1. Slash command — initial response

Use `send!`. Poise automatically calls `create_response` or `create_followup`
depending on whether the interaction has already been answered.

```rust
send!(ctx,
  .embed(embeds::info("Title", "Description"))
  .ephemeral(true)
);
```

Do **not** use `response!` here — that bypasses poise's lifecycle tracking.

---

### 2. Slash command — show a modal

```rust
let modal_res = ctx.interaction
  .quick_modal(ctx.serenity_context(), modal)
  .await?
  .ok_or(CmdError::Timeout)?;
```

> ⚠️ After `quick_modal`, the slash command's interaction token is consumed.
> `ctx.send()` (and by extension the error handler's `CmdError::Embed`) will
> fail silently. All subsequent responses must go through `modal_res.interaction`.

---

### 3. Modal submission — response

Respond to `modal_res.interaction` directly:

```rust
response!(modal_res.interaction, ctx,
  .embed(embeds::success("Title", "Done"))
)?;
```

The `response!` macro always sets `.ephemeral(true)` — you do not need to add it.

---

### 4. Component interaction — update the existing message

The component click replaces the button/select panel with new content:

```rust
update_msg!(mci, ctx,
  .embed(embeds::info("Title", "Updated"))
);
```

`update_msg!` automatically clears existing components before applying yours.
Pass new `.components(...)` explicitly if you want to keep or change them.

---

### 5. Component interaction — new ephemeral message

When you want a *new* message instead of editing the component's parent:

```rust
response!(mci, ctx,
  .embed(embeds::info("Title", "New message"))
);
```

---

### 6. Followup (any interaction, after initial response)

```rust
followup!(mci, ctx,
  .embed(embeds::info("Title", "Followup"))
);
```

Followups are valid for 15 minutes after the original interaction.

---

### 7. Component interaction — silent acknowledge

No message is shown; the loading state is dismissed:

```rust
acknowledge!(mci, ctx);
```

---

### 8. Functions reachable from both a slash command and a component

Accept `helper::AnyInteraction` and use the `any_*` macros. They work the same
as the concrete macros but dispatch correctly for each interaction type:

| Macro                        | What it does                                                 |
| ---------------------------- | ------------------------------------------------------------ |
| `any_response!(i, ctx, ...)` | New ephemeral message (all types)                            |
| `any_update!(i, ctx, ...)`   | Updates message for Component; new message for Command/Modal |
| `any_followup!(i, ctx, ...)` | Followup (all types)                                         |
| `any_acknowledge!(i, ctx)`   | Silent ack (Component only; no-op for others)                |

`any_update!` clears components by default. Override with `.components(vec![...])` in the body.

```rust
async fn my_shared_fn(
  ctx: Context<'_>,
  interaction: helper::AnyInteraction,
  // ...
) -> CommandResult {
  any_update!(interaction, ctx,
    .embed(some_embed)
    // .components(vec![btn]) to add buttons, or omit to clear them
  )?;

  Ok(())
}
```

Call sites:

```rust
// From a slash command
my_shared_fn(ctx, helper::AnyInteraction::Command(ctx.interaction.clone()), ...).await?;

// From a component collector
my_shared_fn(ctx, helper::AnyInteraction::Component(mci), ...).await?;

// From a modal submission
my_shared_fn(ctx, helper::AnyInteraction::Modal(modal_res.interaction), ...).await?;

// From a raw serenity::Interaction (e.g. event handler)
my_shared_fn(ctx, helper::AnyInteraction::from(raw_interaction), ...).await?;
```

---

## Error Patterns

### `CmdError::Embed(embed)` — error before any response

Use when the error occurs **before any response has been sent** to the
interaction — i.e. before `send!`, `response!`, `quick_modal`, etc.

```rust
return Err(CmdError::Embed(embeds::error("Title", "Something went wrong.")));

// Or with ? in a method chain:
let id = parse_input().map_err(|_| CmdError::Embed(embeds::error("", "Bad input.")))?;
```

The framework error handler sends this embed via `ctx.send()` as the initial
response to the slash command.

> ⚠️ Do **not** use after `ctx.interaction.quick_modal()` or any direct
> `interaction.create_response()` call — at that point the token is consumed
> and `ctx.send()` will fail. Use `CmdError::Handled` instead.

---

### `CmdError::Handled` — error after a response was already sent

Use when you have **already shown the error embed** to the user from within the
command body. Tells the error handler to do nothing.

With macros:

```rust
response!(modal_res.interaction, ctx,
  .embed(embeds::error("Title", "Bad input."))
)?;
return Err(CmdError::Handled);
```

With `AnyInteraction`:

```rust
any_update!(interaction, ctx,
  .embed(embeds::error("Title", "Not found."))
)?;
return Err(CmdError::Handled);
```

---

### `CmdError::Timeout` — prompt timed out

Return this when a `ComponentInteractionCollector` or modal times out:

```rust
// quick_modal returns None on timeout
.ok_or(CmdError::Timeout)?

// ComponentInteractionCollector returns None on timeout — handle manually:
let Some(mci) = collector.await else {
  return Err(CmdError::Timeout);
};
```

The error handler sends the standard timeout embed.

---

### System errors — `?` propagation

For Roblox API, Discord API, and database errors, just propagate with `?`.
The error handler formats them into user-facing embeds automatically.

```rust
let group = oxid_roblox::group_from_id(id).await?;   // → CmdError::Roblox
let rows   = MyEntity::find().all(db).await?;         // → CmdError::SeaOrm
```

---

## Quick Reference

| Scenario                             | Tool                                               |
| ------------------------------------ | -------------------------------------------------- |
| Slash command initial response       | `send!(ctx, ...)`                                  |
| Show a modal                         | `ctx.interaction.quick_modal(...)`                 |
| Respond to modal submission          | `response!(modal_res.interaction, ctx, ...)`       |
| Component: update existing message   | `update_msg!(mci, ctx, ...)`                       |
| Component: new ephemeral message     | `response!(mci, ctx, ...)`                         |
| Any: followup after initial response | `followup!(mci, ctx, ...)`                         |
| Component: silent ack                | `acknowledge!(mci, ctx)`                           |
| Mixed command + component function   | `any_update!(interaction, ctx, ...)`               |
| Error before any response            | `return Err(CmdError::Embed(embed))`               |
| Error after response already sent    | Respond directly → `return Err(CmdError::Handled)` |
| Timeout                              | `return Err(CmdError::Timeout)`                    |
| System / unexpected errors           | `?` propagation                                    |
