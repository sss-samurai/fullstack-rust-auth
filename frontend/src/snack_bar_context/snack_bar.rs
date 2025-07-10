// snackbar.rs

use yew::Callback;

#[derive(Clone, PartialEq)]
pub enum SnackbarType {
    Success,
    Warning,
    Error,
}

#[derive(Clone, PartialEq)]
pub struct SnackbarContext {
    pub show: Callback<(String, SnackbarType)>,
}
