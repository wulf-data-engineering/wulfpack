use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_cognitoidentityprovider::config::{Credentials, ProvideCredentials};

// LocalStack endpoint for local development and integration testing
#[cfg(any(debug_assertions, test))]
fn default_endpoint() -> Option<String> {
    Some("http://localhost:4566".into())
}

#[cfg(not(any(debug_assertions, test)))]
fn default_endpoint() -> Option<String> {
    None
}

// cognito-local endpoint for local development and integration testing
#[cfg(any(debug_assertions, test))]
fn default_cognito_endpoint() -> Option<String> {
    Some("http://localhost:9229".into())
}

#[cfg(not(any(debug_assertions, test)))]
fn default_cognito_endpoint() -> Option<String> {
    None
}

#[cfg(any(debug_assertions, test))]
fn default_credentials_provider() -> Option<impl ProvideCredentials> {
    Some(
        Credentials::builder()
            .access_key_id("local")
            .secret_access_key("local")
            .provider_name("dev")
            .build(),
    )
}

#[cfg(not(any(debug_assertions, test)))]
fn default_credentials_provider() -> Option<impl ProvideCredentials> {
    None
}

/// Load AWS SDK configuration.
/// Uses the ENDPOINT_URL environment variable if set.
/// In debug builds, defaults to LocalStack endpoint if not set and local credentials.
pub async fn load_aws_config() -> SdkConfig {
    let endpoint = std::env::var("ENDPOINT_URL").ok().or_else(default_endpoint);

    load_aws_config_for_endpoint(endpoint).await
}

/// Load AWS SDK configuration.
/// Uses the COGNITO_ENDPOINT_URL environment variable if set.
/// In debug builds, defaults to cognito-local endpoint if not set and local credentials.
pub async fn load_aws_cognito_config() -> SdkConfig {
    let endpoint = std::env::var("COGNITO_ENDPOINT_URL")
        .ok()
        .or_else(default_cognito_endpoint);

    load_aws_config_for_endpoint(endpoint).await
}

async fn load_aws_config_for_endpoint(endpoint: Option<String>) -> SdkConfig {
    let mut loader = aws_config::defaults(BehaviorVersion::latest());
    if let Some(url) = &endpoint {
        loader = loader.endpoint_url(url);
    }
    if let Some(credentials_provider) = default_credentials_provider() {
        loader = loader.credentials_provider(credentials_provider);
    }

    loader.load().await
}

///
/// Unit test can use this loader to get default credentials and using given mock server.
///
/// ```
/// use wiremock::matchers::{method, path, header};
/// use wiremock::{MockServer, Mock, ResponseTemplate};
/// use cognito_idp::Client;
///
/// // 1) Start mock server
/// let server = MockServer::start().await;
/// // ... define required routes
///
/// // 2) Create client
/// let shared_cfg = load_aws_config_for_mock(server).await;
/// let client = cognito_idp::Client::new(&shared_cfg);
/// // ... test your component that uses the client
/// ```
///
#[cfg(any(debug_assertions, test))]
pub async fn load_aws_config_for_mock(mock_server: &wiremock::MockServer) -> SdkConfig {
    load_aws_config_for_endpoint(Some(mock_server.uri())).await
}

