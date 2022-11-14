use lazy_static::lazy_static;
lazy_static!{
    pub static ref VersionProviderSingleton: VersionProvider = {
        VersionProvider::new("pos-rtype")
    };
}

pub struct VersionProvider {
    VERSION: &'static str,
}

impl VersionProvider {
    pub fn new(VERSION: &'static str) -> VersionProvider {
        VersionProvider {
            VERSION
        }
    }

    pub fn ver(&self) -> &'static str {
        self.VERSION
    }
}