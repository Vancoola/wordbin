use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <div class="logo">
                <div class="logo-icon"><i class="ti ti-books"></i></div>
                <span class="logo-text">"word"<span>"bin"</span></span>
            </div>

            <NavLink href="/" exact=true>
                <i class="ti ti-list"></i>"Words"
            </NavLink>
            <NavLink href="/review">
                <i class="ti ti-refresh"></i>"Review"
            </NavLink>

            <div class="nav-section">"Sources"</div>
            <div class="nav-item"><i class="ti ti-puzzle"></i>"Extension"</div>
            <div class="nav-item"><i class="ti ti-terminal"></i>"CLI"</div>
            <div class="nav-item"><i class="ti ti-api"></i>"API"</div>

            <div class="nav-section" style="margin-top: 4px;">"App"</div>
            <NavLink href="/settings">
                <i class="ti ti-settings"></i>"Settings"
            </NavLink>

            <div class="stats-block">
                <div class="stat-row">
                    <span class="stat-label">"Total words"</span>
                    <span class="stat-val">"247"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"To review"</span>
                    <span class="stat-val" style="color:var(--stat-review);">"12"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-label">"Learned"</span>
                    <span class="stat-val" style="color:var(--stat-learned);">"89"</span>
                </div>
            </div>
        </aside>
    }
}

#[component]
pub fn NavLink(
    href: &'static str,
    #[prop(optional)] exact: bool,
    children: Children,
) -> impl IntoView {
    let location = use_location();

    let class = move || {
        let current = location.pathname.get();
        let active = if exact {
            current == href
        } else {
            current == href || current.starts_with(&format!("{href}/"))
        };
        if active {
            "nav-item active"
        } else {
            "nav-item"
        }
    };

    view! {
        <A href=href attr:class=class>
            {children()}
        </A>
    }
}
