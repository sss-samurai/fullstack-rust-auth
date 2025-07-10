use crate::snack_bar_context::snack_bar::{SnackbarContext, SnackbarType};

pub fn trigger_snack(snackbar: &SnackbarContext, msg: &str, kind: SnackbarType) {
    snackbar.show.emit((msg.to_string(), kind));
}
