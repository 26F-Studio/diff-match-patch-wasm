mod utils;
use diff_match_patch_rs::{Efficient, Ops, dmp::DiffMatchPatch};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Default)]
pub struct Differ {
    dmp: DiffMatchPatch,
}

#[wasm_bindgen]
impl Differ {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let dmp = DiffMatchPatch::default();
        Self { dmp }
    }

    pub fn diff_main(&self, old: &str, new: &str) -> Result<JsValue, String> {
        let diffs = match self.dmp.diff_main::<Efficient>(old, new) {
            Ok(d) => d,
            Err(_) => return Err("error while diffing".to_string()),
        };
        let serializable_diffs: Vec<(i32, String)> = diffs
            .iter()
            .map(|diff| {
                let mapped_op = match diff.op() {
                    Ops::Delete => -1,
                    Ops::Equal => 0,
                    Ops::Insert => 1,
                };
                let text = diff.to_string();
                // let text = String::from_utf8_lossy(diff.data()).into_owned();
                (mapped_op, text)
            })
            .collect();
        to_value(&serializable_diffs).map_err(|e| e.to_string())
    }
}
