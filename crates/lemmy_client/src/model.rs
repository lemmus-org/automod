use chrono::{DateTime, Utc};
use lemmy_api_common::lemmy_db_schema::newtypes::PrivateMessageId;
use lemmy_api_common::lemmy_db_schema::source::person;
use lemmy_api_common::lemmy_db_schema::source::private_message;
use lemmy_api_common::lemmy_db_schema::source::{comment, community, post};
use std::fmt::{Display, Formatter};
use url::Url;

pub struct Authentication {
    pub jwt: String,
    pub user_id: i32,
}

impl Authentication {
    pub fn new(jwt: String, user_id: i32) -> Self {
        Authentication { jwt, user_id }
    }

    pub fn empty() -> Self {
        Authentication {
            jwt: "".to_string(),
            user_id: -1,
        }
    }
}

#[derive(Default)]
pub struct ModlogActions {
    pub bans: Vec<ModlogBan>,
    pub removals: Vec<ModlogRemoval>,
}

impl ModlogActions {
    pub fn new() -> Self {
        ModlogActions::default()
    }
}

pub enum ModlogBan {
    Site {
        moderator: Person,
        user: Person,
        is_banned: bool,
        reason: Option<String>,
        expires: Option<DateTime<Utc>>,
    },
    Community {
        moderator: Person,
        user: Person,
        community: Community,
        is_banned: bool,
        reason: Option<String>,
        expires: Option<DateTime<Utc>>,
    },
}

impl Display for ModlogBan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModlogBan::Site {
                moderator,
                user,
                is_banned,
                reason,
                expires,
            } => {
                write!(
                    f,
                    "* site_ban = `{}`\r\n\
                    * user = {}\r\n\
                    * mod = {}\r\n\
                    * reason = `{}`\r\n\
                    * expires = `{}`",
                    is_banned,
                    user,
                    moderator,
                    reason.clone().unwrap_or_default(),
                    expires
                        .map(|exp| exp.to_rfc3339())
                        .unwrap_or("N/A".to_string())
                )
            }
            ModlogBan::Community {
                moderator,
                user,
                community,
                is_banned,
                reason,
                expires,
            } => {
                write!(
                    f,
                    "* community_ban = `{}`\r\n\
                    * user = {}\r\n\
                    * community = {}\r\n\
                    * mod = {}\r\n\
                    * reason = `{}`\r\n\
                    * expires = `{}`",
                    is_banned,
                    user,
                    community,
                    moderator,
                    reason.clone().unwrap_or_default(),
                    expires
                        .map(|exp| exp.to_rfc3339())
                        .unwrap_or("N/A".to_string())
                )
            }
        }
    }
}

pub enum ModlogRemoval {
    Comment(ModlogCommentRemoval),
    Post(ModlogPostRemoval),
}

pub struct ModlogCommentRemoval {
    pub moderator: Person,
    pub user: Person,
    pub comment: Comment,
    pub is_removed: bool,
    pub reason: Option<String>,
}

impl Display for ModlogCommentRemoval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "* comment_removed = `{}`\r\n\
            * user = {}\r\n\
            * comment = {}\r\n\
            * mod = {}\r\n\
            * reason = `{}`",
            self.is_removed,
            self.user,
            self.comment,
            self.moderator,
            self.reason.clone().unwrap_or_default()
        )
    }
}

pub struct ModlogPostRemoval {
    pub moderator: Person,
    pub user: Person,
    pub post: Post,
    pub is_removed: bool,
    pub reason: Option<String>,
}

impl Display for ModlogPostRemoval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "* post_removed = `{}`\r\n\
            * user = {}\r\n\
            * post = {}\r\n\
            * mod = {}\r\n\
            * reason = `{}`",
            self.is_removed,
            self.user,
            self.post,
            self.moderator,
            self.reason.clone().unwrap_or_default()
        )
    }
}

pub struct Comment {
    pub id: i32,
    pub content: String,
    pub url: String,
}

impl From<comment::Comment> for Comment {
    fn from(value: comment::Comment) -> Self {
        let url: Url = value.ap_id.into();
        Comment {
            id: value.id.0,
            content: value.content,
            url: url.to_string(),
        }
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

pub struct Post {
    pub id: i32,
    pub name: String,
    pub url: String,
}

impl From<post::Post> for Post {
    fn from(value: post::Post) -> Self {
        let url: Url = value.ap_id.into();
        Post {
            id: value.id.0,
            name: value.name,
            url: url.to_string(),
        }
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

pub struct Community {
    pub name: String,
    pub instance: String,
    pub url: String,
}

impl From<community::Community> for Community {
    fn from(value: community::Community) -> Self {
        let url: Url = value.actor_id.into();
        Community {
            name: value.name,
            instance: url.host().unwrap().to_string(),
            url: url.to_string(),
        }
    }
}

impl Display for Community {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}@{}]({})", self.name, self.instance, self.url)
    }
}

pub struct Person {
    pub id: i32,
    pub name: String,
    pub instance: String,
    pub url: String,
    pub is_local: bool,
}

impl From<person::Person> for Person {
    fn from(value: person::Person) -> Self {
        let url: Url = value.actor_id.into();
        Person {
            id: value.id.0,
            name: value.name,
            instance: url.host().unwrap().to_string(),
            url: url.to_string(),
            is_local: value.local,
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}@{}]({})", self.name, self.instance, self.url)
    }
}

pub struct PrivateMessage {
    pub id: PrivateMessageId, // NOTE: Inner value is private
    pub sender_id: i32,
    pub content: String,
    pub created: DateTime<Utc>,
}

impl From<private_message::PrivateMessage> for PrivateMessage {
    fn from(value: private_message::PrivateMessage) -> Self {
        PrivateMessage {
            id: value.id,
            sender_id: value.creator_id.0,
            content: value.content,
            created: value.published,
        }
    }
}

impl Display for PrivateMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
