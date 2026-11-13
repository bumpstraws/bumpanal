use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use web_sys::{window, Url, UrlSearchParams};

const STORAGE_KEY_TRACKING: &str = "snogo_rust_tracking_state";
const STORAGE_KEY_CART_LOGS: &str = "snogo_rust_cart_logs";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = snogoTrackPlausible)]
    fn js_track_plausible(event_name: &str, props_json: &str);

    #[wasm_bindgen(js_namespace = window, js_name = snogoGetUpPromoteLinker)]
    fn js_get_uppromote_linker() -> String;

    #[wasm_bindgen(js_namespace = window, js_name = snogoInitBuyButtons)]
    pub fn js_init_buy_buttons();
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TrackingState {
    pub sca_ref: Option<String>,
    pub discount_code: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_term: Option<String>,
    pub utm_content: Option<String>,
    pub fbclid: Option<String>,
    pub gclid: Option<String>,
    pub ttclid: Option<String>,
    pub landing_page: Option<String>,
    pub referrer: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CartEventLog {
    pub event_type: String,
    pub detail: String,
    pub cart_id: String,
    pub subtotal: String,
    pub timestamp_iso: String,
}

impl TrackingState {
    /// Load saved tracking state from LocalStorage and merge any fresh URL parameters
    pub fn capture_from_browser() -> Self {
        let mut state: TrackingState = LocalStorage::get(STORAGE_KEY_TRACKING).unwrap_or_default();

        if let Some(win) = window() {
            if let Ok(href) = win.location().href() {
                if state.landing_page.is_none() {
                    state.landing_page = Some(href.clone());
                }
            }
            if let Some(doc) = win.document() {
                let r = doc.referrer();
                if !r.is_empty() && state.referrer.is_none() {
                    state.referrer = Some(r);
                }
            }
            if let Ok(search) = win.location().search() {
                if let Ok(params) = UrlSearchParams::new_with_str(&search) {
                    if let Some(v) = params.get("sca_ref").or_else(|| params.get("ref")) {
                        if !v.trim().is_empty() {
                            state.sca_ref = Some(v.trim().to_string());
                            let _ = LocalStorage::set("uppromote_sca_ref", v.trim());
                        }
                    }
                    if let Some(v) = params
                        .get("discount")
                        .or_else(|| params.get("code"))
                        .or_else(|| params.get("coupon"))
                    {
                        if !v.trim().is_empty() {
                            state.discount_code = Some(v.trim().to_string());
                            let _ = LocalStorage::set("uppromote_discount_code", v.trim());
                        }
                    }
                    if let Some(v) = params.get("utm_source") {
                        state.utm_source = Some(v);
                    }
                    if let Some(v) = params.get("utm_medium") {
                        state.utm_medium = Some(v);
                    }
                    if let Some(v) = params.get("utm_campaign") {
                        state.utm_campaign = Some(v);
                    }
                    if let Some(v) = params.get("utm_term") {
                        state.utm_term = Some(v);
                    }
                    if let Some(v) = params.get("utm_content") {
                        state.utm_content = Some(v);
                    }
                    if let Some(v) = params.get("fbclid") {
                        state.fbclid = Some(v);
                    }
                    if let Some(v) = params.get("gclid") {
                        state.gclid = Some(v);
                    }
                    if let Some(v) = params.get("ttclid") {
                        state.ttclid = Some(v);
                    }
                }
            }
        }

        state.save();
        state
    }

    pub fn save(&self) {
        let _ = LocalStorage::set(STORAGE_KEY_TRACKING, self);
        if let Some(ref sca) = self.sca_ref {
            let _ = LocalStorage::set("uppromote_sca_ref", sca);
        }
        if let Some(ref disc) = self.discount_code {
            let _ = LocalStorage::set("uppromote_discount_code", disc);
        } else {
            LocalStorage::delete("uppromote_discount_code");
        }
    }

    pub fn to_props_map(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        if let Some(ref v) = self.sca_ref {
            map.insert("sca_ref".into(), v.clone());
        }
        if let Some(ref v) = self.discount_code {
            map.insert("discount".into(), v.clone());
        }
        if let Some(ref v) = self.utm_source {
            map.insert("utm_source".into(), v.clone());
        }
        if let Some(ref v) = self.utm_medium {
            map.insert("utm_medium".into(), v.clone());
        }
        if let Some(ref v) = self.utm_campaign {
            map.insert("utm_campaign".into(), v.clone());
        }
        if let Some(ref v) = self.utm_term {
            map.insert("utm_term".into(), v.clone());
        }
        if let Some(ref v) = self.utm_content {
            map.insert("utm_content".into(), v.clone());
        }
        map
    }
}

pub fn fire_plausible_event(event_name: &str, extra_props: &[(&str, &str)]) {
    let state = TrackingState::capture_from_browser();
    let mut props = state.to_props_map();
    for (k, v) in extra_props {
        props.insert((*k).to_string(), (*v).to_string());
    }
    let json = serde_json::to_string(&props).unwrap_or_else(|_| "{}".to_string());
    js_track_plausible(event_name, &json);
}

pub fn load_cart_logs() -> Vec<CartEventLog> {
    LocalStorage::get(STORAGE_KEY_CART_LOGS).unwrap_or_default()
}

pub fn append_cart_log(event_type: &str, detail: &str, cart_id: &str, subtotal: &str) {
    let mut logs = load_cart_logs();
    let now_str = js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default();
    logs.insert(
        0,
        CartEventLog {
            event_type: event_type.to_string(),
            detail: detail.to_string(),
            cart_id: cart_id.to_string(),
            subtotal: subtotal.to_string(),
            timestamp_iso: now_str,
        },
    );
    if logs.len() > 25 {
        logs.truncate(25);
    }
    let _ = LocalStorage::set(STORAGE_KEY_CART_LOGS, &logs);
}

/// Exposed to JavaScript: Decorates a Shopify Checkout URL with UpPromote `_upl`, `sca_ref`, `discount`, and all `utm_*` parameters
#[wasm_bindgen]
pub fn rust_decorate_checkout_url(raw_checkout_url: &str) -> String {
    let state = TrackingState::capture_from_browser();

    let Ok(url) = Url::new(raw_checkout_url) else {
        return raw_checkout_url.to_string();
    };
    let params = url.search_params();

    // 1. UpPromote Cross-Domain Linker (_upl)
    let linker = js_get_uppromote_linker();
    if !linker.is_empty() {
        params.set("_upl", &linker);
    }

    // 2. UpPromote Affiliate Ref (sca_ref)
    if let Some(ref sca) = state.sca_ref {
        params.set("sca_ref", sca);
    }

    // 3. Discount / Promo Code (discount)
    if let Some(ref disc) = state.discount_code {
        params.set("discount", disc);
    }

    // 4. All UTM & Ad Click IDs
    if let Some(ref v) = state.utm_source {
        params.set("utm_source", v);
    }
    if let Some(ref v) = state.utm_medium {
        params.set("utm_medium", v);
    }
    if let Some(ref v) = state.utm_campaign {
        params.set("utm_campaign", v);
    }
    if let Some(ref v) = state.utm_term {
        params.set("utm_term", v);
    }
    if let Some(ref v) = state.utm_content {
        params.set("utm_content", v);
    }
    if let Some(ref v) = state.fbclid {
        params.set("fbclid", v);
    }
    if let Some(ref v) = state.gclid {
        params.set("gclid", v);
    }
    if let Some(ref v) = state.ttclid {
        params.set("ttclid", v);
    }

    let final_url = url.href();
    append_cart_log("checkout_redirect", &final_url, "checkout", "");
    fire_plausible_event("Shopify Checkout Redirect", &[("checkout_url", &final_url)]);
    final_url
}

/// Exposed to JavaScript: Called whenever Shopify Buy Button updates/renders the cart
#[wasm_bindgen]
pub fn rust_log_cart_event(event_type: &str, cart_id: &str, item_summary: &str, subtotal: &str) {
    append_cart_log(event_type, item_summary, cart_id, subtotal);
    fire_plausible_event(
        "Shopify Cart Updated",
        &[
            ("event_type", event_type),
            ("cart_id", cart_id),
            ("items", item_summary),
            ("subtotal", subtotal),
        ],
    );
}
