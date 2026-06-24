//! End-to-end request construction tests against a local HTTP server.

use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use twitcasting::{
    ApiResponse, AppAuth, BearerAuth, Client, ClientBuilder, CommentId, CommentListRequest,
    CommentText, Error, GiftRequest, Hashtag, Language, LiveSearchKind, LiveSearchRequest, MovieId,
    MovieListRequest, OAuthClient, ScreenId, SearchTerms, Subtitle, SupportBatch,
    SupporterListRequest, SupporterSort, ThumbnailOptions, UpcomingSchedulesRequest, UserId,
    UserRef, UserSearchRequest, WebhookEvent, WebhookEvents, WebhookListRequest,
};
use url::Url;

struct Server {
    base_url: Url,
    requests: Arc<Mutex<Vec<String>>>,
}

async fn spawn_server() -> Server {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&requests);

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let recorded = Arc::clone(&recorded);
            tokio::spawn(async move {
                let mut request = Vec::new();
                let mut buffer = [0_u8; 4096];
                let header_end = loop {
                    let count = stream.read(&mut buffer).await.unwrap();
                    if count == 0 {
                        return;
                    }
                    request.extend_from_slice(&buffer[..count]);
                    if let Some(position) =
                        request.windows(4).position(|value| value == b"\r\n\r\n")
                    {
                        break position + 4;
                    }
                };
                let headers = String::from_utf8_lossy(&request[..header_end]);
                let content_length = headers
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(str::trim)
                            .and_then(|value| value.parse::<usize>().ok())
                    })
                    .unwrap_or(0);
                while request.len() < header_end + content_length {
                    let count = stream.read(&mut buffer).await.unwrap();
                    if count == 0 {
                        break;
                    }
                    request.extend_from_slice(&buffer[..count]);
                }

                let text = String::from_utf8_lossy(&request).into_owned();
                let is_thumbnail = text
                    .lines()
                    .next()
                    .is_some_and(|line| line.contains("/live/thumbnail"));
                recorded.lock().unwrap().push(text);

                let response = if is_thumbnail {
                    "HTTP/1.1 200 OK\r\nContent-Type: image/jpeg\r\nContent-Length: 4\r\nConnection: close\r\n\r\nJPEG".to_owned()
                } else {
                    let body = r#"{"error":{"code":1001,"message":"Validation error","details":{"limit":["max"]}}}"#;
                    format!(
                        "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nX-RateLimit-Limit: 60\r\nX-RateLimit-Remaining: 59\r\nX-RateLimit-Reset: 1767193200\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                };
                stream.write_all(response.as_bytes()).await.unwrap();
            });
        }
    });

    Server {
        base_url: Url::parse(&format!("http://{address}/")).unwrap(),
        requests,
    }
}

fn assert_api_error<T: std::fmt::Debug>(result: Result<ApiResponse<T>, Error>) {
    let Error::Api(error) = result.unwrap_err() else {
        panic!("expected structured API error");
    };
    assert_eq!(error.code, 1001);
    assert_eq!(error.details.unwrap()["limit"], ["max"]);
    assert_eq!(error.rate_limit.unwrap().remaining, 59);
}

#[tokio::test]
async fn covers_every_endpoint_family_and_auth_shape() {
    let server = spawn_server().await;
    let bearer = ClientBuilder::new(BearerAuth::new("bearer-token"))
        .unwrap()
        .base_url(server.base_url.clone())
        .build()
        .unwrap();
    let app = ClientBuilder::new(AppAuth::new("client-id", "client-secret"))
        .unwrap()
        .base_url(server.base_url.clone())
        .build()
        .unwrap();

    let user = UserRef::from(ScreenId::new("caster/name"));
    let target = UserRef::from(UserId::new("42"));
    let movie = MovieId::new("100");
    let comment = CommentId::new("200");

    assert_api_error(bearer.users().get(&user).await);
    assert_api_error(
        bearer
            .users()
            .upcoming_live_schedules(&user, &UpcomingSchedulesRequest::default().in_days(30))
            .await,
    );
    let thumbnail = bearer
        .users()
        .live_thumbnail(&user, ThumbnailOptions::default())
        .await
        .unwrap();
    assert_eq!(&thumbnail.value.bytes[..], b"JPEG");
    assert_api_error(bearer.users().verify_credentials().await);

    assert_api_error(bearer.movies().get(&movie).await);
    assert_api_error(
        bearer
            .movies()
            .by_user(&user, &MovieListRequest::default().offset(10).limit(20))
            .await,
    );
    assert_api_error(bearer.movies().current_live(&user).await);
    assert_api_error(
        bearer
            .movies()
            .set_subtitle(&Subtitle::new("subtitle"))
            .await,
    );
    assert_api_error(bearer.movies().unset_subtitle().await);
    assert_api_error(bearer.movies().set_hashtag(&Hashtag::new("#stream")).await);
    assert_api_error(bearer.movies().unset_hashtag().await);

    assert_api_error(
        bearer
            .comments()
            .list(&movie, &CommentListRequest::default().limit(10))
            .await,
    );
    assert_api_error(
        bearer
            .comments()
            .post(&movie, &CommentText::new("hello"))
            .await,
    );
    assert_api_error(bearer.comments().delete(&movie, &comment).await);
    assert_api_error(
        bearer
            .gifts()
            .list(GiftRequest::default().slice_id(-1))
            .await,
    );

    assert_api_error(bearer.supporters().status(&user, &target).await);
    assert_api_error(
        bearer
            .supporters()
            .supporting(&user, &SupporterListRequest::default().limit(20))
            .await,
    );
    assert_api_error(
        bearer
            .supporters()
            .supporters(
                &user,
                &SupporterListRequest::default().sort(SupporterSort::Ranking),
            )
            .await,
    );
    let batch = SupportBatch::new([target.clone()]);
    assert_api_error(bearer.supporters().support(&batch).await);
    assert_api_error(bearer.supporters().unsupport(&batch).await);

    assert_api_error(bearer.categories().list(Language::Ja).await);
    assert_api_error(
        bearer
            .search()
            .users(&UserSearchRequest::new(SearchTerms::new("official")).limit(10))
            .await,
    );
    assert_api_error(
        bearer
            .search()
            .lives(&LiveSearchRequest::new(LiveSearchKind::Recommend).limit(10))
            .await,
    );
    assert_api_error(bearer.broadcasting().rtmp_credentials().await);

    let events = WebhookEvents::new([WebhookEvent::LiveStart, WebhookEvent::LiveEnd]);
    assert_api_error(
        app.webhooks()
            .list(&WebhookListRequest::default().limit(20))
            .await,
    );
    let user_id = UserId::new("42");
    assert_api_error(app.webhooks().register(&user_id, &events).await);
    assert_api_error(app.webhooks().remove(&user_id, &events).await);
    assert_api_error(app.users().get(&target).await);

    let oauth = OAuthClient::builder(
        "client-id",
        "client-secret",
        Url::parse("https://example.com/callback").unwrap(),
    )
    .unwrap()
    .base_url(server.base_url.clone())
    .build()
    .unwrap();
    let Error::Api(oauth_error) = oauth.exchange_code("code").await.unwrap_err() else {
        panic!("expected OAuth API error");
    };
    assert_eq!(oauth_error.code, 1001);

    let requests = server.requests.lock().unwrap();
    assert!(requests.iter().any(|request| {
        request.starts_with("GET /users/caster%2Fname HTTP/1.1")
            && request.contains("authorization: Bearer bearer-token")
            && request.contains("x-api-version: 2.0")
    }));
    assert!(requests.iter().any(|request| {
        request.contains("/live/thumbnail")
            && !request.to_ascii_lowercase().contains("authorization:")
    }));
    assert!(requests.iter().any(|request| {
        request.starts_with("POST /webhooks HTTP/1.1")
            && request.contains("authorization: Basic Y2xpZW50LWlkOmNsaWVudC1zZWNyZXQ=")
            && request.contains(r#""events":["livestart","liveend"]"#)
    }));
    assert!(requests.iter().any(|request| {
        request.starts_with(
            "DELETE /webhooks?user_id=42&events%5B%5D=livestart&events%5B%5D=liveend HTTP/1.1",
        )
    }));
    assert!(requests.iter().any(|request| {
        request.starts_with("POST /oauth2/access_token HTTP/1.1")
            && request.contains("grant_type=authorization_code")
            && request.contains("client_secret=client-secret")
    }));
}

#[test]
fn convenience_constructors_compile() {
    let _: Client<BearerAuth> = Client::bearer("token").unwrap();
    let _: Client<AppAuth> = Client::application("id", "secret").unwrap();
}
