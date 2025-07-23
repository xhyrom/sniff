use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::get_details_multi,
        crate::handlers::get_details_single,
        crate::handlers::get_download_info,
    ),
    components(
        schemas(
            ApiResponse<SerializableDetailsResponse>,
            MultiChannelApiResponse<SerializableDetailsResponse>,
            ApiResponse<DownloadInfo>,
            SerializableDetailsResponse,
            DownloadInfo,
            SplitFile,
            AdditionalFile,
            Item,
            DocumentDetails,
            AppDetails,
            Offer,
            AppInfo,
            AppInfoSection,
            AppInfoContainer,
        )
    ),
    tags(
        (name = "App Details", description = "Get Google Play Store app details"),
        (name = "Downloads", description = "Get app download information")
    ),
    info(
        title = "Sniff API",
        description = "API for retrieving Google Play Store app details across different release channels",
        version = "1.0.0",
        contact(
            name = "API Support",
            url = "https://xhyrom.dev/docs/sniff"
        )
    ),
    servers(
        (url = "/", description = "Current server")
    )
)]
pub struct ApiDoc;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MultiChannelApiResponse<T> {
    pub success: bool,
    pub data: Option<HashMap<String, T>>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "item": {
        "id": "com.discord",
        "sub_id": "com.discord",
        "type": 1,
        "category_id": 3,
        "title": "Discord - Talk, Play, Hang Out",
        "creator": "Discord Inc.",
        "description_html": "Discord is designed for gaming and great for just chilling with friends or building a community. Customize your own space and gather your friends to talk while playing your favorite games, or just hang out.<br><br>GROUP CHAT THAT'S ALL FUN & GAMES<br>∙ Discord is great for playing games and chilling with friends, or even building a worldwide community. Customize your own space to talk, play, and hang out in.",
        "promotional_description": "Group Chat That's Fun & Games",
        "mature": false,
        "available_for_preregistration": false,
        "force_shareability": false,
        "offer": [{
            "micros": 0,
            "currency_code": "EUR",
            "formatted_amount": "",
            "checkout_flow_required": false,
            "offer_type": 1
        }],
        "details": {
            "app_details": {
                "developer_name": "Discord Inc.",
                "version_code": 289020,
                "version_string": "289.20 - Stable",
                "info_download_size": 180070862,
                "developer_email": "support@discord.com",
                "developer_website": "https://dis.gd/contact",
                "info_download": "500,000,000+ downloads",
                "package_name": "com.discord",
                "recent_changes_html": "We've been hard at work making Discord better for you. This includes bug fixes and performance enhancements.",
                "info_updated_on": "Jul 21, 2025",
                "target_sdk_version": 35
            }
        },
        "app_info": {
            "section": [
                {
                    "label": "In-app purchases",
                    "container": {
                        "description": "€1.99 - €274.99 if billed through Play"
                    }
                },
                {
                    "label": "Offered by",
                    "container": {
                        "description": "Google Commerce Ltd"
                    }
                },
                {
                    "label": "Released on",
                    "container": {
                        "description": "May 13, 2015"
                    }
                }
            ]
        }
    },
    "footer_html": "All prices include VAT.",
    "enable_reviews": true
}))]
pub struct SerializableDetailsResponse {
    pub item: Option<Item>,
    pub footer_html: Option<String>,
    pub enable_reviews: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "main_apk_url": "https://play.googleapis.com/download/by-token/download?token=AOTCm0Q...",
    "splits": [
        {
            "name": "config.arm64_v8a",
            "download_url": "https://play.googleapis.com/download/by-token/download?token=AOTCm0R..."
        },
        {
            "name": "config.en",
            "download_url": "https://play.googleapis.com/download/by-token/download?token=AOTCm0S..."
        }
    ],
    "additional_files": []
}))]
pub struct DownloadInfo {
    #[schema(example = "https://play.googleapis.com/download/by-token/download?token=AOTCm0Q...")]
    pub main_apk_url: Option<String>,
    pub splits: Vec<SplitFile>,
    pub additional_files: Vec<AdditionalFile>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SplitFile {
    #[schema(example = "config.arm64_v8a")]
    pub name: Option<String>,
    #[schema(example = "https://play.googleapis.com/download/by-token/download?token=AOTCm0R...")]
    pub download_url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AdditionalFile {
    #[schema(example = "main.1234.com.example.obb")]
    pub filename: Option<String>,
    #[schema(example = "https://play.googleapis.com/download/by-token/download?token=AOTCm0T...")]
    pub download_url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Item {
    #[schema(example = "com.discord")]
    pub id: Option<String>,
    #[schema(example = "com.discord")]
    pub sub_id: Option<String>,
    #[schema(example = 1)]
    pub r#type: Option<i32>,
    #[schema(example = 3)]
    pub category_id: Option<i32>,
    #[schema(example = "Discord - Talk, Play, Hang Out")]
    pub title: Option<String>,
    #[schema(example = "Discord Inc.")]
    pub creator: Option<String>,
    pub description_html: Option<String>,
    #[schema(example = "Group Chat That's Fun & Games")]
    pub promotional_description: Option<String>,
    #[schema(example = false)]
    pub mature: Option<bool>,
    #[schema(example = false)]
    pub available_for_preregistration: Option<bool>,
    #[schema(example = false)]
    pub force_shareability: Option<bool>,
    pub offer: Vec<Offer>,
    pub details: Option<DocumentDetails>,
    pub app_info: Option<AppInfo>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DocumentDetails {
    pub app_details: Option<AppDetails>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AppDetails {
    #[schema(example = "Discord Inc.")]
    pub developer_name: Option<String>,
    #[schema(example = 289020)]
    pub version_code: Option<i32>,
    #[schema(example = "289.20 - Stable")]
    pub version_string: Option<String>,
    #[schema(example = 180070862)]
    pub info_download_size: Option<i64>,
    #[schema(example = "support@discord.com")]
    pub developer_email: Option<String>,
    #[schema(example = "https://dis.gd/contact")]
    pub developer_website: Option<String>,
    #[schema(example = "500,000,000+ downloads")]
    pub info_download: Option<String>,
    #[schema(example = "com.discord")]
    pub package_name: Option<String>,
    pub recent_changes_html: Option<String>,
    #[schema(example = "Jul 21, 2025")]
    pub info_updated_on: Option<String>,
    #[schema(example = 35)]
    pub target_sdk_version: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Offer {
    #[schema(example = 0)]
    pub micros: Option<i64>,
    #[schema(example = "EUR")]
    pub currency_code: Option<String>,
    #[schema(example = "")]
    pub formatted_amount: Option<String>,
    #[schema(example = false)]
    pub checkout_flow_required: Option<bool>,
    #[schema(example = 1)]
    pub offer_type: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AppInfo {
    pub section: Vec<AppInfoSection>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AppInfoSection {
    #[schema(example = "In-app purchases")]
    pub label: Option<String>,
    pub container: Option<AppInfoContainer>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AppInfoContainer {
    #[schema(example = "€1.99 - €274.99 if billed through Play")]
    pub description: Option<String>,
}

impl From<gpapi::DownloadInfo> for DownloadInfo {
    fn from(gpapi_download_info: gpapi::DownloadInfo) -> Self {
        let (main_apk_url, splits_data, additional_files_data) = gpapi_download_info;

        let splits = splits_data
            .into_iter()
            .map(|(name, url)| SplitFile {
                name,
                download_url: url,
            })
            .collect();

        let additional_files = additional_files_data
            .into_iter()
            .map(|(filename, url)| AdditionalFile {
                filename,
                download_url: url,
            })
            .collect();

        DownloadInfo {
            main_apk_url,
            splits,
            additional_files,
        }
    }
}
