use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self {
            status: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn ok_message(message: impl Into<String>) -> Self {
        Self {
            status: true,
            message: Some(message.into()),
            data: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            status: false,
            message: Some(message.into()),
            data: None,
        }
    }
}
