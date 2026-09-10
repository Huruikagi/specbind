use crate::cli::CommandOutput;
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct JsonResponse<T> {
    pub(super) status: &'static str,
    pub(super) code: &'static str,
    pub(super) data: T,
}

#[derive(Serialize)]
struct JsonFailure {
    status: &'static str,
    code: String,
    message: String,
    details: Vec<String>,
}

pub(super) fn render_failure(
    code: impl Into<String>,
    message: impl Into<String>,
    details: Vec<String>,
) -> CommandOutput {
    render(
        &JsonFailure {
            status: "error",
            code: code.into(),
            message: message.into(),
            details,
        },
        false,
    )
}

pub(super) fn render(value: &impl Serialize, success: bool) -> CommandOutput {
    let mut stdout = serde_json::to_vec(value).expect("status JSON response is serializable");
    stdout.push(b'\n');
    CommandOutput {
        stdout,
        stderr: vec![],
        success,
    }
}
