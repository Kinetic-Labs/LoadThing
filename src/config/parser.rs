use super::structure::{Config, FeaturesConfig, ProxyConfig, WebConfig};
use rnix::{Root, SyntaxKind, SyntaxNode};
use std::{
    fs,
    io::{self, Error},
};

fn find_attr_value(node: &SyntaxNode, attr_name: &str) -> Option<String> {
    for child in node.descendants() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            let mut key_found = false;
            let mut value = None;

            for inner in child.children() {
                if inner.kind() == SyntaxKind::NODE_ATTRPATH {
                    let key_text = inner.text().to_string();
                    if key_text == attr_name {
                        key_found = true;
                    }
                }

                if key_found && inner.kind() == SyntaxKind::NODE_STRING {
                    let text = inner.text().to_string();

                    value = Some(text.trim_matches('"').to_string());

                    break;
                } else if key_found && inner.kind() == SyntaxKind::NODE_LITERAL {
                    let text = inner.text().to_string();

                    value = Some(text);

                    break;
                } else if key_found && inner.kind() == SyntaxKind::NODE_IDENT {
                    let text = inner.text().to_string();

                    if text == "true" || text == "false" {
                        value = Some(text);
                        break;
                    }
                }
            }

            if value.is_some() {
                return value;
            }
        }
    }
    None
}

fn find_attrset(node: &SyntaxNode, set_name: &str) -> Option<SyntaxNode> {
    for child in node.descendants() {
        if child.kind() == SyntaxKind::NODE_ATTRPATH_VALUE {
            let mut is_target = false;

            for inner in child.children() {
                if inner.kind() == SyntaxKind::NODE_ATTRPATH {
                    if inner.text().to_string() == set_name {
                        is_target = true;
                    }
                }

                if is_target && inner.kind() == SyntaxKind::NODE_ATTR_SET {
                    return Some(inner);
                }
            }
        }
    }
    None
}

pub fn parse_config() -> io::Result<Config> {
    let config_str = fs::read_to_string("config/loadthing.nix")
        .map_err(|error| Error::new(error.kind(), "Failed to read config"))?;

    let parsed = Root::parse(&config_str);
    let root = parsed.syntax();

    let proxy_target = find_attrset(&root, "proxy")
        .and_then(|node| find_attr_value(&node, "target"))
        .unwrap_or_else(|| "http://localhost".to_string());

    let proxy_port = find_attrset(&root, "proxy")
        .and_then(|node| find_attr_value(&node, "port"))
        .and_then(|s| s.parse().ok())
        .unwrap_or(80);

    let proxy_path = find_attrset(&root, "proxy")
        .and_then(|node| find_attr_value(&node, "path"))
        .unwrap_or_else(|| "/".to_string());

    let lt_port = find_attrset(&root, "web")
        .and_then(|node| find_attr_value(&node, "port"))
        .and_then(|s| s.parse().ok())
        .unwrap_or(9595);

    let lt_hostname = find_attrset(&root, "web")
        .and_then(|node| find_attr_value(&node, "hostname"))
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let log_enabled = find_attrset(&root, "features")
        .and_then(|node| find_attr_value(&node, "log"))
        .map(|s| s == "true")
        .unwrap_or(true);

    Ok(Config {
        proxy_config: ProxyConfig {
            target: proxy_target,
            port: proxy_port,
            path: proxy_path,
        },
        web_config: WebConfig {
            port: lt_port,
            hostname: lt_hostname,
        },
        features_config: FeaturesConfig { log: log_enabled },
    })
}
