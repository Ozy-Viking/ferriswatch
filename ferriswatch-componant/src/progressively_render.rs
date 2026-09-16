#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

pub(crate) const INITIAL_OPTIONS: usize = 4;

#[cfg(target_arch = "wasm32")]
pub(crate) fn progressively_render_options(render_count: Signal<usize>, total: usize) {
    use web_sys::wasm_bindgen::{JsCast, closure::Closure};

    fn schedule_step(mut render_count: Signal<usize>, total: usize, current: usize) {
        use web_sys::wasm_bindgen::{JsCast, closure::Closure};

        let callback = Closure::once(move || {
            if current >= total {
                return;
            }

            let next = (current * 2).min(total);

            render_count.set(next);

            schedule_step(render_count, total, next);
        });

        web_sys::window()
            .unwrap()
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .unwrap();

        callback.forget();
    }

    // One RAF first so the initial 8 actually get a paint opportunity
    // before we start increasing the collection.
    let first_frame = Closure::once(move || {
        schedule_step(render_count, total, INITIAL_OPTIONS.min(total));
    });

    web_sys::window()
        .unwrap()
        .request_animation_frame(first_frame.as_ref().unchecked_ref())
        .unwrap();

    first_frame.forget();
}
