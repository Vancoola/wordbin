use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
            <Stylesheet href="/home.css" />
    <div class="app">

      <aside class="sidebar">
        <div class="logo">
          <div class="logo-icon"><i class="ti ti-books"></i></div>
          <span class="logo-text">word<span>bin</span></span>
        </div>

        <div class="nav-item active" onclick="setNav(this)"><i class="ti ti-list"></i>Words</div>
        <div class="nav-item" onclick="setNav(this)"><i class="ti ti-refresh"></i>Review</div>

        <div class="nav-section">Sources</div>
        <div class="nav-item" onclick="setNav(this)"><i class="ti ti-puzzle"></i>Extension</div>
        <div class="nav-item" onclick="setNav(this)"><i class="ti ti-terminal"></i>CLI</div>
        <div class="nav-item" onclick="setNav(this)"><i class="ti ti-api"></i>API</div>

        <div class="nav-section" style="margin-top: 4px;">App</div>
        <div class="nav-item" onclick="setNav(this)"><i class="ti ti-settings"></i>Settings</div>

        <div class="stats-block">
          <div class="stat-row"><span class="stat-label">Total words</span><span class="stat-val">247</span></div>
          <div class="stat-row"><span class="stat-label">To review</span><span class="stat-val" style="color:var(--stat-review);">12</span></div>
          <div class="stat-row"><span class="stat-label">Learned</span><span class="stat-val" style="color:var(--stat-learned);">89</span></div>
        </div>
      </aside>

      <main class="main">
        <div class="topbar">
          <div class="search-wrap">
            <i class="ti ti-search"></i>
            <input type="text" placeholder="Search words…" aria-label="Search words" oninput="filterWords(this.value)" />
          </div>
          <button class="add-btn"><i class="ti ti-plus"></i>Add word</button>
          <button onclick="toggleTheme()" style="background:transparent;border:0.5px solid var(--border-mid);border-radius:var(--radius-md);padding:7px 10px;cursor:pointer;color:var(--text-secondary);display:flex;align-items:center;" title="Toggle theme" aria-label="Toggle theme"><i class="ti ti-moon" id="theme-icon"></i></button>
        </div>

        <div class="filter-bar">
          <div class="chip active" onclick="setChip(this, 'all')">All</div>
          <div class="chip" onclick="setChip(this, 'new')">New</div>
          <div class="chip" onclick="setChip(this, 'review')">Review</div>
          <div class="chip" onclick="setChip(this, 'learned')">Learned</div>
          <span class="filter-count" id="count-label">247 words</span>
        </div>

        <div class="word-list" id="word-list" role="list"></div>
      </main>

      <aside class="detail">
        <div class="detail-header">
          <div class="detail-term" id="d-term">ephemeral</div>
          <div class="detail-pos"  id="d-pos">adjective</div>
          <div class="detail-pron" id="d-pron">/ɪˈfem(ə)r(ə)l/</div>
        </div>
        <div class="detail-body">
          <div>
            <div class="section-label">Definition</div>
            <div class="detail-def" id="d-def">Lasting for a very short time; transitory.</div>
          </div>
          <div>
            <div class="section-label">Example</div>
            <div class="detail-example" id="d-ex">The ephemeral nature of tech trends makes it hard to predict what will last.</div>
          </div>
          <div>
            <div class="section-label">Details</div>
            <div class="meta-row"><span class="meta-key">Added</span><span class="meta-val" id="d-date">May 14, 2026</span></div>
            <div class="meta-row"><span class="meta-key">Source</span><span class="meta-val" id="d-source">Browser extension</span></div>
            <div class="meta-row"><span class="meta-key">Status</span><span class="meta-val" id="d-status" style="color:var(--stat-new);">New</span></div>
            <div class="meta-row"><span class="meta-key">Reviews</span><span class="meta-val" id="d-reviews">0</span></div>
          </div>
        </div>
        <div class="detail-footer">
          <button class="action-btn"><i class="ti ti-edit"></i>Edit</button>
          <button class="action-btn"><i class="ti ti-trash"></i>Delete</button>
          <button class="action-btn primary"><i class="ti ti-check"></i>Review</button>
        </div>
      </aside>

    </div>
        }
}
