use spacetimedb::{reducer, ReducerContext};

use crate::tables::{user_profile, UserProfile};

#[reducer]
pub fn set_profile(
    ctx: &ReducerContext,
    username: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
    bio: Option<String>,
) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    if username.len() > 32 {
        return Err("Username must be 32 characters or fewer".to_string());
    }

    let profile = ctx.db.user_profile().identity().find(ctx.sender())
        .ok_or("User profile not found. Connect first.")?;

    // Check username uniqueness (if changing)
    if profile.username != username {
        if ctx.db.user_profile().username().find(&username).is_some() {
            return Err(format!("Username '{}' is already taken", username));
        }
    }

    ctx.db.user_profile().identity().update(UserProfile {
        username,
        display_name,
        avatar_url,
        bio,
        ..profile
    });

    Ok(())
}

#[reducer]
pub fn register_agent(
    ctx: &ReducerContext,
    agent_name: String,
    description: String,
) -> Result<(), String> {
    if agent_name.is_empty() {
        return Err("Agent name cannot be empty".to_string());
    }

    let profile = ctx.db.user_profile().identity().find(ctx.sender())
        .ok_or("User profile not found. Connect first.")?;

    if profile.is_agent {
        return Err("Already registered as an agent".to_string());
    }

    // Check username uniqueness if changing
    if profile.username != agent_name {
        if ctx.db.user_profile().username().find(&agent_name).is_some() {
            return Err(format!("Name '{}' is already taken", agent_name));
        }
    }

    ctx.db.user_profile().identity().update(UserProfile {
        username: agent_name,
        bio: Some(description),
        is_agent: true,
        ..profile
    });

    Ok(())
}
