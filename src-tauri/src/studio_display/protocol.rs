pub const MIN_BRIGHTNESS: u32 = 400;
pub const MAX_BRIGHTNESS: u32 = 60_000;
pub const REPORT_ID: u8 = 0x01;
pub const FEATURE_REPORT_LEN_WITH_ID: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    OutOfRange,
}

pub fn validate_brightness(value: u32) -> Result<u32, ProtocolError> {
    if (MIN_BRIGHTNESS..=MAX_BRIGHTNESS).contains(&value) {
        Ok(value)
    } else {
        Err(ProtocolError::OutOfRange)
    }
}

pub fn brightness_from_percent(percent: u8) -> u32 {
    let clamped = percent.min(100) as f64;
    let span = (MAX_BRIGHTNESS - MIN_BRIGHTNESS) as f64;
    MIN_BRIGHTNESS + ((clamped / 100.0) * span).round() as u32
}

pub fn build_brightness_feature_report(
    value: u32,
) -> Result<[u8; FEATURE_REPORT_LEN_WITH_ID], ProtocolError> {
    let value = validate_brightness(value)?;
    let mut report = [0_u8; FEATURE_REPORT_LEN_WITH_ID];
    report[0] = REPORT_ID;
    report[1..5].copy_from_slice(&value.to_le_bytes());
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_verified_feature_reports() {
        assert_eq!(
            build_brightness_feature_report(30_000).unwrap(),
            [0x01, 0x30, 0x75, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            build_brightness_feature_report(20_000).unwrap(),
            [0x01, 0x20, 0x4E, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            build_brightness_feature_report(10_000).unwrap(),
            [0x01, 0x10, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn rejects_values_outside_supported_range() {
        assert_eq!(
            validate_brightness(MIN_BRIGHTNESS - 1),
            Err(ProtocolError::OutOfRange)
        );
        assert_eq!(
            validate_brightness(MAX_BRIGHTNESS + 1),
            Err(ProtocolError::OutOfRange)
        );
    }

    #[test]
    fn maps_percent_to_protocol_range() {
        assert_eq!(brightness_from_percent(0), MIN_BRIGHTNESS);
        assert_eq!(brightness_from_percent(100), MAX_BRIGHTNESS);
        assert_eq!(brightness_from_percent(50), 30_200);
    }
}
