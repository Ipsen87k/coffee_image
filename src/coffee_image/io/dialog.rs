use rfd::MessageDialog;

use crate::coffee_image::error::Error;

pub fn error_dialog_show(error: Error) {
    let _ = MessageDialog::new()
        .set_title("error")
        .set_description(error)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}
