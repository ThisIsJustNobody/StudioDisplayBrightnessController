use hidapi::{DeviceInfo, HidApi};
use serde::Serialize;

use crate::studio_display::error::StudioDisplayError;

pub const APPLE_VENDOR_ID: u16 = 0x05AC;
pub const STUDIO_DISPLAY_PRODUCT_ID: u16 = 0x1114;
pub const STUDIO_DISPLAY_INTERFACE_NUMBER: i32 = 7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HidDeviceMetadata {
    pub vendor_id: u16,
    pub product_id: u16,
    pub interface_number: Option<i32>,
    pub path: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

impl HidDeviceMetadata {
    pub fn is_studio_display_control_interface(&self) -> bool {
        self.vendor_id == APPLE_VENDOR_ID
            && self.product_id == STUDIO_DISPLAY_PRODUCT_ID
            && (self.interface_number == Some(STUDIO_DISPLAY_INTERFACE_NUMBER)
                || (self.interface_number.is_none() && self.path.to_lowercase().contains("mi_07")))
    }
}

impl From<&DeviceInfo> for HidDeviceMetadata {
    fn from(device: &DeviceInfo) -> Self {
        let interface_number = device.interface_number();

        Self {
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),
            interface_number: (interface_number >= 0).then_some(interface_number),
            path: device.path().to_string_lossy().into_owned(),
            manufacturer: device.manufacturer_string().map(ToOwned::to_owned),
            product: device.product_string().map(ToOwned::to_owned),
        }
    }
}

pub fn list_studio_display_devices() -> Result<Vec<HidDeviceMetadata>, StudioDisplayError> {
    let api = HidApi::new().map_err(|err| StudioDisplayError::OpenFailed(err.to_string()))?;
    Ok(api
        .device_list()
        .map(HidDeviceMetadata::from)
        .filter(HidDeviceMetadata::is_studio_display_control_interface)
        .collect())
}

pub fn first_studio_display_device_info(api: &HidApi) -> Result<DeviceInfo, StudioDisplayError> {
    api.device_list()
        .find(|device| HidDeviceMetadata::from(*device).is_studio_display_control_interface())
        .cloned()
        .ok_or(StudioDisplayError::NoDisplayFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(interface_number: Option<i32>, path: &str) -> HidDeviceMetadata {
        HidDeviceMetadata {
            vendor_id: 0x05AC,
            product_id: 0x1114,
            interface_number,
            path: path.to_string(),
            manufacturer: Some("Apple Inc.".to_string()),
            product: Some("Studio Display".to_string()),
        }
    }

    #[test]
    fn matches_studio_display_control_interface_by_interface_number() {
        assert!(metadata(Some(7), r"\\?\hid#vid_05ac&pid_1114&mi_07")
            .is_studio_display_control_interface());
    }

    #[test]
    fn rejects_other_studio_display_interfaces() {
        assert!(!metadata(Some(6), r"\\?\hid#vid_05ac&pid_1114&mi_06")
            .is_studio_display_control_interface());
    }

    #[test]
    fn rejects_path_fallback_when_interface_number_is_not_control_interface() {
        assert!(!metadata(Some(6), r"\\?\hid#vid_05ac&pid_1114&mi_07")
            .is_studio_display_control_interface());
    }

    #[test]
    fn matches_studio_display_control_interface_by_path_when_interface_number_missing() {
        assert!(metadata(None, r"\\?\hid#vid_05ac&pid_1114&mi_07")
            .is_studio_display_control_interface());
    }
}
