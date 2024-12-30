use std::{fs::File, io::Read};

use cubix::getset2::Getset2;
use serde::{Deserialize, Serialize};

/// Config is the core configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Getset2)]
#[getset2(get_ref(pub))]
pub struct Config {
    /// Casdoor Server Url, such as `http://localhost:8000`
    endpoint: String,
    /// Client ID for the Casdoor application
    client_id: String,
    /// Client secret for the Casdoor application
    client_secret: String,
    /// x509 certificate content of Application.cert
    certificate: String,
    /// The name for the Casdoor organization
    org_name: String,
    /// The name for the Casdoor application
    app_name: Option<String>,
}

impl Config {
    /// Create a new Config.
    pub fn new(
        endpoint: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        certificate: impl Into<String>,
        org_name: impl Into<String>,
        app_name: Option<String>,
    ) -> Self {
        Config {
            endpoint: endpoint.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            certificate: Self::replace_cert_to_pub_key(certificate.into()),
            org_name: org_name.into(),
            app_name,
        }
    }

    /// Create a new Config from a Toml file.
    pub fn from_toml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // read path file content
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut conf: Config = toml::from_str(&content)?;

        // need to convert the certificate to pem format
        conf.certificate = Self::replace_cert_to_pub_key(conf.certificate);

        Ok(conf)
    }

    fn replace_cert_to_pub_key(certificate: String) -> String {
        certificate.replace("CERTIFICATE", "PUBLIC KEY")
    }
}
