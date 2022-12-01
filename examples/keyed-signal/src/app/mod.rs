mod world_time;

use self::world_time::{WorldTime, WorldTimeParams};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_signals::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());

    let clear_cache = |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            storage.clear().unwrap();
        }
    };
    view! {
        cx,
        <div>
            <Router>
              <h1>"UTC offset for a Timezone"</h1>
              <div>"Try alternating between the two links to see how values are fetched or retrieved from the cache and clear the cache manually with the button."</div>
              <A href="/America/Bogota">"America/Bogota"</A>
              <A href="/Europe/London">"Europe/London"</A>
              <Content />
              <button on:click=clear_cache>"Clear cache"</button>
            </Router>
        </div>
    }
}

#[component]
pub fn Content(cx: Scope) -> Element {
    view! {
        cx,
        <div class="content">
          <Routes>
            <Route path="/:area/:location" element=move |cx| {
                let key = WorldTimeParams::new(use_params_map(cx));
                view! { cx, <TimeZone key /> }
            } />
            <Route path="" element=move |_cx| {
                view! { _cx, <h1 class="error">"Invalid route"</h1> }
            } />
          </Routes>
        </div>
    }
}

#[component]
pub fn TimeZone(cx: Scope, key: WorldTimeParams) -> Element {
    let key = create_rw_signal(cx, key);

    let params = use_params_map(cx);
    create_effect(cx, move |_| key.set(WorldTimeParams::new(params)));

    // Will be updated if the key changed.
    let world_time = create_keyed_signal(cx, key, |key, value| async move {
        cache_or_fetch(key, value).await
    });

    view! { cx, <h2>{move || world_time.get()}</h2> }
}

#[allow(dead_code)]
async fn cache_or_fetch(key: WorldTimeParams, value: WriteSignal<String>) {
    let storage = match window().local_storage() {
        Ok(Some(s)) => s,
        _ => {
            return log!("No storage found");
        }
    };
    let key_json = serde_json::to_string(&key).unwrap();
    if let Ok(Some(val)) = storage.get(&key_json) {
        value.set(format!("[cache] {val}"));
    } else {
        value.set("fetching...".to_string());
        let res = match WorldTime::fetch(&key).await {
            Ok(wt) => wt.to_string(),
            Err(e) => return log!("Error {e}"),
        };

        storage.set(&key_json, &res).unwrap();
        value.set(format!("[fetch] {res}"));
    }
}
