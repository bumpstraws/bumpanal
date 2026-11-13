# Snogo Straws — Rust + WebAssembly Landing Page

A fast, image-free **Rust (Leptos CSR + WebAssembly)** storefront landing page for **Snogo Straws** (`snogocacao.myshopify.com`).

## What This Repo Does
1. **100% Rust Component & State Architecture (`src/`):**
   - Renders the entire Snogo Straws catalog (Straws, Necklaces, Wear It / Carry It / Maintain It / Enhance It Accessories, and Curated Bundles) using **Leptos** compiled to WebAssembly.
   - Zero product photography (`img: false` in Shopify Buy Button) — uses pure CSS metallic finish swatches and FontAwesome 6 icons.
2. **Automatic Affiliate, Discount & UTM Capture (`src/tracking.rs`):**
   - Automatically captures and persists `sca_ref` (UpPromote Affiliate ID), `discount` / `code` / `coupon` (Promo Code), `utm_source`, `utm_medium`, `utm_campaign`, `utm_term`, `utm_content`, `fbclid`, `gclid`, and `ttclid` in browser `localStorage`.
   - Includes a live **Affiliate & UTM Status** telemetry drawer in the top bar so you can verify captured attribution and cart item logs at any time.
3. **Plausible Analytics Integration (`index.html` + `src/tracking.rs`):**
   - Fires tagged custom events (`Landing Page Loaded`, `Discount Code Applied`, `Shopify Cart Updated`, and `Shopify Checkout Redirect`) along with affiliate and UTM properties directly to Plausible.
4. **Shopify Buy Button Checkout Passthrough:**
   - When a customer clicks **Checkout** inside the slide-out Shopify Buy Button cart, Rust (`rust_decorate_checkout_url`) intercepts the Shopify Checkout URL and appends:
     - `_upl` (UpPromote Cross-Domain Linker)
     - `sca_ref` (UpPromote Affiliate Reference)
     - `discount` (Active Promo / Affiliate Discount Code)
     - All `utm_*` and ad click IDs

---

## Repository Structure
```text
snogo-rust-landing/
├── .github/
│   └── workflows/
│       └── deploy.yml     # GitHub Actions CI/CD to build Rust WASM & deploy to GitHub Pages
├── src/
│   ├── main.rs            # Leptos UI components, category filters, and telemetry drawer
│   ├── products.rs        # Strongly-typed catalog mapped to exact Shopify product handles
│   └── tracking.rs        # Rust UTM/Affiliate localStorage persistence, cart logger & URL decorator
├── .gitignore
├── Cargo.toml             # Rust crate dependencies (leptos, wasm-bindgen, web-sys, serde)
├── Trunk.toml             # Trunk WASM bundler configuration
├── index.html             # Head scripts (UpPromote, Plausible, Fonts) & JS<->WASM bridge
└── styles.css             # Snogo dark/light luxury theme & animations
```

---

## Local Development (Quickstart)

1. **Install Rust & the WebAssembly target:**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Install Trunk (the Rust WASM bundler):**
   ```bash
   cargo install trunk
   ```

3. **Run the local dev server with hot-reloading:**
   ```bash
   trunk serve --open
   ```
   Visit `http://localhost:8080/?sca_ref=TEST_AFFILIATE&discount=SNOGO10&utm_source=instagram&utm_campaign=launch` and click **"Affiliate & UTM Status"** in the top right bar to inspect live Rust attribution capture.

---

## Deploying to GitHub Pages / Cloudflare Pages / Netlify

- **GitHub Pages (Automated):**
  1. Push all files in this repository to a new GitHub repo on the `main` branch.
  2. In your GitHub repo, go to **Settings > Pages** and set **Source** to **GitHub Actions**.
  3. Every push to `main` automatically runs `.github/workflows/deploy.yml`, compiles the Rust code to optimized WebAssembly (`trunk build --release`), and publishes the static `dist/` bundle.
