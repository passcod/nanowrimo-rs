use super::data::*;
use super::error::Error;
use super::kind::NanoKind;

use std::collections::HashMap;
use std::sync::Arc;

use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::{error, trace};

#[cfg(test)]
mod tests;

fn add_included(data: &mut Vec<(String, String)>, include: &[NanoKind]) {
    if !include.is_empty() {
        data.push((
            "include".to_string(),
            include
                .iter()
                .map(|kind| kind.api_name())
                .collect::<Vec<&str>>()
                .join(","),
        ))
    }
}

#[derive(Clone, Debug)]
struct Creds {
    username: String,
    password: String,
}

/// A client with which to connect to the Nano site. Can be used with or without login.
#[derive(Clone, Debug)]
pub struct NanoClient {
    client: Client,
    creds: Option<Arc<Creds>>,
    token: Arc<RwLock<Option<String>>>,
}

impl NanoClient {
    const BASE_URL: &'static str = "https://api.nanowrimo.org/";

    fn new(user: &str, pass: &str) -> NanoClient {
        NanoClient {
            client: Client::new(),
            creds: Some(Arc::new(Creds {
                username: user.into(),
                password: pass.into(),
            })),
            token: Default::default(),
        }
    }

    /// Create a new client with the 'anonymous' or 'guest' user, not logged in
    pub fn new_anon() -> NanoClient {
        NanoClient {
            client: Client::new(),
            creds: None,
            token: Default::default(),
        }
    }

    /// Create a new client that is automatically logged in as a specific user
    pub async fn new_user(user: &str, pass: &str) -> Result<NanoClient, Error> {
        let client = NanoClient::new(user, pass);
        client.login().await?;
        Ok(client)
    }

    async fn make_request<T, U>(&self, path: &str, method: Method, data: &T) -> Result<U, Error>
    where
        T: Serialize + ?Sized + std::fmt::Debug,
        U: DeserializeOwned + std::fmt::Debug,
    {
        trace!(?path, "preparing request to nanowrimo.org");

        let mut query = None;
        let mut json = None;

        match method {
            Method::GET => query = Some(data),
            _ => json = Some(data),
        }

        let mut req = self
            .client
            .request(method, format!("{}{}", NanoClient::BASE_URL, path));

        if let Some(token) = self.token.read().await.as_deref() {
            req = req.header("Authorization", token)
        }

        if let Some(query) = query {
            trace!(?query, "query request to nanowrimo.org");
            req = req.query(query);
        }

        if let Some(json) = json {
            req = req.header(reqwest::header::CONTENT_TYPE, "application/vnd.api+json");
            trace!(
                ?json,
                actual = %serde_json::to_string(&json).unwrap_or("unable to render JSON".into()),
                "json request to nanowrimo.org"
            );
            req = req.json(json);
        }

        let resp = req.send().await?;

        let status = resp.status();

        match status {
            StatusCode::INTERNAL_SERVER_ERROR => {
                return Err(Error::SimpleNanoError(
                    status,
                    "Internal Server Error".to_string(),
                ))
            }
            StatusCode::NOT_FOUND => {
                return Err(Error::SimpleNanoError(status, "Page Not Found".to_string()))
            }
            _ => (),
        }

        let nano_resp = resp.text().await?;
        trace!(?nano_resp, "response from nanowrimo.org");

        let nano_val: serde_json::Value = serde_json::from_str(&nano_resp).unwrap_or_default();
        if nano_val.as_object().map_or(false, |obj| {
            obj.contains_key("error") || obj.contains_key("errors")
        }) {
            // parse the error(s)
            let nano_error: NanoError = serde_json::from_value(nano_val)?;
            return match nano_error {
                NanoError::SimpleError { error } => Err(Error::SimpleNanoError(status, error)),
                NanoError::ErrorList { errors } => Err(Error::NanoErrors(errors)),
            };
        }

        let jd = &mut serde_json::Deserializer::from_str(&nano_resp);
        let nano_resp = serde_path_to_error::deserialize(jd).map_err(|err| {
            let path = err.path().to_string();
            let err = err.into_inner();
            error!(%path, %err, raw=%nano_val, "error parsing nanowrimo.org response as json");
            Error::ResponseDecoding { path, err }
        })?;
        trace!(?nano_resp, "response from nanowrimo.org");

        Ok(nano_resp)
    }

    async fn retry_request<T, U>(&self, path: &str, method: Method, data: &T) -> Result<U, Error>
    where
        T: Serialize + ?Sized + std::fmt::Debug,
        U: DeserializeOwned + std::fmt::Debug,
    {
        let res = self.make_request(path, method.clone(), data).await;

        match res {
            Err(Error::SimpleNanoError(code, _))
                if code == StatusCode::UNAUTHORIZED && self.is_logged_in().await =>
            {
                self.login().await?;
                self.make_request(path, method, data).await
            }
            _ => res,
        }
    }

    /// Check whether this client is currently logged in
    pub async fn is_logged_in(&self) -> bool {
        self.token.read().await.is_some()
    }

    /// Log in this client, without logging out
    pub async fn login(&self) -> Result<(), Error> {
        let Some(ref creds) = self.creds else {
            return Err(Error::NoCredentials);
        };

        let mut map = HashMap::new();
        map.insert("identifier", &creds.username);
        map.insert("password", &creds.password);

        let res = self
            .make_request::<_, LoginResponse>("users/sign_in", Method::POST, &map)
            .await?;

        self.token.write().await.replace(res.auth_token);

        Ok(())
    }

    /// Log out this client, without checking if it's logged in
    pub async fn logout(&self) -> Result<(), Error> {
        self.make_request::<_, ()>("users/logout", Method::POST, &())
            .await?;
        self.token.write().await.take();

        Ok(())
    }

    // Commands

    /// Get information about the Nano fundometer
    pub async fn fundometer(&self) -> Result<Fundometer, Error> {
        self.retry_request("fundometer", Method::GET, &()).await
    }

    /// Search for users by username
    pub async fn search(&self, name: &str) -> Result<CollectionResponse<UserObject>, Error> {
        self.retry_request("search", Method::GET, &[("q", name)])
            .await
    }

    /// Get a random sponsor offer
    pub async fn random_offer(&self) -> Result<ItemResponse<PostObject>, Error> {
        self.retry_request("random_offer", Method::GET, &()).await
    }

    /// Get a list of all store items
    pub async fn store_items(&self) -> Result<Vec<StoreItem>, Error> {
        self.retry_request("store_items", Method::GET, &()).await
    }

    /// Get a list of all current sponsor offers
    pub async fn offers(&self) -> Result<Vec<ItemResponse<PostObject>>, Error> {
        self.retry_request("offers", Method::GET, &()).await
    }

    /// Get the currently logged in user, with included linked items
    pub async fn current_user_include(
        &self,
        include: &[NanoKind],
    ) -> Result<ItemResponse<UserObject>, Error> {
        let mut data = Vec::new();

        add_included(&mut data, include);

        self.retry_request("users/current", Method::GET, &data)
            .await
    }

    /// Get the currently logged in user
    pub async fn current_user(&self) -> Result<ItemResponse<UserObject>, Error> {
        self.current_user_include(&[]).await
    }

    /// Get info about a specific set of pages. Known valid values include:
    ///
    /// - `"what-is-camp-nanowrimo"`
    /// - `"nano-prep-101"`
    /// - `"pep-talks"`
    /// - `"dei"`
    /// - `"come-write-in"`
    /// - `"about-nano"`
    /// - `"staff"`
    /// - `"board-of-directors"`
    /// - `"writers-board"`
    /// - `"terms-and-conditions"`
    /// - `"writers-board"`
    /// - `"brought-to-you-by"`
    ///
    /// If you know of other valid values, please open an issue with the values to add to this list!
    pub async fn pages(&self, page: &str) -> Result<ItemResponse<PageObject>, Error> {
        self.retry_request(&format!("pages/{}", page), Method::GET, &())
            .await
    }

    /// Get the list of notifications for the current user
    pub async fn notifications(&self) -> Result<CollectionResponse<NotificationObject>, Error> {
        self.retry_request("notifications", Method::GET, &()).await
    }

    /// Get a set of all the challenges this user has access to (Possibly all they can make
    /// projects in)
    pub async fn available_challenges(&self) -> Result<CollectionResponse<ChallengeObject>, Error> {
        self.retry_request("challenges/available", Method::GET, &())
            .await
    }

    /// Get the daily aggregates for a given ProjectChallenge
    /// ProjectChallenge is the common link between a project and a challenge it was part of,
    /// thus providing info for counts on given days
    pub async fn daily_aggregates(
        &self,
        id: u64,
    ) -> Result<CollectionResponse<DailyAggregateObject>, Error> {
        self.retry_request(
            &format!("project-challenges/{}/daily-aggregates", id),
            Method::GET,
            &(),
        )
        .await
    }

    // Type queries

    /// Get all accessible items of a specific kind, with included linked items and filtering to
    /// certain related IDs.
    ///
    /// 'includes' will add more items in the response as part of an 'includes' list,
    /// so one request can get more items
    ///
    /// 'filter' will filter certain types of objects by IDs of other objects related to them.
    ///
    /// **Warning**: Many filter combinations are invalid, and the rules are not currently fully
    /// understood.
    pub async fn get_all_include_filtered<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
        include: &[NanoKind],
        filter: &[(&str, u64)],
    ) -> Result<CollectionResponse<D>, Error> {
        let mut data = Vec::new();

        for i in filter {
            data.push((format!("filter[{}]", i.0), i.1.to_string()))
        }

        add_included(&mut data, include);

        self.retry_request(ty.api_name(), Method::GET, &data).await
    }

    /// Get all accessible items of a specific kind, with filtering to certain related IDs
    /// (See [`Self::get_all_include_filtered`])
    pub async fn get_all_filtered<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
        filter: &[(&str, u64)],
    ) -> Result<CollectionResponse<D>, Error> {
        self.get_all_include_filtered(ty, &[], filter).await
    }

    /// Get all accessible items of a specific kind, with included linked items
    /// (See [`Self::get_all_include_filtered`])
    pub async fn get_all_include<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
        include: &[NanoKind],
    ) -> Result<CollectionResponse<D>, Error> {
        self.get_all_include_filtered(ty, include, &[]).await
    }

    /// Get all accessible items of a specific kind, neither filtering nor including linked items
    /// (See [`Self::get_all_include_filtered`])
    pub async fn get_all<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
    ) -> Result<CollectionResponse<D>, Error> {
        self.get_all_include_filtered(ty, &[], &[]).await
    }

    /// Get an item of a specific type and ID, with included linked items
    pub async fn get_id_include<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
        id: u64,
        include: &[NanoKind],
    ) -> Result<ItemResponse<D>, Error> {
        let mut data = Vec::new();

        add_included(&mut data, include);

        self.retry_request(&format!("{}/{}", ty.api_name(), id), Method::GET, &data)
            .await
    }

    /// Get an item of a specific type and ID, with no included items.
    /// (See [`Self::get_id_include`])
    pub async fn get_id<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
        id: u64,
    ) -> Result<ItemResponse<D>, Error> {
        self.get_id_include(ty, id, &[]).await
    }

    /// Get an item of a specific type and slug, with included items.
    /// A slug is a unique text identifier for an object, not all types have one.
    pub async fn get_slug_include<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
        slug: &str,
        include: &[NanoKind],
    ) -> Result<ItemResponse<D>, Error> {
        let mut data = Vec::new();

        add_included(&mut data, include);

        self.retry_request(&format!("{}/{}", ty.api_name(), slug), Method::GET, &data)
            .await
    }

    /// Get an item of a specific type and slug, with no included items.
    /// A slug is a unique text identifier for an object, not all types have one.
    pub async fn get_slug<D: ObjectInfo + DeserializeOwned>(
        &self,
        ty: NanoKind,
        slug: &str,
    ) -> Result<ItemResponse<D>, Error> {
        self.get_slug_include(ty, slug, &[]).await
    }

    /// Get all items from a given RelationLink, a tie from one object to object(s) of a specific
    /// type that are related to it.
    ///
    /// **Warning**: Not all RelationLinks can be retrieved, some will return a 404 due to the
    /// way Nano handle them on its end, if you know ahead of time that you will need the relations,
    /// it's better to use [`Self::get_id_include`] or [`Self::get_all_include`]
    pub async fn get_all_related(&self, rel: &RelationLink) -> Result<CollectionResponse, Error> {
        if !rel.related.ends_with('s') {
            panic!("get_all_related can only get many-relation links")
        }

        self.retry_request(&rel.related, Method::GET, &()).await
    }

    /// Get a single item from a given RelationLink, a tie from one object to object(s) of a
    /// specific type that are related to it. Single relations tend to not have the same pitfalls as
    /// multiple relations, so this is less dangerous than [`Self::get_all_related`]
    pub async fn get_unique_related(&self, rel: &RelationLink) -> Result<ItemResponse, Error> {
        if rel.related.ends_with('s') {
            panic!("get_unique_related can only get single-relation links")
        }

        self.retry_request(&rel.related, Method::GET, &()).await
    }

    /// Update wordcount
    ///
    /// You'll need to retrieve the current count for the project challenge, compute the
    /// difference, and call this with it. Alternatively if you've got the session's count you can
    /// update with that directly.
    ///
    /// Returns the saved project session.
    pub async fn add_project_session(
        &self,
        project_id: u64,
        project_challenge_id: u64,
        words: i64,
    ) -> Result<ItemResponse<ProjectSessionObject>, Error> {
        if !self.is_logged_in().await {
            return Err(Error::NoCredentials);
        };

        let data = ItemResponse {
            data: Object::ProjectSession(ProjectSessionObject {
                id: 0,
                links: None,
                attributes: ProjectSessionData {
                    count: words,
                    ..Default::default()
                },
                relationships: Some(RelationInfo {
                    relations: Default::default(),
                    included: vec![
                        (
                            NanoKind::Project,
                            vec![ObjectRef {
                                id: project_id,
                                kind: NanoKind::Project,
                            }],
                        ),
                        (
                            NanoKind::ProjectChallenge,
                            vec![ObjectRef {
                                id: project_challenge_id,
                                kind: NanoKind::ProjectChallenge,
                            }],
                        ),
                    ]
                    .into_iter()
                    .collect(),
                }),
            }),
            included: None,
            post_info: None,
        };

        self.retry_request("project-sessions", Method::POST, &data)
            .await
    }
}
