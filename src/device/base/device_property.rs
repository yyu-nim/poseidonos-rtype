use strum_macros::Display;
use uuid::Uuid;

#[derive(Display, PartialEq, Debug, Clone)]
pub enum DeviceClass {
    SYSTEM,
    ARRAY,
}

#[derive(Display, Debug, Clone)]
pub enum DeviceType {
    SSD,
    NVRAM,
}

#[derive(Debug, Clone)]
pub struct DeviceProperty {
    pub deviceType: DeviceType,
    pub deviceClass: Option<DeviceClass>,
    pub name: String,
    pub size: usize,
    pub mn: String,
    pub sn: String,
    pub fr: String,
    pub bdf: String,
    pub numa: i32,
}

impl DeviceProperty {
    pub fn new(deviceType: DeviceType, name: String, size: usize) -> Self {
        Self {
            deviceType,
            deviceClass: None,
            name,
            size,
            mn: String::new(),
            sn: Uuid::new_v4().to_string(),
            fr: String::new(),
            bdf: String::new(),
            numa: 0,
        }
    }
    pub fn GetType(&self) -> String {
        self.deviceType.to_string()
    }

    pub fn GetClass(&self) -> String {
        match &self.deviceClass {
            Some(class) => class.to_string(),
            None => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_to_string() {
        assert_eq!("SYSTEM".to_string(), DeviceClass::SYSTEM.to_string());
        assert_eq!("ARRAY".to_string(), DeviceClass::ARRAY.to_string());

        assert_eq!("SSD".to_string(), DeviceType::SSD.to_string());
        assert_eq!("NVRAM".to_string(), DeviceType::NVRAM.to_string());
    }
}
