mod handlers;

use actix_web::web;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use mattermost_core::data::User;
use mattermost_core::modules::UsersModule;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users")
        .route("", web::get().to(handlers::list_users))
        .route("", web::post().to(handlers::create_user))
    );
}

#[derive(Deserialize)]
struct CreateUserQuery {
    /// Token Id
    t: Option<String>,
    /// Invite id
    iid: Option<String>,
}

#[derive(Deserialize)]
struct ListUsersQuery {
    #[serde(default)]
    /// The page to select.
    page: u64,
    #[serde(default = "default_per_page")]
    /// The number of users per page. There is a maximum limit of 200 users per page.
    per_page: u64,
    /// The ID of the team to get users for.
    in_team: Option<String>,
    /// The ID of the team to exclude users for. Must not be used with "in_team" query parameter.
    not_in_team: Option<String>,
    /// The ID of the channel to get users for.
    in_channel: Option<String>,
    /// The ID of the channel to exclude users for. Must be used with "in_channel" query parameter.
    not_in_channel: Option<String>,
    /// The ID of the group to get users for. Must have `manage_system` permission.
    in_group: Option<String>,
    /// When used with `not_in_channel` or `not_in_team`, returns only the users that are allowed to
    /// join the channel or team based on its group constrains.
    group_constrained: Option<bool>,
    /// Whether or not to list users that are not on any team. This option takes precendence over
    /// `in_team`, `in_channel`, and `not_in_channel`.
    without_team: Option<bool>,
    /// Whether or not to list only users that are active. This option cannot be used along with
    /// the `inactive` option.
    active: Option<bool>,
    /// Whether or not to list only users that are deactivated. This option cannot be used along
    /// with the `active` option.
    inactive: Option<bool>,
    /// Returns users that have this role.
    role: Option<String>,
    /// Sort is only available in conjunction with certain options below. The paging parameter is
    /// also always available.
    /// * `in_team`: Can be "", "last_activity_at" or "create_at". When left blank, sorting is done by username.
    /// * `in_channel`: Can be "", "status". When left blank, sorting is done by username. `status` will sort by
    /// User's current status (Online, Away, DND, Offline), then by Username.
    sort: Option<String>,
    /// Comma separated string used to filter users based on any of the specified system roles.
    /// Example: `?roles=system_admin,system_user` will return users that are either system admins or system users
    roles: Option<String>,
    /// Comma separated string used to filter users based on any of the specified channel roles,
    /// can only be used in conjunction with in_channel. Example: `?in_channel=4eb6axxw7fg3je5iyasnfudc5y&channel_roles=channel_user`
    /// will return users that are only channel users and not admins or guests
    channel_roles: Option<String>,
    /// Comma separated string used to filter users based on any of the specified team roles, can
    /// only be used in conjunction with in_team. Example: `?in_team=4eb6axxw7fg3je5iyasnfudc5y&team_roles=team_user`
    /// will return users that are only team users and not admins or guests
    team_roles: Option<String>,
}

fn default_per_page() -> u64 {
    60
}

#[derive(Deserialize, Serialize)]
struct CreateUser {
    email: String,
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    props: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_props: Option<NotifyProps>,
}

#[derive(Deserialize, Serialize)]
struct NotifyProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desktop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desktop_sound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention_keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<bool>,
}