use reqwest::Method;
use serde::Serialize;

use crate::auth::Authentication;
use crate::{
    AddedWebhooks, ApiResponse, AppAuth, BearerAuth, CategoryList, CommentList, CommentListRequest,
    CommentText, DeletedComment, DeletedWebhooks, Error, GiftList, GiftRequest, Hashtag,
    HashtagUpdate, Language, LiveSchedules, LiveSearchRequest, LiveSearchResults, MovieId,
    MovieInfo, MovieList, MovieListRequest, PostedComment, RtmpCredentials, Subtitle,
    SubtitleUpdate, SupportBatch, SupportResult, SupporterList, SupporterListRequest,
    SupportingList, SupportingStatus, Thumbnail, ThumbnailOptions, UnsupportResult,
    UpcomingSchedulesRequest, UserId, UserInfo, UserRef, UserSearchRequest, UserSearchResults,
    VerifiedCredentials, WebhookEvents, WebhookList, WebhookListRequest,
};

macro_rules! resource {
    ($name:ident) => {
        /// Resource-scoped API methods.
        pub struct $name<'a, A> {
            client: &'a crate::Client<A>,
        }

        impl<'a, A> $name<'a, A> {
            pub(crate) const fn new(client: &'a crate::Client<A>) -> Self {
                Self { client }
            }
        }
    };
}

resource!(Users);
resource!(Movies);
resource!(Comments);
resource!(Gifts);
resource!(Supporters);
resource!(Categories);
resource!(Search);
resource!(Webhooks);
resource!(Broadcasting);

impl<A: Authentication> Users<'_, A> {
    /// Gets user information.
    pub async fn get(&self, user: &UserRef) -> Result<ApiResponse<UserInfo>, Error> {
        let request = self.client.request(Method::GET, &["users", user.as_str()]);
        self.client.send_json(request).await
    }

    /// Gets upcoming live schedules.
    pub async fn upcoming_live_schedules(
        &self,
        user: &UserRef,
        options: &UpcomingSchedulesRequest,
    ) -> Result<ApiResponse<LiveSchedules>, Error> {
        let request = self
            .client
            .request(
                Method::GET,
                &["users", user.as_str(), "upcoming_live_schedules"],
            )
            .query(options);
        self.client.send_json(request).await
    }

    /// Downloads the live thumbnail without requiring authentication.
    pub async fn live_thumbnail(
        &self,
        user: &UserRef,
        options: ThumbnailOptions,
    ) -> Result<ApiResponse<Thumbnail>, Error> {
        let request = self
            .client
            .unauthenticated_request(Method::GET, &["users", user.as_str(), "live", "thumbnail"])
            .query(&options);
        self.client.send_thumbnail(request).await
    }
}

impl Users<'_, BearerAuth> {
    /// Verifies the bearer token and returns its user and application.
    ///
    /// This operation is unavailable to application-authenticated clients:
    ///
    /// ```compile_fail
    /// use twitcasting::Client;
    ///
    /// let client = Client::application("client-id", "client-secret")?;
    /// client.users().verify_credentials();
    /// # Ok::<(), twitcasting::Error>(())
    /// ```
    pub async fn verify_credentials(&self) -> Result<ApiResponse<VerifiedCredentials>, Error> {
        let request = self.client.request(Method::GET, &["verify_credentials"]);
        self.client.send_json(request).await
    }
}

impl<A: Authentication> Movies<'_, A> {
    /// Gets an expanded movie.
    pub async fn get(&self, movie_id: &MovieId) -> Result<ApiResponse<MovieInfo>, Error> {
        let request = self
            .client
            .request(Method::GET, &["movies", movie_id.as_str()]);
        self.client.send_json(request).await
    }

    /// Lists movies owned by a user.
    pub async fn by_user(
        &self,
        user: &UserRef,
        options: &MovieListRequest,
    ) -> Result<ApiResponse<MovieList>, Error> {
        let request = self
            .client
            .request(Method::GET, &["users", user.as_str(), "movies"])
            .query(options);
        self.client.send_json(request).await
    }

    /// Gets the user's current live movie.
    pub async fn current_live(&self, user: &UserRef) -> Result<ApiResponse<MovieInfo>, Error> {
        let request = self
            .client
            .request(Method::GET, &["users", user.as_str(), "current_live"]);
        self.client.send_json(request).await
    }
}

impl Movies<'_, BearerAuth> {
    /// Sets the current live subtitle.
    pub async fn set_subtitle(
        &self,
        subtitle: &Subtitle,
    ) -> Result<ApiResponse<SubtitleUpdate>, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            subtitle: &'a str,
        }
        let request = self
            .client
            .request(Method::POST, &["movies", "subtitle"])
            .json(&Body {
                subtitle: subtitle.as_str(),
            });
        self.client.send_json(request).await
    }

    /// Removes the current live subtitle.
    pub async fn unset_subtitle(&self) -> Result<ApiResponse<SubtitleUpdate>, Error> {
        let request = self.client.request(Method::DELETE, &["movies", "subtitle"]);
        self.client.send_json(request).await
    }

    /// Sets the current live hashtag.
    pub async fn set_hashtag(
        &self,
        hashtag: &Hashtag,
    ) -> Result<ApiResponse<HashtagUpdate>, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            hashtag: &'a str,
        }
        let request = self
            .client
            .request(Method::POST, &["movies", "hashtag"])
            .json(&Body {
                hashtag: hashtag.as_str(),
            });
        self.client.send_json(request).await
    }

    /// Removes the current live hashtag.
    pub async fn unset_hashtag(&self) -> Result<ApiResponse<HashtagUpdate>, Error> {
        let request = self.client.request(Method::DELETE, &["movies", "hashtag"]);
        self.client.send_json(request).await
    }
}

impl<A: Authentication> Comments<'_, A> {
    /// Lists comments on a movie.
    pub async fn list(
        &self,
        movie_id: &MovieId,
        options: &CommentListRequest,
    ) -> Result<ApiResponse<CommentList>, Error> {
        let request = self
            .client
            .request(Method::GET, &["movies", movie_id.as_str(), "comments"])
            .query(options);
        self.client.send_json(request).await
    }
}

impl Comments<'_, BearerAuth> {
    /// Posts a comment.
    pub async fn post(
        &self,
        movie_id: &MovieId,
        comment: &CommentText,
    ) -> Result<ApiResponse<PostedComment>, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            comment: &'a str,
        }
        let request = self
            .client
            .request(Method::POST, &["movies", movie_id.as_str(), "comments"])
            .json(&Body {
                comment: comment.as_str(),
            });
        self.client.send_json(request).await
    }

    /// Deletes a comment.
    pub async fn delete(
        &self,
        movie_id: &MovieId,
        comment_id: &crate::CommentId,
    ) -> Result<ApiResponse<DeletedComment>, Error> {
        let request = self.client.request(
            Method::DELETE,
            &["movies", movie_id.as_str(), "comments", comment_id.as_str()],
        );
        self.client.send_json(request).await
    }
}

impl Gifts<'_, BearerAuth> {
    /// Polls recently received gifts.
    pub async fn list(&self, options: GiftRequest) -> Result<ApiResponse<GiftList>, Error> {
        let request = self.client.request(Method::GET, &["gifts"]).query(&options);
        self.client.send_json(request).await
    }
}

impl<A: Authentication> Supporters<'_, A> {
    /// Gets whether one user supports another.
    pub async fn status(
        &self,
        user: &UserRef,
        target: &UserRef,
    ) -> Result<ApiResponse<SupportingStatus>, Error> {
        let request = self
            .client
            .request(Method::GET, &["users", user.as_str(), "supporting_status"])
            .query(&[("target_user_id", target.as_str())]);
        self.client.send_json(request).await
    }

    /// Lists users supported by a user.
    pub async fn supporting(
        &self,
        user: &UserRef,
        options: &SupporterListRequest,
    ) -> Result<ApiResponse<SupportingList>, Error> {
        let request = self
            .client
            .request(Method::GET, &["users", user.as_str(), "supporting"])
            .query(options);
        self.client.send_json(request).await
    }

    /// Lists a user's supporters.
    pub async fn supporters(
        &self,
        user: &UserRef,
        options: &SupporterListRequest,
    ) -> Result<ApiResponse<SupporterList>, Error> {
        let request = self
            .client
            .request(Method::GET, &["users", user.as_str(), "supporters"])
            .query(options);
        self.client.send_json(request).await
    }
}

impl Supporters<'_, BearerAuth> {
    /// Supports one or more users.
    pub async fn support(&self, users: &SupportBatch) -> Result<ApiResponse<SupportResult>, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            target_user_ids: Vec<&'a str>,
        }
        let request = self.client.request(Method::PUT, &["support"]).json(&Body {
            target_user_ids: users.wire_values(),
        });
        self.client.send_json(request).await
    }

    /// Stops supporting one or more users.
    pub async fn unsupport(
        &self,
        users: &SupportBatch,
    ) -> Result<ApiResponse<UnsupportResult>, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            target_user_ids: Vec<&'a str>,
        }
        let request = self
            .client
            .request(Method::PUT, &["unsupport"])
            .json(&Body {
                target_user_ids: users.wire_values(),
            });
        self.client.send_json(request).await
    }
}

impl<A: Authentication> Categories<'_, A> {
    /// Lists live categories in the requested language.
    pub async fn list(&self, language: Language) -> Result<ApiResponse<CategoryList>, Error> {
        let request = self
            .client
            .request(Method::GET, &["categories"])
            .query(&[("lang", language)]);
        self.client.send_json(request).await
    }
}

impl<A: Authentication> Search<'_, A> {
    /// Searches users.
    pub async fn users(
        &self,
        options: &UserSearchRequest,
    ) -> Result<ApiResponse<UserSearchResults>, Error> {
        let request = self
            .client
            .request(Method::GET, &["search", "users"])
            .query(options);
        self.client.send_json(request).await
    }

    /// Searches live movies.
    pub async fn lives(
        &self,
        options: &LiveSearchRequest,
    ) -> Result<ApiResponse<LiveSearchResults>, Error> {
        let request = self
            .client
            .request(Method::GET, &["search", "lives"])
            .query(options);
        self.client.send_json(request).await
    }
}

impl Webhooks<'_, AppAuth> {
    /// Lists application webhook registrations.
    ///
    /// This operation is unavailable to bearer-authenticated clients:
    ///
    /// ```compile_fail
    /// use twitcasting::{Client, WebhookListRequest};
    ///
    /// let client = Client::bearer("token")?;
    /// client.webhooks().list(&WebhookListRequest::default());
    /// # Ok::<(), twitcasting::Error>(())
    /// ```
    pub async fn list(
        &self,
        options: &WebhookListRequest,
    ) -> Result<ApiResponse<WebhookList>, Error> {
        let request = self
            .client
            .request(Method::GET, &["webhooks"])
            .query(options);
        self.client.send_json(request).await
    }

    /// Registers application webhooks.
    pub async fn register(
        &self,
        user_id: &UserId,
        events: &WebhookEvents,
    ) -> Result<ApiResponse<AddedWebhooks>, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            user_id: &'a UserId,
            events: &'a [crate::WebhookEvent],
        }
        let request = self
            .client
            .request(Method::POST, &["webhooks"])
            .json(&Body {
                user_id,
                events: events.as_slice(),
            });
        self.client.send_json(request).await
    }

    /// Removes application webhooks.
    pub async fn remove(
        &self,
        user_id: &UserId,
        events: &WebhookEvents,
    ) -> Result<ApiResponse<DeletedWebhooks>, Error> {
        let mut query = vec![("user_id", user_id.as_str())];
        for event in events.as_slice() {
            query.push(("events[]", event.as_str()));
        }
        let request = self
            .client
            .request(Method::DELETE, &["webhooks"])
            .query(&query);
        self.client.send_json(request).await
    }
}

impl Broadcasting<'_, BearerAuth> {
    /// Gets RTMP publishing credentials.
    pub async fn rtmp_credentials(&self) -> Result<ApiResponse<RtmpCredentials>, Error> {
        let request = self.client.request(Method::GET, &["rtmp_url"]);
        self.client.send_json(request).await
    }
}
