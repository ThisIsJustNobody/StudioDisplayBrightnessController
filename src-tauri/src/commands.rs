use serde::Serialize;

use crate::studio_display::{
    brightness,
    device::{list_studio_display_devices, HidDeviceMetadata},
    error::StudioDisplayError,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommandError {
    code: &'static str,
    message: &'static str,
}

impl From<StudioDisplayError> for CommandError {
    fn from(error: StudioDisplayError) -> Self {
        Self {
            code: error.code(),
            message: error.user_message(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetBrightnessResponse {
    brightness: u32,
}

#[tauri::command]
pub(crate) fn list_displays() -> Result<Vec<HidDeviceMetadata>, CommandError> {
    list_studio_display_devices().map_err(CommandError::from)
}

#[tauri::command]
pub(crate) fn set_brightness(value: u32) -> Result<SetBrightnessResponse, CommandError> {
    brightness::set_brightness(value).map_err(CommandError::from)?;

    Ok(SetBrightnessResponse { brightness: value })
}

#[cfg(test)]
mod tests {
    use crate::studio_display::error::StudioDisplayError;

    use super::*;

    #[test]
    fn maps_brightness_out_of_range_to_stable_command_error() {
        let error = CommandError::from(StudioDisplayError::BrightnessOutOfRange);

        assert_eq!(error.code, "BRIGHTNESS_OUT_OF_RANGE");
        assert_eq!(error.message, "亮度值超出支持范围");
    }
}
