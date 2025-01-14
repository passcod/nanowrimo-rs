use crate::utils::*;
use crate::{
    ActionType, AdheresTo, AdminLevel, BadgeType, ContentType, DisplayStatus, EntryMethod,
    EventType, Feeling, GroupType, How, InvitationStatus, JoiningRule, NanoKind, PrivacySetting,
    ProjectStatus, RegistrationPath, UnitType, Where, WritingType,
};

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use paste::paste;
use serde::{Deserialize, Serialize};

// TODO: May be possible to make time_zone a type from chrono

/// The response of the Nano API when a command results in an expected error
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum NanoError {
    /// A simple error with just a basic message
    SimpleError { error: String },
    /// A response with multiple complex errors
    ErrorList { errors: Vec<ErrorData> },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ErrorData {
    #[serde(deserialize_with = "de_str_num")]
    pub code: u64,
    pub detail: String,
    #[serde(deserialize_with = "de_str_num")]
    pub status: u64,
    pub title: String,
}

/// The response from logging into the Nano API
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LoginResponse {
    /// The authorization token for this log-in session
    pub auth_token: String,
}

/// Information about Nano's current funraising goals
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Fundometer {
    /// The current end-goal
    pub goal: u64,
    /// The current total raised
    #[serde(deserialize_with = "de_str_num")]
    pub raised: f64,
    /// The number of people who have donated
    #[serde(rename = "donorCount")]
    pub donor_count: u64,
}

/// An item from the Nano store
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StoreItem {
    /// The unique slug for this item
    pub handle: String,
    /// The URL for the image tied to this item
    #[serde(deserialize_with = "de_heighten_img")]
    pub image: String,
    /// The user-facing title of this item
    pub title: String,
}

/// A successful response from a call to the API which returns multiple items.
/// Is generic over the inner data type, which allows for the case of a known return type
/// to avoid needing an unwrap. Defaults to the generic Object
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CollectionResponse<D: ObjectInfo = Object> {
    /// The array of returned objects
    pub data: Vec<D>,
    /// Any included linked objects
    pub included: Option<Vec<Object>>,

    /// Extra info provided for Post objects
    #[serde(flatten)]
    pub post_info: Option<Box<PostInfo>>,
}

impl<D: ObjectInfo> CollectionResponse<D> {
    /// Find the instance of an ObjectRef in this response's included list, if
    /// an instance exists. Otherwise returns None
    pub fn get_ref(&self, obj_ref: &ObjectRef) -> Option<&Object> {
        self.included.as_ref().and_then(|val| {
            val.iter()
                .find(|obj| obj.id() == obj_ref.id && obj.kind() == obj_ref.kind)
        })
    }
}

/// A successful response from a call to the API which returns a single item.
/// Is generic over the inner data type, which allows for the case of a known return type
/// to avoid needing an unwrap. Defaults to the generic Object
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ItemResponse<D: ObjectInfo = Object> {
    /// The returned object
    pub data: D,
    /// Any included linked objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<Object>>,

    /// Extra info provided for Post/Page objects
    #[serde(flatten)]
    pub post_info: Option<Box<PostInfo>>,
}

impl<D: ObjectInfo> ItemResponse<D> {
    /// Find the instance of an ObjectRef in this response's included list, if
    /// an instance exists. Otherwise returns None
    pub fn get_ref(&self, obj_ref: &ObjectRef) -> Option<&Object> {
        self.included.as_ref().and_then(|val| {
            val.iter()
                .find(|obj| obj.id() == obj_ref.id && obj.kind() == obj_ref.kind)
        })
    }
}

/// The extra info provided when getting a Post/Page object
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostInfo {
    /// Posts that come after this one
    pub after_posts: Vec<ItemResponse<PostObject>>,
    /// Info about the author(s) of this post
    pub author_cards: CollectionResponse<PostObject>,
    /// Posts that come before this one
    pub before_posts: Vec<ItemResponse<PostObject>>,
}

/// A reference to an included [`Object`]. Declares the kind and ID of the Object,
/// so that it can be uniquely located in the include list
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ObjectRef {
    /// The ID of the referenced Object
    #[serde(deserialize_with = "de_str_num", serialize_with = "se_str_id")]
    pub id: u64,
    /// The kind of the referenced Object
    #[serde(
        rename = "type",
        deserialize_with = "de_nanokind",
        serialize_with = "se_nanokind"
    )]
    pub kind: NanoKind,
}

/// A trait for all types that represent an 'Object' in the Nano API. See [`Object`] for the
/// most general form of this.
pub trait ObjectInfo: std::fmt::Debug {
    /// Retrive the kind of this Object
    fn kind(&self) -> NanoKind;
    /// Retrieve the ID of this object
    fn id(&self) -> u64;
    /// Get the relationships of this Object, if it has any
    fn relationships(&self) -> &Option<RelationInfo>;
    /// Get the links for this Object, of which there should always be at least a link for `self`
    fn links(&self) -> &Option<LinkInfo>;
}

/// A common type for all Nano API objects. Most useful when you're either not sure of an API type,
/// or want to accept multiple types in your program. See [`ObjectInfo`] for the kind of things
/// all these objects have in common
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Object {
    #[serde(rename = "badges")]
    Badge(BadgeObject),
    #[serde(rename = "challenges")]
    Challenge(ChallengeObject),
    #[serde(rename = "daily-aggregates")]
    DailyAggregate(DailyAggregateObject),
    #[serde(rename = "favorite-authors")]
    FavoriteAuthor(FavoriteAuthorObject),
    #[serde(rename = "favorite-books")]
    FavoriteBook(FavoriteBookObject),
    #[serde(rename = "genres")]
    Genre(GenreObject),
    #[serde(rename = "groups")]
    Group(GroupObject),
    #[serde(rename = "group-external-links")]
    GroupExternalLink(GroupExternalLinkObject),
    #[serde(rename = "locations")]
    Location(LocationObject),
    #[serde(rename = "nanomessages")]
    NanoMessage(NanoMessageObject),
    #[serde(rename = "notifications")]
    Notification(NotificationObject),
    #[serde(rename = "pages")]
    Page(PageObject),
    #[serde(rename = "posts")]
    Post(PostObject),
    #[serde(rename = "projects")]
    Project(ProjectObject),
    #[serde(rename = "project-sessions")]
    ProjectSession(ProjectSessionObject),
    #[serde(rename = "stopwatches")]
    StopWatch(StopWatchObject),
    #[serde(rename = "timers")]
    Timer(TimerObject),
    #[serde(rename = "users")]
    User(UserObject),
    #[serde(rename = "writing-locations")]
    WritingLocation(WritingLocationObject),
    #[serde(rename = "writing-methods")]
    WritingMethod(WritingMethodObject),

    #[serde(rename = "group-users")]
    GroupUser(GroupUserObject),
    #[serde(rename = "location-groups")]
    LocationGroup(LocationGroupObject),
    #[serde(rename = "project-challenges")]
    ProjectChallenge(ProjectChallengeObject),
    #[serde(rename = "user-badges")]
    UserBadge(UserBadgeObject),
}

impl Object {
    fn inner(&self) -> &dyn ObjectInfo {
        match self {
            Object::Badge(data) => data,
            Object::Challenge(data) => data,
            Object::DailyAggregate(data) => data,
            Object::FavoriteAuthor(data) => data,
            Object::FavoriteBook(data) => data,
            Object::Genre(data) => data,
            Object::Group(data) => data,
            Object::GroupExternalLink(data) => data,
            Object::Location(data) => data,
            Object::NanoMessage(data) => data,
            Object::Notification(data) => data,
            Object::Page(data) => data,
            Object::Post(data) => data,
            Object::Project(data) => data,
            Object::ProjectSession(data) => data,
            Object::StopWatch(data) => data,
            Object::Timer(data) => data,
            Object::User(data) => data,
            Object::WritingLocation(data) => data,
            Object::WritingMethod(data) => data,

            Object::GroupUser(data) => data,
            Object::LocationGroup(data) => data,
            Object::ProjectChallenge(data) => data,
            Object::UserBadge(data) => data,
        }
    }
}

impl ObjectInfo for Object {
    fn kind(&self) -> NanoKind {
        self.inner().kind()
    }

    fn id(&self) -> u64 {
        self.inner().id()
    }

    fn relationships(&self) -> &Option<RelationInfo> {
        self.inner().relationships()
    }

    fn links(&self) -> &Option<LinkInfo> {
        self.inner().links()
    }
}

const fn is_zero(n: &u64) -> bool {
    *n == 0
}

macro_rules! obj_ty {
    ($( $name:ident )+) => {
        paste! {
            $(

            #[doc = "A struct representing an object of kind " $name]
            #[derive(Clone, Serialize, Deserialize, Debug)]
            pub struct [<$name Object>] {
                #[serde(deserialize_with = "de_str_num", skip_serializing_if = "is_zero")]
                pub id: u64,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub relationships: Option<RelationInfo>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub links: Option<LinkInfo>,

                /// The attributes unique to this object
                pub attributes: [<$name Data>]
            }

            impl ObjectInfo for [<$name Object>] {
                fn kind(&self) -> NanoKind {
                    NanoKind::$name
                }

                fn id(&self) -> u64 {
                    self.id
                }

                fn relationships(&self) -> &Option<RelationInfo> {
                    &self.relationships
                }

                fn links(&self) -> &Option<LinkInfo> {
                    &self.links
                }
            }

            impl Object {
                #[doc = "Get this Object as kind " $name ", or panic"]
                #[track_caller]
                pub fn [<unwrap_ $name:snake>](&self) -> &[<$name Object>] {
                    if let Object::$name(inner) = self {
                        inner
                    } else {
                        panic!(concat!("Expected object type ", stringify!($name), " while unwrapping Object"))
                    }
                }
            }
            )+
        }
    }
}

obj_ty!(
    Badge
    Challenge
    DailyAggregate
    FavoriteAuthor
    FavoriteBook
    Genre
    Group
    GroupExternalLink
    Location
    NanoMessage
    Notification
    Page
    Post
    Project
    ProjectSession
    StopWatch
    Timer
    User
    WritingLocation
    WritingMethod

    GroupUser
    LocationGroup
    ProjectChallenge
    UserBadge
);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct BadgeData {
    pub active: bool,
    pub adheres_to: AdheresTo,
    pub awarded: String,
    pub awarded_description: String,
    pub badge_type: BadgeType,
    pub description: String,
    pub generic_description: String,
    pub list_order: u64,
    pub suborder: Option<u64>,
    pub title: String,
    pub unawarded: String,
    pub winner: bool,
}

/// A challenge (Nano, Camp Nano, or custom).
/// The Optional fields will generally be populated for Nanos or Camps,
/// but null for custom challenges. (Warning: This is only mostly, not absolutely, true)
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ChallengeData {
    pub default_goal: u64,
    pub ends_at: NaiveDate,
    pub event_type: Option<EventType>,
    pub flexible_goal: Option<bool>,
    pub name: String,
    pub prep_starts_at: Option<NaiveDate>,
    pub starts_at: NaiveDate,
    pub unit_type: UnitType,
    pub user_id: u64,
    pub win_allowed_at: Option<NaiveDate>,
    pub writing_type: WritingType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct DailyAggregateData {
    pub count: u64,
    pub day: NaiveDate,
    pub project_id: u64,
    pub unit_type: UnitType,
    pub user_id: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct FavoriteAuthorData {
    pub name: String,
    pub user_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct FavoriteBookData {
    pub title: String,
    pub user_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GenreData {
    pub name: String,
    /// The user who created this Genre label
    pub user_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GroupData {
    pub approved_by_id: u64,
    pub avatar: Option<String>,
    pub cancelled_by_id: u64,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub end_dt: Option<DateTime<Utc>>,
    pub forum_link: Option<String>,
    pub group_id: Option<u64>,
    pub group_type: GroupType,
    pub joining_rule: Option<JoiningRule>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub max_member_count: Option<u64>,
    pub member_count: Option<u64>,
    pub name: String,
    pub plate: Option<String>,
    pub slug: String,
    pub start_dt: Option<DateTime<Utc>>,
    pub time_zone: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub url: Option<String>,
    pub user_id: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GroupExternalLinkData {
    pub group_id: u64,
    pub label: Option<String>,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LocationData {
    pub city: String,
    pub country: String,
    pub county: Option<String>,
    pub formatted_address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub map_url: Option<String>,
    pub municipality: Option<String>,
    pub name: String,
    pub neighborhood: Option<String>,
    #[serde(deserialize_with = "de_opt_str_num")]
    pub postal_code: Option<u64>,
    pub state: String,
    #[serde(rename = "street1")]
    pub street1: Option<String>,
    #[serde(rename = "street2")]
    pub street2: Option<String>,
    pub utc_offset: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct NanoMessageData {
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub group_id: u64,
    pub official: bool,
    pub send_email: Option<bool>,
    pub sender_avatar_url: Option<String>,
    pub sender_name: Option<String>,
    pub sender_slug: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub user_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct NotificationData {
    pub action_id: Option<u64>,
    pub action_type: ActionType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub data_count: Option<u64>,
    pub display_at: DateTime<Utc>,
    pub display_status: DisplayStatus,
    pub headline: String,
    pub image_url: Option<String>,
    pub last_viewed_at: Option<DateTime<Utc>>,
    pub redirect_url: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub user_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct PageData {
    pub body: String,
    pub url: String,
    pub headline: String,
    pub content_type: ContentType,
    pub show_after: Option<DateTime<Utc>>,
    pub promotional_card_image: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct PostData {
    pub api_code: Option<String>, // TODO: ???
    pub body: String,
    pub card_image: Option<String>,
    pub content_type: ContentType,
    pub expires_at: Option<NaiveDate>,
    pub external_link: Option<String>,
    pub headline: String,
    pub offer_code: Option<String>,
    pub order: Option<u64>,
    pub published: bool,
    pub subhead: Option<String>, // TODO: ???
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ProjectData {
    pub cover: Option<String>,
    pub created_at: DateTime<Utc>,
    pub excerpt: Option<String>,
    pub pinterest_url: Option<String>,
    pub playlist_url: Option<String>,
    pub primary: Option<i64>, // TODO: Figure out what this means. It's not an enum
    pub privacy: PrivacySetting,
    pub slug: String,
    pub status: ProjectStatus,
    pub summary: Option<String>,
    pub title: String,
    pub unit_count: Option<u64>,
    pub unit_type: UnitType,
    pub user_id: u64,
    pub writing_type: WritingType,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ProjectSessionData {
    pub count: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub feeling: Option<Feeling>,
    pub how: Option<How>,
    pub project_challenge_id: Option<u64>,
    pub project_id: Option<u64>,
    pub session_date: Option<NaiveDate>,
    pub start: Option<DateTime<Utc>>,
    pub unit_type: UnitType,
    pub r#where: Option<Where>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct StopWatchData {
    pub start: DateTime<Utc>,
    pub stop: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TimerData {
    pub cancelled: bool,
    #[serde(
        deserialize_with = "de_duration_mins",
        serialize_with = "se_duration_mins"
    )]
    pub duration: chrono::Duration,
    pub start: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct UserData {
    pub admin_level: AdminLevel,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub confirmed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub discourse_username: Option<String>,
    pub email: Option<String>,

    #[serde(flatten)]
    pub email_settings: Option<EmailSettings>,

    pub halo: bool,
    pub laurels: u64,
    pub location: Option<String>,
    pub name: String,

    #[serde(flatten)]
    pub notification_settings: Option<NotificationSettings>,

    pub notifications_viewed_at: DateTime<Utc>,
    pub plate: Option<String>,
    #[serde(deserialize_with = "de_opt_str_num")]
    pub postal_code: Option<u64>,

    #[serde(flatten)]
    pub privacy_settings: Option<PrivacySettings>,

    pub registration_path: RegistrationPath,
    pub setting_session_count_by_session: u8, // TODO: ???
    pub setting_session_more_info: bool,      // TODO: ???
    pub slug: String,

    #[serde(flatten)]
    pub stats: StatsInfo,

    pub time_zone: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EmailSettings {
    #[serde(rename = "email-blog-posts")]
    pub blog_posts: bool,
    #[serde(rename = "email-buddy-requests")]
    pub buddy_requests: bool,
    #[serde(rename = "email-events-in-home-region")]
    pub events_in_home_region: bool,
    #[serde(rename = "email-nanomessages-buddies")]
    pub nanomessages_buddies: bool,
    #[serde(rename = "email-nanomessages-hq")]
    pub nanomessages_hq: bool,
    #[serde(rename = "email-nanomessages-mls")]
    pub nanomessages_mls: bool,
    #[serde(rename = "email-nanowrimo-updates")]
    pub nanowrimo_updates: bool,
    #[serde(rename = "email-newsletter")]
    pub newsletter: bool,
    #[serde(rename = "email-writing-reminders")]
    pub writing_reminders: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NotificationSettings {
    #[serde(rename = "notification-buddy-activities")]
    pub buddy_activities: bool,
    #[serde(rename = "notification-buddy-requests")]
    pub buddy_requests: bool,
    #[serde(rename = "notification-events-in-home-region")]
    pub events_in_home_region: bool,
    #[serde(rename = "notification-goal-milestones")]
    pub goal_milestones: bool,
    #[serde(rename = "notification-nanomessages-buddies")]
    pub nanomessages_buddies: bool,
    #[serde(rename = "notification-nanomessages-hq")]
    pub nanomessages_hq: bool,
    #[serde(rename = "notification-nanomessages-mls")]
    pub nanomessages_mls: bool,
    #[serde(rename = "notification-new-badges")]
    pub new_badges: bool,
    #[serde(rename = "notification-sprint-invitation")]
    pub sprint_invitation: bool,
    #[serde(rename = "notification-sprint-start")]
    pub sprint_start: bool,
    #[serde(rename = "notification-writing-reminders")]
    pub writing_reminders: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PrivacySettings {
    #[serde(rename = "privacy-send-nanomessages")]
    pub send_nanomessages: PrivacySetting,
    #[serde(rename = "privacy-view-buddies")]
    pub view_buddies: PrivacySetting,
    #[serde(rename = "privacy-view-profile")]
    pub view_profile: PrivacySetting,
    #[serde(rename = "privacy-view-projects")]
    pub view_projects: PrivacySetting,
    #[serde(rename = "privacy-view-search")]
    pub view_search: PrivacySetting,
    #[serde(rename = "privacy-visibility-activity-logs")]
    pub visibility_activity_logs: bool,
    #[serde(rename = "privacy-visibility-buddy-lists")]
    pub visibility_buddy_lists: bool,
    #[serde(rename = "privacy-visibility-regions")]
    pub visibility_regions: bool,
}

// TODO: What do these *mean*, are all of them the right type?
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StatsInfo {
    #[serde(rename = "stats-projects")]
    pub projects: u64,
    #[serde(rename = "stats-projects-enabled")]
    pub projects_enabled: bool,
    #[serde(rename = "stats-streak")]
    pub streak: u64,
    #[serde(rename = "stats-streak-enabled")]
    pub streak_enabled: bool,
    #[serde(rename = "stats-word-count")]
    pub word_count: u64,
    #[serde(rename = "stats-word-count-enabled")]
    pub word_count_enabled: bool,
    #[serde(rename = "stats-wordiest")]
    pub wordiest: u64,
    #[serde(rename = "stats-wordiest-enabled")]
    pub wordiest_enabled: bool,
    #[serde(rename = "stats-writing-pace")]
    pub writing_pace: Option<u64>,
    #[serde(rename = "stats-writing-pace-enabled")]
    pub writing_pace_enabled: bool,
    #[serde(rename = "stats-years-done")]
    pub years_done: Option<u64>,
    #[serde(rename = "stats-years-enabled")]
    pub years_enabled: bool,
    #[serde(rename = "stats-years-won")]
    pub years_won: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WritingLocationData {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WritingMethodData {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GroupUserData {
    pub created_at: DateTime<Utc>,
    pub entry_at: Option<DateTime<Utc>>,
    pub entry_method: EntryMethod,
    pub exit_at: Option<DateTime<Utc>>,
    pub exit_method: Option<String>, // TODO: Enum
    pub group_code_id: Option<u64>,
    pub group_id: u64,
    pub group_type: GroupType,
    pub invitation_accepted: InvitationStatus,
    pub invited_by_id: Option<u64>,
    pub is_admin: Option<bool>,
    pub latest_message: Option<String>,
    pub num_unread_messages: u64,
    pub primary: u64,
    pub updated_at: DateTime<Utc>,
    pub user_id: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LocationGroupData {
    pub group_id: u64,
    pub location_id: u64,
    pub primary: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ProjectChallengeData {
    pub challenge_id: u64,
    pub current_count: u64,
    pub ends_at: NaiveDate,
    pub event_type: EventType,
    pub feeling: Option<Feeling>,
    pub goal: u64,
    pub how: Option<How>,
    pub last_recompute: Option<DateTime<Utc>>,
    pub name: String,
    pub project_id: u64,
    pub speed: Option<u64>, // TODO: ???
    pub start_count: Option<u64>,
    pub starts_at: NaiveDate,
    pub streak: Option<u64>,
    pub unit_type: UnitType,
    pub user_id: u64,
    pub when: Option<u64>, // TODO: ???
    pub won_at: Option<DateTime<Utc>>,
    pub writing_location: Option<String>, // TODO: ???
    pub writing_type: Option<WritingType>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct UserBadgeData {
    pub badge_id: u64,
    pub created_at: DateTime<Utc>,
    pub project_challenge_id: u64,
    pub user_id: u64,
}

// This doesn't like deny_unknown_fields, due to flatten
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RelationInfo {
    /// If this is Some, all references are included in the response Include array
    #[serde(
        flatten,
        deserialize_with = "de_rel_includes",
        serialize_with = "se_rel_includes"
    )]
    pub included: HashMap<NanoKind, Vec<ObjectRef>>,
    #[serde(
        flatten,
        deserialize_with = "de_relation",
        serialize_with = "se_relation"
    )]
    pub relations: HashMap<NanoKind, RelationLink>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RelationLink {
    #[serde(rename = "self")]
    pub this: String,
    pub related: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LinkInfo {
    #[serde(rename = "self")]
    pub this: String,
    #[serde(flatten)]
    pub others: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LinkData {
    #[serde(rename = "self")]
    pub this: String,

    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}
