use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Cursor, Write};
use std::path::Path;

use configparser::ini::Ini;
use prost::Message;

use googleplay_protobuf::{
    AndroidBuildProto, AndroidCheckinProto, DeviceConfigurationProto, DeviceFeature,
};

use bincode::{Decode, Encode};
include!("src/device_properties.rs");

fn main() {
    if !Path::new("src/device_properties.bin").exists() {
        let mut config = Ini::new();
        config
            .read(fs::read_to_string("device.properties").unwrap())
            .unwrap();

        let mut device_properties_map = HashMap::new();
        for section in config.sections() {
            println!("{:?}", section);
            let mut extra_info = HashMap::new();
            extra_info.insert(
                "Build.ID".to_string(),
                config.get(&section, "Build.ID").unwrap_or_default(),
            );
            extra_info.insert(
                "Vending.versionString".to_string(),
                config
                    .get(&section, "Vending.versionString")
                    .unwrap_or_default(),
            );
            extra_info.insert(
                "Vending.version".to_string(),
                config.get(&section, "Vending.version").unwrap_or_default(),
            );
            extra_info.insert(
                "Build.VERSION.RELEASE".to_string(),
                config
                    .get(&section, "Build.VERSION.RELEASE")
                    .unwrap_or_default(),
            );
            if let Some(sim_operator) = config.get(&section, "SimOperator") {
                extra_info.insert("SimOperator".to_string(), sim_operator);
            }
            let android_build = AndroidBuildProto {
                id: config.get(&section, "Build.FINGERPRINT"),
                product: config.get(&section, "Build.HARDWARE"),
                carrier: config.get(&section, "Build.BRAND"),
                radio: config.get(&section, "Build.RADIO"),
                bootloader: config.get(&section, "Build.BOOTLOADER"),
                device: config.get(&section, "Build.DEVICE"),
                sdk_version: config
                    .getint(&section, "Build.VERSION.SDK_INT")
                    .unwrap()
                    .map(|v| v as i32),
                model: config.get(&section, "Build.MODEL"),
                manufacturer: config.get(&section, "Build.MANUFACTURER"),
                build_product: config.get(&section, "Build.PRODUCT"),
                client: config.get(&section, "Client"),
                ota_installed: Some(false),
                google_services: config
                    .getint(&section, "GSF.version")
                    .unwrap()
                    .map(|v| v as i32),
                ..Default::default()
            };
            let android_checkin = AndroidCheckinProto {
                build: Some(android_build),
                last_checkin_msec: Some(0),
                cell_operator: config.get(&section, "CellOperator"),
                sim_operator: config.get(&section, "SimOperator"),
                roaming: config.get(&section, "Roaming"),
                user_number: Some(0),
                ..Default::default()
            };
            let mut android_checkin_encoded = Vec::with_capacity(android_checkin.encoded_len());
            android_checkin
                .encode(&mut android_checkin_encoded)
                .unwrap();

            let device_configuration = DeviceConfigurationProto {
                touch_screen: config
                    .getint(&section, "TouchScreen")
                    .unwrap()
                    .map(|v| v as i32),
                keyboard: config
                    .getint(&section, "Keyboard")
                    .unwrap()
                    .map(|v| v as i32),
                navigation: config
                    .getint(&section, "Navigation")
                    .unwrap()
                    .map(|v| v as i32),
                screen_layout: config
                    .getint(&section, "ScreenLayout")
                    .unwrap()
                    .map(|v| v as i32),
                has_hard_keyboard: config.getbool(&section, "HasHardKeyboard").unwrap(),
                has_five_way_navigation: config.getbool(&section, "HasFiveWayNavigation").unwrap(),
                screen_density: config
                    .getint(&section, "Screen.Density")
                    .unwrap()
                    .map(|v| v as i32),
                gl_es_version: config
                    .getint(&section, "GL.Version")
                    .unwrap()
                    .map(|v| v as i32),
                system_shared_library: config
                    .get(&section, "SharedLibraries")
                    .unwrap()
                    .split(",")
                    .map(String::from)
                    .collect(),
                system_available_feature: config
                    .get(&section, "Features")
                    .unwrap()
                    .split(",")
                    .map(String::from)
                    .collect(),
                native_platform: config
                    .get(&section, "Platforms")
                    .unwrap_or_default()
                    .split(",")
                    .map(String::from)
                    .collect(),
                screen_width: config
                    .getint(&section, "Screen.Width")
                    .unwrap()
                    .map(|v| v as i32),
                screen_height: config
                    .getint(&section, "Screen.Height")
                    .unwrap()
                    .map(|v| v as i32),
                system_supported_locale: config
                    .get(&section, "Locales")
                    .unwrap()
                    .split(",")
                    .map(String::from)
                    .collect(),
                gl_extension: config
                    .get(&section, "GL.Extensions")
                    .unwrap()
                    .split(",")
                    .map(String::from)
                    .collect(),
                device_feature: config
                    .get(&section, "Features")
                    .unwrap()
                    .split(",")
                    .map(|s| {
                        let feature_name = String::from(s);
                        DeviceFeature {
                            name: Some(feature_name),
                            value: Some(0),
                        }
                    })
                    .collect(),
                ..Default::default()
            };
            let mut device_configuration_encoded =
                Vec::with_capacity(device_configuration.encoded_len());
            device_configuration
                .encode(&mut device_configuration_encoded)
                .unwrap();
            let device_properties_encoded = EncodedDeviceProperties::new(
                device_configuration_encoded,
                android_checkin_encoded,
                extra_info,
            );
            device_properties_map.insert(
                section.replace("gplayapi_", "").replace(".properties", ""),
                device_properties_encoded,
            );
        }

        let devices_encoded: Vec<u8> =
            bincode::encode_to_vec(&device_properties_map, bincode::config::standard()).unwrap();

        let mut file = File::create("src/device_properties.bin").unwrap();
        file.write_all(&devices_encoded).unwrap();
    }
}
