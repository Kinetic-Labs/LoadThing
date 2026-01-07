pub struct ProxyConfig {
    pub target: String,
    pub port: u16,
    pub path: String,
}

pub struct WebConfig {
    pub port: u16,
    pub hostname: String,
}

pub struct FeaturesConfig {
    pub log: bool,
}

pub struct Config {
    pub proxy_config: ProxyConfig,
    pub web_config: WebConfig,
    pub features_config: FeaturesConfig,
}
