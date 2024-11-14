use mockito::{Matcher, Mock};

/// Builder for a mock endpoint.
///
/// This struct allows for configuring a mock endpoint to be constructed in
/// the `mock_endpoint()` function. Options can be chained together to create
/// complex mocks.
///
/// The mock is only created once `mock_endpoint()` is called with the builder.
pub struct MockEndpointBuilder {
    /// The mock object we are building.
    mock: Mock,
}

impl MockEndpointBuilder {
    /// Create a new endpoint options struct
    pub fn new(method: &str, endpoint: &str, status: usize) -> Self {
        Self {
            mock: mockito::mock(method, endpoint)
                .with_status(status)
                .with_header("content-type", "application/json"),
        }
    }

    /// Set the response body of the mock endpoint.
    pub fn with_response_body<T>(mut self, body: T) -> Self
    where
        T: Into<String>,
    {
        self.mock = self.mock.with_body(body.into());
        self
    }

    /// Set the response body of the mock endpoint to a file with the given path.
    /// The path is relative to the `tests/integration/_responses` directory.
    pub fn with_response_file(mut self, path: &str) -> Self {
        let response_file = format!("tests/integration/_responses/{path}");

        self.mock = self.mock.with_body_from_file(response_file);
        self
    }

    /// Set the matcher for the response body of the mock endpoint. The mock will only
    /// respond to requests if the response body matches the matcher.
    pub fn with_matcher(mut self, matcher: Matcher) -> Self {
        self.mock = self.mock.match_body(matcher);
        self
    }

    /// Matches a header of the mock endpoint. The header must be present and its value must
    /// match the provided matcher in order for the endpoint to be reached.
    pub fn with_header_matcher(mut self, key: &'static str, matcher: Matcher) -> Self {
        self.mock = self.mock.match_header(key, matcher);
        self
    }

    /// Expect the mock endpoint to be hit at least `hits` times.
    ///
    /// This expectation is only checked when the created mock is asserted.
    pub fn expect_at_least(mut self, hits: usize) -> Self {
        self.mock = self.mock.expect_at_least(hits);
        self
    }

    /// Expect the mock endpoint to be hit exactly `hits` times.
    ///
    /// This expectation is only checked when the created mock is asserted.
    pub fn expect(mut self, hits: usize) -> Self {
        self.mock = self.mock.expect(hits);
        self
    }
}

/// Build and return a mock endpoint with the provided configuration. The mock is automatically
/// created and started. It is active until dropped.
pub fn mock_endpoint(opts: MockEndpointBuilder) -> Mock {
    opts.mock.create()
}
