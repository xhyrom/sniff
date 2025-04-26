use googleplay_protobuf::DetailsResponse;
use serde::Serialize;

mod details_response_serde {
    use googleplay_protobuf::{
        AppDetails, AppInfo, AppInfoContainer, AppInfoSection, DetailsResponse, DiscoveryBadge,
        DiscoveryBadgeLink, DocumentDetails, Feature, Features, Image, Item, Link, Offer,
        PlayerBadge,
    };
    use serde::ser::{SerializeStruct, Serializer};

    pub fn serialize<S>(details: &DetailsResponse, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DetailsResponse", 10)?;

        if let Some(ref item) = details.item {
            state.serialize_field("item", &SerializableItem(item))?;
        }

        if let Some(ref footer_html) = details.footer_html {
            state.serialize_field("footer_html", footer_html)?;
        }

        if !details.discovery_badge.is_empty() {
            let serializable_badges: Vec<SerializableDiscoveryBadge> = details
                .discovery_badge
                .iter()
                .map(SerializableDiscoveryBadge)
                .collect();
            state.serialize_field("discovery_badge", &serializable_badges)?;
        }

        if let Some(enable_reviews) = details.enable_reviews {
            state.serialize_field("enable_reviews", &enable_reviews)?;
        }

        if let Some(ref features) = details.features {
            state.serialize_field("features", &SerializableFeatures(features))?;
        }

        state.end()
    }

    struct SerializableItem<'a>(&'a Item);
    struct SerializableDiscoveryBadge<'a>(&'a DiscoveryBadge);
    struct SerializableFeatures<'a>(&'a Features);
    struct SerializableFeature<'a>(&'a Feature);
    struct SerializablePlayerBadge<'a>(&'a PlayerBadge);
    struct SerializableDiscoveryBadgeLink<'a>(&'a DiscoveryBadgeLink);
    struct SerializableImage<'a>(&'a Image);
    struct SerializableLink<'a>(&'a Link);
    struct SerializableAppInfo<'a>(&'a AppInfo);
    struct SerializableAppInfoSection<'a>(&'a AppInfoSection);
    struct SerializableAppInfoContainer<'a>(&'a AppInfoContainer);
    struct SerializableAppDetails<'a>(&'a AppDetails);
    struct SerializableDocumentDetails<'a>(&'a DocumentDetails);
    struct SerializableOffer<'a>(&'a Offer);

    impl<'a> serde::Serialize for SerializableItem<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let item = self.0;
            let mut state = serializer.serialize_struct("Item", 20)?;

            if let Some(ref id) = item.id {
                state.serialize_field("id", id)?;
            }

            if let Some(ref sub_id) = item.sub_id {
                state.serialize_field("sub_id", sub_id)?;
            }

            if let Some(ref r#type) = item.r#type {
                state.serialize_field("type", r#type)?;
            }

            if let Some(ref category_id) = item.category_id {
                state.serialize_field("category_id", category_id)?;
            }

            if let Some(ref title) = item.title {
                state.serialize_field("title", title)?;
            }

            if let Some(ref creator) = item.creator {
                state.serialize_field("creator", creator)?;
            }

            if let Some(ref description_html) = item.description_html {
                state.serialize_field("description_html", description_html)?;
            }

            // offer
            if !item.offer.is_empty() {
                let serializable_offers: Vec<SerializableOffer> =
                    item.offer.iter().map(SerializableOffer).collect();
                state.serialize_field("offer", &serializable_offers)?;
            }

            // availability

            // container_metadata

            if let Some(ref details) = item.details {
                state.serialize_field("details", &SerializableDocumentDetails(details))?;
            }

            // aggregate_rating

            if let Some(ref subtitle) = item.subtitle {
                state.serialize_field("subtitle", subtitle)?;
            }

            if let Some(ref app_info) = item.app_info {
                state.serialize_field("app_info", &SerializableAppInfo(app_info))?;
            }

            if let Some(ref mature) = item.mature {
                state.serialize_field("mature", mature)?;
            }

            if let Some(ref promotional_description) = item.promotional_description {
                state.serialize_field("promotional_description", promotional_description)?;
            }

            if let Some(ref available_for_preregistration) = item.available_for_preregistration {
                state.serialize_field(
                    "available_for_preregistration",
                    available_for_preregistration,
                )?;
            }

            // tip

            if let Some(ref force_shareability) = item.force_shareability {
                state.serialize_field("force_shareability", force_shareability)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableDiscoveryBadge<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let badge = self.0;
            let mut state = serializer.serialize_struct("DiscoveryBadge", 14)?;

            if let Some(ref label) = badge.label {
                state.serialize_field("label", label)?;
            }

            if let Some(ref image) = badge.image {
                state.serialize_field("image", &SerializableImage(image))?;
            }

            if let Some(background_color) = badge.background_color {
                state.serialize_field("background_color", &background_color)?;
            }

            if let Some(ref badge_container1) = badge.badge_container1 {
                state.serialize_field(
                    "badge_container1",
                    &SerializableDiscoveryBadgeLink(badge_container1),
                )?;
            }

            if let Some(is_plus_one) = badge.is_plus_one {
                state.serialize_field("is_plus_one", &is_plus_one)?;
            }

            if let Some(aggregate_rating) = badge.aggregate_rating {
                state.serialize_field("aggregate_rating", &aggregate_rating)?;
            }

            if let Some(user_star_rating) = badge.user_star_rating {
                state.serialize_field("user_star_rating", &user_star_rating)?;
            }

            if let Some(ref download_count) = badge.download_count {
                state.serialize_field("download_count", download_count)?;
            }

            if let Some(ref download_units) = badge.download_units {
                state.serialize_field("download_units", download_units)?;
            }

            if let Some(ref content_description) = badge.content_description {
                state.serialize_field("content_description", content_description)?;
            }

            if let Some(ref player_badge) = badge.player_badge {
                state.serialize_field("player_badge", &SerializablePlayerBadge(player_badge))?;
            }

            if let Some(ref family_age_range_badge) = badge.family_age_range_badge {
                state.serialize_field("family_age_range_badge", family_age_range_badge)?;
            }

            if let Some(ref family_category_badge) = badge.family_category_badge {
                state.serialize_field("family_category_badge", family_category_badge)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableFeatures<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let features = self.0;
            let mut state = serializer.serialize_struct("Features", 2)?;

            if !features.feature_presence.is_empty() {
                let serializable_features: Vec<SerializableFeature> = features
                    .feature_presence
                    .iter()
                    .map(SerializableFeature)
                    .collect();
                state.serialize_field("feature_presence", &serializable_features)?;
            }

            if !features.feature_rating.is_empty() {
                let serializable_ratings: Vec<SerializableFeature> = features
                    .feature_rating
                    .iter()
                    .map(SerializableFeature)
                    .collect();
                state.serialize_field("feature_rating", &serializable_ratings)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableFeature<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let feature = self.0;
            let mut state = serializer.serialize_struct("Feature", 2)?;

            if let Some(ref label) = feature.label {
                state.serialize_field("label", label)?;
            }

            if let Some(ref value) = feature.value {
                state.serialize_field("value", value)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializablePlayerBadge<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let badge = self.0;
            let mut state = serializer.serialize_struct("PlayerBadge", 1)?;

            if let Some(ref overlay_icon) = badge.overlay_icon {
                state.serialize_field("overlay_icon", &SerializableImage(overlay_icon))?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableDiscoveryBadgeLink<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let link = self.0;
            let mut state = serializer.serialize_struct("DiscoveryBadgeLink", 3)?;

            if let Some(ref badge_link) = link.link {
                state.serialize_field("link", &SerializableLink(badge_link))?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableImage<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let image = self.0;
            let mut state = serializer.serialize_struct("Image", 5)?;

            if let Some(ref url) = image.image_url {
                state.serialize_field("image_url", url)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableLink<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let link = self.0;
            let mut state = serializer.serialize_struct("Link", 3)?;

            if let Some(ref uri) = link.uri {
                state.serialize_field("uri", uri)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableAppInfo<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let app_info = self.0;
            let mut state = serializer.serialize_struct("AppInfo", 2)?;

            if let Some(ref title) = app_info.title {
                state.serialize_field("title", title)?;
            }

            if !app_info.section.is_empty() {
                let section: Vec<SerializableAppInfoSection> = app_info
                    .section
                    .iter()
                    .map(SerializableAppInfoSection)
                    .collect();

                state.serialize_field("section", &section)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableAppInfoSection<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let section = self.0;
            let mut state = serializer.serialize_struct("AppInfoSection", 2)?;

            if let Some(ref label) = section.label {
                state.serialize_field("label", label)?;
            }

            if let Some(ref container) = section.container {
                state.serialize_field("container", &SerializableAppInfoContainer(container))?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableAppInfoContainer<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let container = self.0;
            let mut state = serializer.serialize_struct("AppInfoContainer", 2)?;

            if let Some(ref image) = container.image {
                state.serialize_field("image", &SerializableImage(image))?;
            }

            if let Some(ref description) = container.description {
                state.serialize_field("description", description)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableAppDetails<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let details = self.0;
            let mut state = serializer.serialize_struct("AppDetails", 14)?;

            if let Some(ref developer_name) = details.developer_name {
                state.serialize_field("developer_name", developer_name)?;
            }

            if let Some(ref major_version_number) = details.major_version_number {
                state.serialize_field("major_version_number", major_version_number)?;
            }

            if let Some(ref version_code) = details.version_code {
                state.serialize_field("version_code", version_code)?;
            }

            if let Some(ref version_string) = details.version_string {
                state.serialize_field("version_string", version_string)?;
            }

            if let Some(ref title) = details.title {
                state.serialize_field("title", title)?;
            }

            // category

            if let Some(ref info_download_size) = details.info_download_size {
                state.serialize_field("info_download_size", info_download_size)?;
            }

            // permission

            if let Some(ref developer_email) = details.developer_email {
                state.serialize_field("developer_email", developer_email)?;
            }

            if let Some(ref developer_website) = details.developer_website {
                state.serialize_field("developer_website", developer_website)?;
            }

            if let Some(ref info_download) = details.info_download {
                state.serialize_field("info_download", info_download)?;
            }

            if let Some(ref package_name) = details.package_name {
                state.serialize_field("package_name", package_name)?;
            }

            if let Some(ref recent_changes_html) = details.recent_changes_html {
                state.serialize_field("recent_changes_html", recent_changes_html)?;
            }

            if let Some(ref info_updated_on) = details.info_updated_on {
                state.serialize_field("info_updated_on", info_updated_on)?;
            }

            // files

            if let Some(ref app_type) = details.app_type {
                state.serialize_field("app_type", app_type)?;
            }

            // split id

            if let Some(ref target_sdk_version) = details.target_sdk_version {
                state.serialize_field("target_sdk_version", target_sdk_version)?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableDocumentDetails<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let details = self.0;
            let mut state = serializer.serialize_struct("DocumentDetails", 1)?;

            if let Some(ref app_details) = details.app_details {
                state.serialize_field("app_details", &SerializableAppDetails(app_details))?;
            }

            state.end()
        }
    }

    impl<'a> serde::Serialize for SerializableOffer<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let offer = self.0;
            let mut state = serializer.serialize_struct("Offer", 1)?;

            if let Some(ref micros) = offer.micros {
                state.serialize_field("micros", micros)?;
            }

            if let Some(ref currency_code) = offer.currency_code {
                state.serialize_field("currency_code", currency_code)?;
            }

            if let Some(ref formatted_amount) = offer.formatted_amount {
                state.serialize_field("formatted_amount", formatted_amount)?;
            }

            if !offer.converted_price.is_empty() {
                let converted_prices: Vec<SerializableOffer> = offer
                    .converted_price
                    .iter()
                    .map(SerializableOffer)
                    .collect();
                state.serialize_field("converted_price", &converted_prices)?;
            }

            if let Some(ref checkout_flow_required) = offer.checkout_flow_required {
                state.serialize_field("checkout_flow_required", checkout_flow_required)?;
            }

            if let Some(ref full_price_micros) = offer.full_price_micros {
                state.serialize_field("full_price_micros", full_price_micros)?;
            }

            if let Some(ref formatted_full_amount) = offer.formatted_full_amount {
                state.serialize_field("formatted_full_amount", formatted_full_amount)?;
            }

            if let Some(ref offer_type) = offer.offer_type {
                state.serialize_field("offer_type", offer_type)?;
            }

            if let Some(ref on_sale_date) = offer.on_sale_date {
                state.serialize_field("on_sale_date", on_sale_date)?;
            }

            if !offer.promotion_label.is_empty() {
                let promotion_labels: Vec<String> = offer
                    .promotion_label
                    .iter()
                    .map(|label| label.to_string())
                    .collect();
                state.serialize_field("promotion_label", &promotion_labels)?;
            }

            if let Some(ref formatted_name) = offer.formatted_name {
                state.serialize_field("formatted_name", formatted_name)?;
            }

            if let Some(ref formatted_description) = offer.formatted_description {
                state.serialize_field("formatted_description", formatted_description)?;
            }

            if let Some(ref licensed_offer_type) = offer.licensed_offer_type {
                state.serialize_field("licensed_offer_type", licensed_offer_type)?;
            }

            //if let Some(ref subscription_content_terms) = offer.subscription_content_terms {
            //    state.serialize_field("subscription_content_terms", subscription_content_terms)?;
            //}

            if let Some(ref offer_id) = offer.offer_id {
                state.serialize_field("offer_id", offer_id)?;
            }

            if let Some(ref sale) = offer.sale {
                state.serialize_field("sale", sale)?;
            }

            if let Some(ref instant_purchase_enabled) = offer.instant_purchase_enabled {
                state.serialize_field("instant_purchase_enabled", instant_purchase_enabled)?;
            }

            if let Some(ref sale_message) = offer.sale_message {
                state.serialize_field("sale_message", sale_message)?;
            }

            state.end()
        }
    }
}

#[derive(Serialize)]
pub struct SerializableDetailsResponse(
    #[serde(with = "details_response_serde")] pub DetailsResponse,
);
