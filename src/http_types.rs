use axum::{
    http::{HeaderValue, header},
    response::IntoResponse,
    response::Response,
};

/// An CSS response.
///
/// Will automatically get `Content-Type: text/css`.
#[derive(Clone, Copy, Debug)]
#[allow(unused)]
pub struct OxCss<T>(pub T);

impl<T> IntoResponse for OxCss<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::TEXT_CSS_UTF_8.as_ref()),
            )],
            self.0,
        )
            .into_response()
    }
}

impl<T> From<T> for OxCss<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

pub fn ox_css(content: String) -> OxCss<String> {
    OxCss(content)
}

/// An CSS response.
///
/// Will automatically get `Content-Type: text/css`.
#[derive(Clone, Copy, Debug)]
#[allow(unused)]
pub struct OxJs<T>(pub T);

impl<T> IntoResponse for OxJs<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::TEXT_JAVASCRIPT.as_ref()),
            )],
            self.0,
        )
            .into_response()
    }
}

impl<T> From<T> for OxJs<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

pub fn ox_js(content: String) -> OxJs<String> {
    OxJs(content)
}
