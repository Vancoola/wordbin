use leptos::prelude::*;

#[component]
pub fn DetailPanel() -> impl IntoView {
    view! {
        <aside class="detail">
            <div class="detail-header">
                <div class="detail-term">"ephemeral"</div>
                <div class="detail-pos">"adjective"</div>
                <div class="detail-pron">"/ɪˈfem(ə)r(ə)l/"</div>
            </div>

            <div class="detail-body">
                <div>
                    <div class="section-label">"Definition"</div>
                    <div class="detail-def">"Lasting for a very short time; transitory."</div>
                </div>
                <div>
                    <div class="section-label">"Example"</div>
                    <div class="detail-example">
                        "The ephemeral nature of tech trends makes it hard to predict what will last."
                    </div>
                </div>
                <div>
                    <div class="section-label">"Details"</div>
                    <div class="meta-row">
                        <span class="meta-key">"Added"</span>
                        <span class="meta-val">"May 14, 2026"</span>
                    </div>
                    <div class="meta-row">
                        <span class="meta-key">"Source"</span>
                        <span class="meta-val">"Browser extension"</span>
                    </div>
                    <div class="meta-row">
                        <span class="meta-key">"Status"</span>
                        <span class="meta-val" style="color:var(--stat-new);">"New"</span>
                    </div>
                    <div class="meta-row">
                        <span class="meta-key">"Reviews"</span>
                        <span class="meta-val">"0"</span>
                    </div>
                </div>
            </div>

            <div class="detail-footer">
                <button class="action-btn"><i class="ti ti-edit"></i>"Edit"</button>
                <button class="action-btn"><i class="ti ti-trash"></i>"Delete"</button>
                <button class="action-btn primary"><i class="ti ti-check"></i>"Review"</button>
            </div>
        </aside>
    }
}