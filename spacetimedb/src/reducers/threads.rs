use spacetimedb::{reducer, ReducerContext, Table};

use crate::auth;
use crate::tables::{message, thread, Message, Thread};
use crate::types::MessageType;

#[reducer]
pub fn create_thread(
    ctx: &ReducerContext,
    channel_id: u64,
    name: String,
    initial_message: String,
) -> Result<(), String> {
    if name.is_empty() {
        return Err("Thread name cannot be empty".to_string());
    }
    if initial_message.is_empty() {
        return Err("Initial message cannot be empty".to_string());
    }

    let (channel, _server) = auth::get_server_for_channel(ctx, channel_id)?;
    auth::require_member(ctx, channel.server_id)?;

    let thread = ctx.db.thread().insert(Thread {
        id: 0,
        channel_id,
        name,
        creator: ctx.sender(),
        created_at: ctx.timestamp,
        archived: false,
        last_message_at: ctx.timestamp,
    });

    // Insert the initial message in the thread
    ctx.db.message().insert(Message {
        id: 0,
        channel_id,
        thread_id: Some(thread.id),
        sender: ctx.sender(),
        content: initial_message,
        message_type: MessageType::Text,
        created_at: ctx.timestamp,
        edited_at: None,
        reply_to_id: None,
    });

    Ok(())
}

#[reducer]
pub fn archive_thread(ctx: &ReducerContext, thread_id: u64) -> Result<(), String> {
    let thread = ctx.db.thread().id().find(thread_id)
        .ok_or("Thread not found")?;

    let (channel, _server) = auth::get_server_for_channel(ctx, thread.channel_id)?;

    // Creator or admin+ can archive
    let is_creator = thread.creator == ctx.sender();
    let is_admin = auth::get_member_role(ctx, channel.server_id, ctx.sender())
        .map(|r| r.level() >= crate::types::MemberRole::Admin.level())
        .unwrap_or(false);

    if !is_creator && !is_admin {
        return Err("Only the thread creator or an admin can archive a thread".to_string());
    }

    ctx.db.thread().id().update(Thread {
        archived: true,
        ..thread
    });

    Ok(())
}
