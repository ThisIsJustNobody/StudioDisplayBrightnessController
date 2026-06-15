use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StudioDisplayError {
    #[error("No Apple Studio Display control interface found")]
    NoDisplayFound,
    #[error("Brightness value is out of range")]
    BrightnessOutOfRange,
    #[error("Failed to open HID device: {0}")]
    OpenFailed(String),
    #[error("Failed to send HID feature report: {0}")]
    SendFailed(String),
}

impl StudioDisplayError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoDisplayFound => "NO_DISPLAY_FOUND",
            Self::BrightnessOutOfRange => "BRIGHTNESS_OUT_OF_RANGE",
            Self::OpenFailed(_) => "HID_OPEN_FAILED",
            Self::SendFailed(_) => "HID_SEND_FAILED",
        }
    }

    pub fn user_message(&self) -> &'static str {
        match self {
            Self::NoDisplayFound => "未找到 Apple Studio Display 控制接口",
            Self::BrightnessOutOfRange => "亮度值超出支持范围",
            Self::OpenFailed(_) => "打开 Apple Studio Display 控制接口失败",
            Self::SendFailed(_) => "写入 Apple Studio Display 亮度失败",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_display_found_has_stable_code_and_user_message() {
        assert_eq!(
            StudioDisplayError::NoDisplayFound.code(),
            "NO_DISPLAY_FOUND"
        );
        assert_eq!(
            StudioDisplayError::NoDisplayFound.user_message(),
            "未找到 Apple Studio Display 控制接口"
        );
    }

    #[test]
    fn hid_failures_have_stable_codes() {
        assert_eq!(
            StudioDisplayError::OpenFailed("x".into()).code(),
            "HID_OPEN_FAILED"
        );
        assert_eq!(
            StudioDisplayError::SendFailed("x".into()).code(),
            "HID_SEND_FAILED"
        );
    }
}
