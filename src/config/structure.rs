#[derive(Clone)]
pub struct ProxyConfig {
    pub target: String,
    pub port: u16,
    pub path: String,
}

#[derive(Clone)]
pub struct WebConfig {
    pub port: u16,
    pub hostname: String,
}

#[derive(Clone)]
pub struct FeaturesConfig {
    pub log: bool,
    pub time: bool,
}

#[derive(Clone)]
pub struct Config {
    pub proxy_config: ProxyConfig,
    pub web_config: WebConfig,
    pub features_config: FeaturesConfig,
}
