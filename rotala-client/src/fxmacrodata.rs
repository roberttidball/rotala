pub const API_KEY_HEADER: &str = "X-API-Key";

#[derive(Clone)]
pub struct FXMacroDataClient {
    base_url: String,
    api_key: Option<String>,
}

impl FXMacroDataClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        Self {
            base_url: "https://api.fxmacrodata.com/v1".to_string(),
            api_key: if api_key.is_empty() {
                None
            } else {
                Some(api_key)
            },
        }
    }

    pub fn data_catalogue(&self, currency: &str) -> String {
        self.url(&format!("/data_catalogue/{}", norm(currency)))
    }
    pub fn announcements(&self, currency: &str, indicator: &str) -> String {
        self.url(&format!("/announcements/{}/{}", norm(currency), indicator))
    }
    pub fn calendar(&self, currency: &str) -> String {
        self.url(&format!("/calendar/{}", norm(currency)))
    }
    pub fn predictions(&self, currency: &str, indicator: &str) -> String {
        self.url(&format!("/predictions/{}/{}", norm(currency), indicator))
    }
    pub fn forex(&self, base: &str, quote: &str) -> String {
        self.url(&format!("/forex/{}/{}", norm(base), norm(quote)))
    }
    pub fn cot(&self, currency: &str) -> String {
        self.url(&format!("/cot/{}", norm(currency)))
    }
    pub fn commodities_latest(&self) -> String {
        self.url("/commodities/latest")
    }
    pub fn commodity(&self, indicator: &str) -> String {
        self.url(&format!("/commodities/{}", indicator))
    }
    pub fn curves(&self, currency: &str) -> String {
        self.url(&format!("/curves/{}", norm(currency)))
    }
    pub fn curve_proxies(&self, currency: &str) -> String {
        self.url(&format!("/curve_proxies/{}", norm(currency)))
    }
    pub fn forward_curves(&self, currency: &str) -> String {
        self.url(&format!("/forward_curves/{}", norm(currency)))
    }
    pub fn market_sessions(&self) -> String {
        self.url("/market_sessions")
    }
    pub fn risk_sentiment(&self) -> String {
        self.url("/risk_sentiment")
    }
    pub fn news(&self, currency: &str) -> String {
        self.url(&format!("/news/{}", norm(currency)))
    }
    pub fn press_releases(&self, currency: &str) -> String {
        self.url(&format!("/press-releases/{}", norm(currency)))
    }
    pub fn central_bankers(&self, currency: &str) -> String {
        self.url(&format!("/central_bankers/{}", norm(currency)))
    }

    /// Header to send with each request, or `None` when no key is set.
    pub fn api_key_header(&self) -> Option<(&'static str, &str)> {
        self.api_key.as_deref().map(|key| (API_KEY_HEADER, key))
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

impl std::fmt::Debug for FXMacroDataClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FXMacroDataClient")
            .field("base_url", &self.base_url)
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

fn norm(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
