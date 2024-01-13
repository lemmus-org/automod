use crate::endpoints::MODLOG;
use crate::model::{
    Comment, Community, ModlogActions, ModlogBan, ModlogCommentRemoval, ModlogPostRemoval,
    ModlogRemoval, Person, Post,
};
use crate::person::{person_get, PersonRef};
use crate::{Client, ClientError};
use chrono::{DateTime, Utc};
use lemmy_api_common::lemmy_db_views_moderator::structs::{
    ModBanFromCommunityView, ModBanView, ModRemoveCommentView, ModRemovePostView,
};
use lemmy_api_common::site::{GetModlog, GetModlogResponse};
use reqwest::StatusCode;

pub async fn modlog_local_get(
    client: &Client,
    since: DateTime<Utc>,
) -> Result<ModlogActions, ClientError> {
    // Create and perform request
    let path = MODLOG;
    let body = GetModlog {
        limit: Some(50),
        ..Default::default()
    };
    let result = client.get(path, true).query(&body).send().await;
    if let Err(err) = result {
        return Err(ClientError::new(path, err.to_string()));
    }

    // Validate response status
    let response = result.ok().unwrap();
    match response.status() {
        StatusCode::OK => {
            // Parse response body
            let body = response.json::<GetModlogResponse>().await;
            if let Err(err) = body {
                return Err(ClientError::new(path, err.to_string()));
            }
            let modlog = body.ok().unwrap();

            let mut actions = ModlogActions::new();

            // Filter Site Bans
            if let Err(err) = get_site_bans(client, modlog.banned, since, &mut actions.bans).await {
                return Err(ClientError::new(path, err.to_string()));
            }

            // Filter Community Bans
            if let Err(err) = get_community_bans(
                client,
                modlog.banned_from_community,
                since,
                &mut actions.bans,
            )
            .await
            {
                return Err(ClientError::new(path, err.to_string()));
            }

            // Filter Comment Removals
            if let Err(err) = get_comment_removals(
                client,
                modlog.removed_comments,
                since,
                &mut actions.removals,
            )
            .await
            {
                return Err(ClientError::new(path, err.to_string()));
            }

            // Filter Post Removals
            if let Err(err) =
                get_post_removals(client, modlog.removed_posts, since, &mut actions.removals).await
            {
                return Err(ClientError::new(path, err.to_string()));
            }

            Ok(actions)
        }
        status => Err(ClientError::new(path, status.to_string())),
    }
}

async fn get_site_bans(
    client: &Client,
    views: Vec<ModBanView>,
    since: DateTime<Utc>,
    actions: &mut Vec<ModlogBan>,
) -> Result<(), ClientError> {
    for view in views {
        // Filter actions that are older than the given timespan
        let timestamp = view.mod_ban.when_;
        if since <= timestamp {
            // Filter any users that are not local
            let banned_user = Person::from(view.banned_person);
            if !banned_user.is_local {
                continue;
            }

            // Look up the moderator details
            let mod_id = PersonRef::Id(view.mod_ban.mod_person_id.0);
            match person_get(client, mod_id).await {
                Ok(mod_user) => {
                    // Filter any moderators that are local
                    if mod_user.is_local {
                        continue;
                    }

                    // Create and append mod action
                    let action = ModlogBan::Site {
                        moderator: mod_user,
                        user: banned_user,
                        is_banned: view.mod_ban.banned,
                        reason: view.mod_ban.reason,
                        expires: view.mod_ban.expires,
                    };
                    actions.push(action);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

async fn get_community_bans(
    client: &Client,
    views: Vec<ModBanFromCommunityView>,
    since: DateTime<Utc>,
    actions: &mut Vec<ModlogBan>,
) -> Result<(), ClientError> {
    for view in views {
        // Filter actions that are older than the given timespan
        let timestamp = view.mod_ban_from_community.when_;
        if since <= timestamp {
            // Filter any users that are not local
            let banned_user = Person::from(view.banned_person);
            if !banned_user.is_local {
                continue;
            }

            // Look up the moderator details
            let mod_id = PersonRef::Id(view.mod_ban_from_community.mod_person_id.0);
            match person_get(client, mod_id).await {
                Ok(mod_user) => {
                    // Filter any moderators that are local
                    if mod_user.is_local {
                        continue;
                    }

                    // Create and append mod action
                    let action = ModlogBan::Community {
                        moderator: mod_user,
                        user: banned_user,
                        community: Community::from(view.community),
                        is_banned: view.mod_ban_from_community.banned,
                        reason: view.mod_ban_from_community.reason,
                        expires: view.mod_ban_from_community.expires,
                    };
                    actions.push(action);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

async fn get_comment_removals(
    client: &Client,
    views: Vec<ModRemoveCommentView>,
    since: DateTime<Utc>,
    actions: &mut Vec<ModlogRemoval>,
) -> Result<(), ClientError> {
    for view in views {
        // Filter actions that are older than the given timespan
        let timestamp = view.mod_remove_comment.when_;
        if since <= timestamp {
            // Filter any users that are not local
            let commenter = Person::from(view.commenter);
            if !commenter.is_local {
                continue;
            }

            // Look up the moderator details
            let mod_id = PersonRef::Id(view.mod_remove_comment.mod_person_id.0);
            match person_get(client, mod_id).await {
                Ok(mod_user) => {
                    // Filter any moderators that are local
                    if mod_user.is_local {
                        continue;
                    }

                    // Create and append mod action
                    let action = ModlogCommentRemoval {
                        moderator: mod_user,
                        user: commenter,
                        comment: Comment::from(view.comment),
                        is_removed: view.mod_remove_comment.removed,
                        reason: view.mod_remove_comment.reason,
                    };
                    actions.push(ModlogRemoval::Comment(action));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

async fn get_post_removals(
    client: &Client,
    views: Vec<ModRemovePostView>,
    since: DateTime<Utc>,
    actions: &mut Vec<ModlogRemoval>,
) -> Result<(), ClientError> {
    for view in views {
        // Filter actions that are older than the given timespan
        let timestamp = view.mod_remove_post.when_;
        if since <= timestamp {
            // Filter any users that are not local
            let user_id = PersonRef::Id(view.post.creator_id.0);
            let poster = match person_get(client, user_id).await {
                Ok(user) => user,
                Err(err) => {
                    return Err(err);
                }
            };
            if !poster.is_local {
                continue;
            }

            // Look up the moderator details
            let mod_id = PersonRef::Id(view.mod_remove_post.mod_person_id.0);
            match person_get(client, mod_id).await {
                Ok(mod_user) => {
                    // Filter any moderators that are local
                    if mod_user.is_local {
                        continue;
                    }

                    // Create and append mod action
                    let action = ModlogPostRemoval {
                        moderator: mod_user,
                        user: poster,
                        post: Post::from(view.post),
                        is_removed: view.mod_remove_post.removed,
                        reason: view.mod_remove_post.reason,
                    };
                    actions.push(ModlogRemoval::Post(action));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}
