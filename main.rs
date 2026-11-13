mod products;
mod tracking;

use leptos::*;
use products::{ProductItem, ALL_PRODUCTS};
use tracking::{
    fire_plausible_event, js_init_buy_buttons, load_cart_logs, TrackingState,
};

fn main() {
    console_error_panic_hook::set_once();
    let initial_state = TrackingState::capture_from_browser();
    fire_plausible_event(
        "Landing Page Loaded",
        &[
            ("sca_ref", initial_state.sca_ref.as_deref().unwrap_or("none")),
            (
                "utm_source",
                initial_state.utm_source.as_deref().unwrap_or("direct"),
            ),
        ],
    );
    mount_to_body(|| view! { <App/> });
}

#[component]
fn ProductCard(item: &'static ProductItem) -> impl IntoView {
    let theme_str = if item.is_light_card { "light" } else { "dark" };
    let tag_cls = if item.badge_class.is_empty() {
        "snogo-tag".to_string()
    } else {
        format!("snogo-tag {}", item.badge_class)
    };

    view! {
        <div class="snogo-card">
            <div>
                <div class="snogo-card-top">
                    <div class="snogo-card-icon">
                        <i class=item.icon></i>
                    </div>
                    <span class=tag_cls>{item.badge}</span>
                </div>
                <h3>{item.title}</h3>
                <p>{item.description}</p>
                {if !item.swatches.is_empty() {
                    view! {
                        <div class="snogo-swatches">
                            <span class="snogo-swatch-label">"Finishes:"</span>
                            {item.swatches.iter().map(|s| {
                                let cls = format!("swatch-dot {}", s);
                                view! { <span class=cls></span> }
                            }).collect_view()}
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </div>
            <div
                class="snogo-shopify-mount"
                data-handle=item.handle
                data-theme=theme_str
            ></div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let initial_tracking = TrackingState::capture_from_browser();
    let (tracking_state, set_tracking_state) = create_signal(initial_tracking.clone());
    let (discount_input, set_discount_input) = create_signal(
        initial_tracking
            .discount_code
            .clone()
            .unwrap_or_default(),
    );
    let (discount_status, set_discount_status) = create_signal(
        initial_tracking
            .discount_code
            .as_ref()
            .map(|c| format!("✓ Code "{}" active for checkout", c.to_uppercase()))
            .unwrap_or_default(),
    );
    let (acc_filter, set_acc_filter) = create_signal("all".to_string());
    let (show_telemetry, set_show_telemetry) = create_signal(false);
    let (cart_logs, set_cart_logs) = create_signal(load_cart_logs());

    // Initialize Shopify Buy Buttons once DOM nodes are mounted
    create_effect(move |_| {
        js_init_buy_buttons();
    });

    let on_apply_discount = move |_| {
        let trimmed = discount_input.get().trim().to_string();
        let mut st = tracking_state.get();
        if trimmed.is_empty() {
            st.discount_code = None;
            st.save();
            set_tracking_state.set(st);
            set_discount_status.set("Code cleared.".to_string());
        } else {
            st.discount_code = Some(trimmed.clone());
            st.save();
            set_tracking_state.set(st);
            set_discount_status.set(format!(
                "✓ Code "{}" applied to checkout!",
                trimmed.to_uppercase()
            ));
            fire_plausible_event("Discount Code Applied", &[("discount", &trimmed)]);
        }
    };

    let hero_left = ALL_PRODUCTS
        .iter()
        .find(|p| p.category == "hero_left")
        .expect("hero_left product");
    let hero_right = ALL_PRODUCTS
        .iter()
        .find(|p| p.category == "hero_right")
        .expect("hero_right product");

    let straws: Vec<&'static ProductItem> = ALL_PRODUCTS
        .iter()
        .filter(|p| p.category == "straw")
        .collect();
    let necklaces: Vec<&'static ProductItem> = ALL_PRODUCTS
        .iter()
        .filter(|p| p.category == "necklace")
        .collect();
    let accessories: Vec<&'static ProductItem> = ALL_PRODUCTS
        .iter()
        .filter(|p| {
            matches!(
                p.category,
                "wear" | "carry" | "maintain" | "enhance"
            )
        })
        .collect();
    let bundles: Vec<&'static ProductItem> = ALL_PRODUCTS
        .iter()
        .filter(|p| p.category == "bundle")
        .collect();

    view! {
        <div class="snogo-landing-wrapper">
            // --- TOP PROMO BAR ---
            <div class="snogo-topbar">
                <div>
                    <a href="https://snogocacao.myshopify.com/account/login">
                        <i class="fa-solid fa-angles-right"></i>" Store Login"
                    </a>
                </div>
                <div class="snogo-typing-wrap">
                    <span>"Free Shipping On "</span>
                    <span class="snogo-typing-dynamic">"US Orders over $250 / Intl over $500"</span>
                    <span>" - Save!"</span>
                </div>
                <div style="display:flex; gap:14px; align-items:center;">
                    <button
                        type="button"
                        class="snogo-telemetry-toggle"
                        on:click=move |_| {
                            set_cart_logs.set(load_cart_logs());
                            set_show_telemetry.update(|v| *v = !*v);
                        }
                    >
                        <i class="fa-solid fa-satellite-dish"></i>" Affiliate & UTM Status"
                    </button>
                    <a href="mailto:help@snogostraws.com">
                        <i class="fa-solid fa-envelope"></i>" Get In Touch"
                    </a>
                </div>
            </div>

            // --- AFFILIATE & DISCOUNT CODE BAR ---
            <div class="snogo-discount-bar">
                <span>
                    <i class="fa-solid fa-tag" style="color:var(--snogo-cyan);"></i>
                    " Have a Promo / Affiliate Discount Code?"
                </span>
                <input
                    type="text"
                    class="snogo-discount-input"
                    placeholder="ENTER CODE"
                    prop:value=discount_input
                    on:input=move |ev| set_discount_input.set(event_target_value(&ev))
                />
                <button
                    type="button"
                    class="snogo-discount-btn"
                    on:click=on_apply_discount
                >
                    "Apply to Checkout"
                </button>
                <span class="snogo-discount-status">{move || discount_status.get()}</span>
            </div>

            // --- COLLAPSIBLE RUST TELEMETRY DRAWER (AFFILIATE, UTM, CART LOGS) ---
            {move || if show_telemetry.get() {
                let st = tracking_state.get();
                let logs = cart_logs.get();
                view! {
                    <div class="snogo-telemetry-panel">
                        <div class="snogo-telemetry-inner">
                            <div>
                                <h4><i class="fa-solid fa-link"></i>" Captured Affiliate & UTM Parameters (Rust State)"</h4>
                                <ul class="snogo-telemetry-list">
                                    <li><strong>"UpPromote Ref (sca_ref): "</strong>{st.sca_ref.unwrap_or_else(|| "None".into())}</li>
                                    <li><strong>"Discount Code: "</strong>{st.discount_code.unwrap_or_else(|| "None".into())}</li>
                                    <li><strong>"utm_source: "</strong>{st.utm_source.unwrap_or_else(|| "None".into())}</li>
                                    <li><strong>"utm_medium: "</strong>{st.utm_medium.unwrap_or_else(|| "None".into())}</li>
                                    <li><strong>"utm_campaign: "</strong>{st.utm_campaign.unwrap_or_else(|| "None".into())}</li>
                                    <li><strong>"utm_content / term: "</strong>{format!("{:?} / {:?}", st.utm_content, st.utm_term)}</li>
                                </ul>
                            </div>
                            <div>
                                <h4><i class="fa-solid fa-cart-shopping"></i>" Recent Cart & Checkout Telemetry Logs"</h4>
                                {if logs.is_empty() {
                                    view! { <p style="font-size:12px; opacity:0.7;">"No cart events logged yet. Add a product to cart to see live Rust/WASM telemetry."</p> }.into_view()
                                } else {
                                    view! {
                                        <ul class="snogo-telemetry-list">
                                            {logs.into_iter().take(6).map(|l| {
                                                view! {
                                                    <li>
                                                        <span style="color:var(--snogo-cyan);">"["{l.event_type}"] "</span>
                                                        {l.detail}" (Subtotal: $"{l.subtotal}")"
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    }.into_view()
                                }}
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! { <div></div> }.into_view()
            }}

            // --- STICKY NAVIGATION HEADER ---
            <header class="snogo-header">
                <div class="snogo-socials">
                    <a href="https://www.facebook.com/SnogoJewelry" target="_blank" aria-label="Facebook"><i class="fa-brands fa-facebook-f"></i></a>
                    <a href="https://www.instagram.com/snogostraws/" target="_blank" aria-label="Instagram"><i class="fa-brands fa-instagram"></i></a>
                    <a href="https://www.youtube.com/@snogostraws" target="_blank" aria-label="YouTube"><i class="fa-brands fa-youtube"></i></a>
                    <a href="https://www.tiktok.com/@snogolife" target="_blank" aria-label="TikTok"><i class="fa-brands fa-tiktok"></i></a>
                    <a href="https://twitter.com/snogostraws" target="_blank" aria-label="X"><i class="fa-brands fa-x-twitter"></i></a>
                </div>

                <a href="#" class="snogo-brand">
                    <i class="fa-solid fa-wand-magic-sparkles"></i>" SNOGO"
                </a>

                <nav class="snogo-nav">
                    <a href="#new-arrivals">"New"</a>
                    <a href="#straw-collection">"Straws"</a>
                    <a href="#necklace-collection">"Necklaces"</a>
                    <a href="#accessories-collection">"Accessories"</a>
                    <a href="#bundles-collection">"Bundles"</a>
                </nav>
            </header>

            // --- DUAL HERO SPLIT ---
            <section class="snogo-hero-grid" id="new-arrivals">
                <div class="snogo-hero-panel snogo-hero-noir">
                    <span class="snogo-hero-badge badge-cyan">
                        <i class="fa-regular fa-gem"></i>" "{hero_left.badge}
                    </span>
                    <div class="snogo-hero-icon" style="color: var(--snogo-cyan);">
                        <i class=hero_left.icon></i>
                    </div>
                    <h1>{hero_left.title}</h1>
                    <p>{hero_left.description}</p>
                    <div
                        class="snogo-shopify-mount"
                        data-handle=hero_left.handle
                        data-theme="dark"
                        style="max-width:320px; border:none;"
                    ></div>
                </div>

                <div class="snogo-hero-panel snogo-hero-gogo">
                    <span class="snogo-hero-badge badge-pink">
                        <i class="fa-solid fa-bolt"></i>" "{hero_right.badge}
                    </span>
                    <div class="snogo-hero-icon" style="color: var(--snogo-pink);">
                        <i class=hero_right.icon></i>
                    </div>
                    <h2>{hero_right.title}</h2>
                    <p>{hero_right.description}</p>
                    <div
                        class="snogo-shopify-mount"
                        data-handle=hero_right.handle
                        data-theme="dark"
                        style="max-width:320px; border:none;"
                    ></div>
                </div>
            </section>

            // --- SECTION 1: THE STRAW COLLECTION ---
            <section class="snogo-section snogo-section-light" id="straw-collection">
                <div class="snogo-section-header">
                    <h2>"The Straw Collection"</h2>
                    <p>"Select your finish and add directly to your cart. Affiliate tracking, UTMs, and promo codes push automatically to Shopify checkout."</p>
                </div>
                <div class="snogo-grid">
                    {straws.into_iter().map(|item| view! { <ProductCard item=item/> }).collect_view()}
                </div>
            </section>

            // --- SECTION 2: NECKLACE COLLECTION ---
            <section class="snogo-section" id="necklace-collection" style="border-bottom: 1px solid rgba(255,255,255,0.1);">
                <div class="snogo-section-header">
                    <h2>"Designer Straw Necklaces"</h2>
                    <p>"Transform your Snogo into statement fine jewelry. Available in Thick and Thin chains engineered to lock your straw securely in place."</p>
                </div>
                <div class="snogo-grid">
                    {necklaces.into_iter().map(|item| view! { <ProductCard item=item/> }).collect_view()}
                </div>
            </section>

            // --- SECTION 3: ACCESSORIES ---
            <section class="snogo-section" id="accessories-collection">
                <div class="snogo-section-header">
                    <h2>"Snogo Accessories"</h2>
                    <p>"Filter by how you move: Wear It, Carry It, Maintain It, or Enhance It."</p>
                </div>

                <div class="snogo-tabs">
                    <button
                        class=move || if acc_filter.get() == "all" { "snogo-tab-btn active" } else { "snogo-tab-btn" }
                        on:click=move |_| set_acc_filter.set("all".into())
                    >
                        <i class="fa-solid fa-border-all"></i>" All Accessories"
                    </button>
                    <button
                        class=move || if acc_filter.get() == "wear" { "snogo-tab-btn active" } else { "snogo-tab-btn" }
                        on:click=move |_| set_acc_filter.set("wear".into())
                    >
                        <i class="fa-solid fa-hat-cowboy"></i>" Wear It"
                    </button>
                    <button
                        class=move || if acc_filter.get() == "carry" { "snogo-tab-btn active" } else { "snogo-tab-btn" }
                        on:click=move |_| set_acc_filter.set("carry".into())
                    >
                        <i class="fa-solid fa-wallet"></i>" Carry It"
                    </button>
                    <button
                        class=move || if acc_filter.get() == "maintain" { "snogo-tab-btn active" } else { "snogo-tab-btn" }
                        on:click=move |_| set_acc_filter.set("maintain".into())
                    >
                        <i class="fa-solid fa-broom"></i>" Maintain It"
                    </button>
                    <button
                        class=move || if acc_filter.get() == "enhance" { "snogo-tab-btn active" } else { "snogo-tab-btn" }
                        on:click=move |_| set_acc_filter.set("enhance".into())
                    >
                        <i class="fa-solid fa-gift"></i>" Enhance It"
                    </button>
                </div>

                <div class="snogo-grid">
                    {accessories.into_iter().map(|item| {
                        let cat = item.category;
                        view! {
                            <div
                                style=move || {
                                    let f = acc_filter.get();
                                    if f == "all" || f == cat {
                                        "display:flex;"
                                    } else {
                                        "display:none;"
                                    }
                                }
                            >
                                <ProductCard item=item/>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            // --- SECTION 4: CURATED BUNDLES ---
            <section class="snogo-section snogo-section-light" id="bundles-collection">
                <div class="snogo-section-header">
                    <h2>"Curated Snogo Bundles"</h2>
                    <p>"Complete party-ready kits pairing our best-selling straws, necklaces, keychains, and maintenance tools at exclusive savings."</p>
                </div>
                <div class="snogo-grid">
                    {bundles.into_iter().map(|item| view! { <ProductCard item=item/> }).collect_view()}
                </div>
            </section>

            // --- FOOTER ---
            <footer class="snogo-footer">
                <div class="snogo-footer-grid">
                    <div>
                        <div class="snogo-brand" style="margin-bottom: 16px;">
                            <i class="fa-solid fa-wand-magic-sparkles"></i>" SNOGO"
                        </div>
                        <p style="font-size:14px; color:#94a3b8;">
                            "The Ultimate Designer Cacao Bump Straw. Precision-crafted in Boulder, CO to take the party wherever you go."
                        </p>
                    </div>
                    <div>
                        <h4>"Collections"</h4>
                        <ul>
                            <li><a href="#straw-collection">"Straw Collection"</a></li>
                            <li><a href="#necklace-collection">"Necklaces"</a></li>
                            <li><a href="#accessories-collection">"Wear It / Carry It / Maintain It"</a></li>
                            <li><a href="#bundles-collection">"Curated Bundles"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4>"Telemetry & Attribution"</h4>
                        <ul>
                            <li><a href="#" on:click=move |e| { e.prevent_default(); set_cart_logs.set(load_cart_logs()); set_show_telemetry.set(true); }>"Inspect Affiliate & UTM State"</a></li>
                            <li><a href="https://snogocacao.myshopify.com/account/login">"Customer Portal"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h4>"Support"</h4>
                        <ul>
                            <li><a href="mailto:help@snogostraws.com">"Help@SnogoStraws.com"</a></li>
                        </ul>
                    </div>
                </div>
                <div style="text-align:center; padding-top:24px; border-top:1px solid rgba(255,255,255,0.1); font-size:13px; color:#64748b;">
                    "© 2026 Snogo Straws. All Rights Reserved. | 1601 29th St. Suite 1292, #1148, Boulder, CO 80301"
                </div>
            </footer>
        </div>
    }
}
