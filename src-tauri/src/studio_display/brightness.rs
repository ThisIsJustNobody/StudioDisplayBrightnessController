use hidapi::{HidApi, HidDevice};

use crate::studio_display::{
    device::first_studio_display_device_info,
    error::StudioDisplayError,
    protocol::{build_brightness_feature_report, ProtocolError},
};

pub trait FeatureReportWriter {
    fn send_feature_report(&mut self, report: &[u8]) -> Result<(), StudioDisplayError>;
}

impl FeatureReportWriter for HidDevice {
    fn send_feature_report(&mut self, report: &[u8]) -> Result<(), StudioDisplayError> {
        HidDevice::send_feature_report(self, report)
            .map(|_| ())
            .map_err(|err| StudioDisplayError::SendFailed(err.to_string()))
    }
}

pub fn write_brightness_to_writer(
    writer: &mut impl FeatureReportWriter,
    value: u32,
) -> Result<(), StudioDisplayError> {
    let report = build_brightness_feature_report(value).map_err(|err| match err {
        ProtocolError::OutOfRange => StudioDisplayError::BrightnessOutOfRange,
    })?;

    writer.send_feature_report(&report)
}

pub fn set_brightness(value: u32) -> Result<(), StudioDisplayError> {
    let api = HidApi::new().map_err(|err| StudioDisplayError::OpenFailed(err.to_string()))?;
    let device_info = first_studio_display_device_info(&api)?;
    let mut device = device_info
        .open_device(&api)
        .map_err(|err| StudioDisplayError::OpenFailed(err.to_string()))?;

    write_brightness_to_writer(&mut device, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct RecordingWriter {
        reports: Vec<Vec<u8>>,
    }

    impl FeatureReportWriter for RecordingWriter {
        fn send_feature_report(
            &mut self,
            report: &[u8],
        ) -> Result<(), crate::studio_display::error::StudioDisplayError> {
            self.reports.push(report.to_vec());
            Ok(())
        }
    }

    #[test]
    fn writes_brightness_feature_report_to_writer() {
        let mut writer = RecordingWriter::default();

        write_brightness_to_writer(&mut writer, 30_000).unwrap();

        assert_eq!(
            writer.reports,
            vec![vec![0x01, 0x30, 0x75, 0x00, 0x00, 0x00, 0x00, 0x00]]
        );
    }
}
