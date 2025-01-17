use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum PrivacySetting {
    Private,
    Buddies,
    Anyone,
}

impl TryFrom<u8> for PrivacySetting {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<PrivacySetting, Self::Error> {
        match val {
            0 => Ok(PrivacySetting::Private),
            1 => Ok(PrivacySetting::Buddies),
            2 => Ok(PrivacySetting::Anyone),
            _ => Err("Cannot convert u8 into PrivacySetting"),
        }
    }
}

impl From<PrivacySetting> for u8 {
    fn from(val: PrivacySetting) -> Self {
        match val {
            PrivacySetting::Private => 0,
            PrivacySetting::Buddies => 1,
            PrivacySetting::Anyone => 2,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum ProjectStatus {
    Prepping,
    InProgress,
    Drafted,
    Completed,
    Published,
}

impl TryFrom<&str> for ProjectStatus {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<ProjectStatus, Self::Error> {
        match val.to_ascii_lowercase().as_str() {
            "prepping" => Ok(ProjectStatus::Prepping),
            "in progress" | "inprogress" => Ok(ProjectStatus::InProgress),
            "drafted" => Ok(ProjectStatus::Drafted),
            "completed" => Ok(ProjectStatus::Completed),
            "published" => Ok(ProjectStatus::Published),
            _ => Err("Cannot convert &str into ProjectStatus"),
        }
    }
}

impl From<ProjectStatus> for &'static str {
    fn from(val: ProjectStatus) -> Self {
        match val {
            ProjectStatus::Prepping => "Prepping",
            ProjectStatus::InProgress => "In Progress",
            ProjectStatus::Drafted => "Drafted",
            ProjectStatus::Completed => "Completed",
            ProjectStatus::Published => "Published",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum EventType {
    NanoWrimo,
    CampNano,
    Custom,
}

impl TryFrom<u8> for EventType {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<EventType, Self::Error> {
        match val {
            0 => Ok(EventType::NanoWrimo),
            1 => Ok(EventType::CampNano),
            2 => Ok(EventType::Custom),
            _ => Err("Cannot convert u8 into EventType"),
        }
    }
}

impl From<EventType> for u8 {
    fn from(val: EventType) -> Self {
        match val {
            EventType::NanoWrimo => 0,
            EventType::CampNano => 1,
            EventType::Custom => 2,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum GroupType {
    Everyone,
    Region,
    Buddies,
    WritingGroup,
    Event,
}

impl TryFrom<&str> for GroupType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<GroupType, Self::Error> {
        match val.to_ascii_lowercase().as_str() {
            "everyone" => Ok(GroupType::Everyone),
            "region" => Ok(GroupType::Region),
            "buddies" => Ok(GroupType::Buddies),
            "writing group" => Ok(GroupType::WritingGroup),
            "event" => Ok(GroupType::Event),
            _ => Err("Cannot convert &str into GroupType"),
        }
    }
}

impl From<GroupType> for &'static str {
    fn from(val: GroupType) -> Self {
        match val {
            GroupType::Everyone => "everyone",
            GroupType::Region => "region",
            GroupType::Buddies => "buddies",
            GroupType::WritingGroup => "writing group",
            GroupType::Event => "event",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum EntryMethod {
    Join,
    Creator,
    Create,
    Invited,
    Blocked,
}

impl TryFrom<&str> for EntryMethod {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<EntryMethod, Self::Error> {
        match val.to_ascii_lowercase().as_str() {
            "join" => Ok(EntryMethod::Join),
            "creator" => Ok(EntryMethod::Creator),
            "create" => Ok(EntryMethod::Create),
            "invited" => Ok(EntryMethod::Invited),
            "blocked" => Ok(EntryMethod::Blocked),
            _ => Err("Cannot convert &str into EntryMethod"),
        }
    }
}

impl From<EntryMethod> for &'static str {
    fn from(val: EntryMethod) -> Self {
        match val {
            EntryMethod::Join => "join",
            EntryMethod::Creator => "creator",
            EntryMethod::Create => "create",
            EntryMethod::Invited => "invited",
            EntryMethod::Blocked => "blocked",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum AdminLevel {
    User,
    Admin,
}

impl TryFrom<u8> for AdminLevel {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<AdminLevel, Self::Error> {
        match val {
            0 => Ok(AdminLevel::User),
            1 => Ok(AdminLevel::Admin),
            _ => Err("Cannot convert u8 into AdminLevel"),
        }
    }
}

impl From<AdminLevel> for u8 {
    fn from(val: AdminLevel) -> Self {
        match val {
            AdminLevel::User => 0,
            AdminLevel::Admin => 1,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum ActionType {
    BadgeAwarded,
    BuddiesPage,
    NanoMessages,
    ProjectsPage,
}

impl TryFrom<&str> for ActionType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<ActionType, Self::Error> {
        match val {
            "BADGE_AWARDED" => Ok(ActionType::BadgeAwarded),
            "BUDDIES_PAGE" => Ok(ActionType::BuddiesPage),
            "NANOMESSAGES" => Ok(ActionType::NanoMessages),
            "PROJECTS_PAGE" => Ok(ActionType::ProjectsPage),
            _ => Err("Cannot convert &str into ActionType"),
        }
    }
}

impl From<ActionType> for &'static str {
    fn from(val: ActionType) -> Self {
        match val {
            ActionType::BadgeAwarded => "BADGE_AWARDED",
            ActionType::BuddiesPage => "BUDDIES_PAGE",
            ActionType::NanoMessages => "NANOMESSAGES",
            ActionType::ProjectsPage => "PROJECTS_PAGE",
        }
    }
}

/// Whether to display the notification in the 'recent notifications'
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum DisplayStatus {
    AllNotifs,
    RecentNotifs,
}

impl TryFrom<u8> for DisplayStatus {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<DisplayStatus, Self::Error> {
        match val {
            0 => Ok(DisplayStatus::AllNotifs),
            1 => Ok(DisplayStatus::RecentNotifs),
            _ => Err("Cannot convert u8 into DisplayStatus"),
        }
    }
}

impl From<DisplayStatus> for u8 {
    fn from(val: DisplayStatus) -> Self {
        match val {
            DisplayStatus::AllNotifs => 0,
            DisplayStatus::RecentNotifs => 1,
        }
    }
}

// TODO: This may be wrong
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum WritingType {
    Novel,
    ShortStories,
    Memoir,
    Script,
    Nonfiction,
    Poetry,
    Other,
}

impl TryFrom<u8> for WritingType {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<WritingType, Self::Error> {
        match val {
            0 => Ok(WritingType::Novel),
            1 => Ok(WritingType::ShortStories),
            2 => Ok(WritingType::Memoir),
            3 => Ok(WritingType::Script),
            4 => Ok(WritingType::Nonfiction),
            5 => Ok(WritingType::Poetry),
            6 => Ok(WritingType::Other),
            _ => Err("Cannot convert u8 into WritingType"),
        }
    }
}

impl From<WritingType> for u8 {
    fn from(val: WritingType) -> Self {
        match val {
            WritingType::Novel => 0,
            WritingType::ShortStories => 1,
            WritingType::Memoir => 2,
            WritingType::Script => 3,
            WritingType::Nonfiction => 4,
            WritingType::Poetry => 5,
            WritingType::Other => 8,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum ContentType {
    GeneralContent,
    StackedContent,
    Plate,
    GroupOfPeople,
    GroupOfPageCards,
    PersonCard,
    PepTalk,
    PlainText,
}

impl TryFrom<&str> for ContentType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<ContentType, Self::Error> {
        match val {
            "General content" => Ok(ContentType::GeneralContent),
            "Stacked Content" => Ok(ContentType::StackedContent),
            "Plate" => Ok(ContentType::Plate),
            "Group of people" => Ok(ContentType::GroupOfPeople),
            "Group of page cards" => Ok(ContentType::GroupOfPageCards),
            "Person Card" => Ok(ContentType::PersonCard),
            "Pep Talk" => Ok(ContentType::PepTalk),
            "Plain Text" => Ok(ContentType::PlainText),
            _ => Err("Cannot convert &str into ContentType"),
        }
    }
}

impl From<ContentType> for &'static str {
    fn from(val: ContentType) -> Self {
        match val {
            ContentType::GeneralContent => "General content",
            ContentType::StackedContent => "Stacked Content",
            ContentType::Plate => "Plate",
            ContentType::GroupOfPeople => "Group of people",
            ContentType::GroupOfPageCards => "Group of page cards",
            ContentType::PersonCard => "Person Card",
            ContentType::PepTalk => "Pep Talk",
            ContentType::PlainText => "Plain Text",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum RegistrationPath {
    Email,
    Facebook,
    Google,
}

impl TryFrom<&str> for RegistrationPath {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<RegistrationPath, Self::Error> {
        match val.to_ascii_lowercase().as_str() {
            "email" => Ok(RegistrationPath::Email),
            "facebook" => Ok(RegistrationPath::Facebook),
            "google" => Ok(RegistrationPath::Google),
            _ => Err("Cannot convert &str into RegistrationPath"),
        }
    }
}

impl From<RegistrationPath> for &'static str {
    fn from(val: RegistrationPath) -> Self {
        match val {
            RegistrationPath::Email => "email",
            RegistrationPath::Facebook => "Facebook",
            RegistrationPath::Google => "Google",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum BadgeType {
    WordCount,
    SelfAwarded,
    Participation,
}

impl TryFrom<&str> for BadgeType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<BadgeType, Self::Error> {
        match val.to_ascii_lowercase().as_str() {
            "word count" => Ok(BadgeType::WordCount),
            "self-awarded" => Ok(BadgeType::SelfAwarded),
            "participation" => Ok(BadgeType::Participation),
            _ => Err("Cannot convert &str into BadgeType"),
        }
    }
}

impl From<BadgeType> for &'static str {
    fn from(val: BadgeType) -> Self {
        match val {
            BadgeType::WordCount => "word count",
            BadgeType::SelfAwarded => "self-awarded",
            BadgeType::Participation => "participation",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum JoiningRule {
    AdminOnly,
    AnyUser,
}

impl TryFrom<u8> for JoiningRule {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<JoiningRule, Self::Error> {
        match val {
            0 => Ok(JoiningRule::AdminOnly),
            1 => Ok(JoiningRule::AnyUser),
            _ => Err("Cannot convert u8 into JoiningRule"),
        }
    }
}

impl From<JoiningRule> for u8 {
    fn from(val: JoiningRule) -> Self {
        match val {
            JoiningRule::AdminOnly => 0,
            JoiningRule::AnyUser => 1,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone, Default)]
#[serde(try_from = "u8", into = "u8")]
pub enum UnitType {
    #[default]
    Words,
    Hours,
}

impl TryFrom<u8> for UnitType {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<UnitType, Self::Error> {
        match val {
            0 => Ok(UnitType::Words),
            1 => Ok(UnitType::Hours),
            _ => Err("Cannot convert u8 into UnitType"),
        }
    }
}

impl From<UnitType> for u8 {
    fn from(val: UnitType) -> Self {
        match val {
            UnitType::Words => 0,
            UnitType::Hours => 1,
        }
    }
}

// This may someday be replaced with NanoKind
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum AdheresTo {
    Unknown,
    User,
    ProjectChallenge,
}

impl TryFrom<&str> for AdheresTo {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<AdheresTo, Self::Error> {
        match val {
            "" => Ok(AdheresTo::Unknown),
            "user" => Ok(AdheresTo::User),
            "project_challenge" => Ok(AdheresTo::ProjectChallenge),
            _ => Err("Cannot convert &str into AdheresTo"),
        }
    }
}

impl From<AdheresTo> for &'static str {
    fn from(val: AdheresTo) -> Self {
        match val {
            AdheresTo::Unknown => "",
            AdheresTo::User => "user",
            AdheresTo::ProjectChallenge => "project_challenge",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum Feeling {
    Upset,
    Stressed,
    Okay,
    PrettyGood,
    Great,
}

impl TryFrom<u8> for Feeling {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Feeling, Self::Error> {
        match val {
            1 => Ok(Feeling::Upset),
            2 => Ok(Feeling::Stressed),
            3 => Ok(Feeling::Okay),
            4 => Ok(Feeling::PrettyGood),
            5 => Ok(Feeling::Great),
            _ => Err("Cannot convert u8 into Feeling"),
        }
    }
}

impl From<Feeling> for u8 {
    fn from(val: Feeling) -> Self {
        match val {
            Feeling::Upset => 1,
            Feeling::Stressed => 2,
            Feeling::Okay => 3,
            Feeling::PrettyGood => 4,
            Feeling::Great => 5,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(from = "u8", into = "u8")]
pub enum Where {
    Home,
    Office,
    Library,
    Cafe,
    Other(u8),
}

impl From<u8> for Where {
    fn from(val: u8) -> Where {
        match val {
            0 => Where::Home,
            1 => Where::Office,
            2 => Where::Library,
            3 => Where::Cafe,
            _ => Where::Other(val),
        }
    }
}

impl Into<u8> for Where {
    fn into(self) -> u8 {
        match self {
            Where::Home => 0,
            Where::Office => 1,
            Where::Library => 2,
            Where::Cafe => 3,
            Where::Other(val) => val,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(from = "u64", into = "u64")]
pub enum How {
    ByHand,
    Typewriter,
    Laptop,
    Phone,
    Other(u64),
}

impl From<u64> for How {
    fn from(val: u64) -> How {
        match val {
            0 => How::ByHand,
            1 => How::Typewriter,
            2 => How::Laptop,
            3 => How::Phone,
            _ => How::Other(val),
        }
    }
}

impl Into<u64> for How {
    fn into(self) -> u64 {
        match self {
            How::ByHand => 0,
            How::Typewriter => 1,
            How::Laptop => 2,
            How::Phone => 3,
            How::Other(val) => val,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "i8", into = "i8")]
pub enum InvitationStatus {
    Blocked,
    Sent,
    Accepted,
}

impl TryFrom<i8> for InvitationStatus {
    type Error = &'static str;

    fn try_from(val: i8) -> Result<InvitationStatus, Self::Error> {
        match val {
            -2 => Ok(InvitationStatus::Blocked),
            0 => Ok(InvitationStatus::Sent),
            1 => Ok(InvitationStatus::Accepted),
            _ => Err("Cannot convert i8 into InvitationStatus"),
        }
    }
}

impl From<InvitationStatus> for i8 {
    fn from(val: InvitationStatus) -> Self {
        match val {
            InvitationStatus::Blocked => -2,
            InvitationStatus::Sent => 0,
            InvitationStatus::Accepted => 1,
        }
    }
}
