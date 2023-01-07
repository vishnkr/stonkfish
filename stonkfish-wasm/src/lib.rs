use wasm_bindgen::prelude::*;
use serde::Serialize;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, stonkfish wasm"));
}

#[wasm_bindgen]
pub struct Stonkfish{
    engine: stonkfish::Engine
}

#[derive(Serialize,Debug)]
struct MoveJSON{
    src: u8,
    dest: u8,
    mtype: String,
}

#[wasm_bindgen]
impl Stonkfish{
    pub fn get_best_move(&mut self,depth:u8)->String{
        let best_move = self.engine.get_best_move_depth(depth).unwrap();
        let json = MoveJSON{
            src : best_move.parse_from(),
            dest :best_move.parse_to(),
            mtype : best_move.parse_mtype().unwrap().to_string()
        };
        serde_json::to_string(&json).unwrap()
    }
}