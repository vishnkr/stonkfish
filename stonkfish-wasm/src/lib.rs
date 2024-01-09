use stonkfish::{chesscore::{load_config, Move}, ChessCore, engine, Engine};
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
    alert(&format!("Hello {}, welcome to stonkfish wasm",name));
}

#[wasm_bindgen]
pub struct Stonkfish{
    engine: stonkfish::Engine,
}

#[wasm_bindgen]
pub struct ChessCoreLib{
    chesscore: stonkfish::ChessCore
}


#[derive(Serialize,Debug)]
struct EngineMoveJSON{
    src: u8,
    dest: u8,
    mtype: String,
}

#[wasm_bindgen]
impl ChessCoreLib{
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(constructor)]
    pub fn new(game_config: JsValue)->Self{
        let config: String = serde_wasm_bindgen::from_value(game_config).unwrap();
        ChessCoreLib { chesscore: ChessCore::new(config)}
    }

    #[wasm_bindgen(js_name= getLegalMoves)]
    pub fn get_legal_moves(&self)->JsValue{
        serde_wasm_bindgen::to_value(&self.chesscore.variant.get_legal_moves()).unwrap()
    }

    #[wasm_bindgen(js_name= makeMove)]
    pub fn make_move(&mut self,mv:JsValue)->JsValue{
       let val:Result<Move, serde_wasm_bindgen::Error> = serde_wasm_bindgen::from_value(mv);
       match val{
        Ok(mov) => serde_wasm_bindgen::to_value(&self.chesscore.variant.make_move(mov)).unwrap(),
        Err(e) => serde_wasm_bindgen::to_value(&e.to_string()).unwrap()
       }
        
    }

}

#[wasm_bindgen]
impl Stonkfish{
    #[wasm_bindgen(constructor)]
    pub fn new(fen:JsValue)->Self{
        Stonkfish{
            engine: Engine::new(fen.as_string().unwrap())
        }
    }
    pub fn get_best_move(&mut self,depth:u8){
        let best_move = self.engine.get_best_move_depth(depth).unwrap();
        let json = EngineMoveJSON{
            src : best_move.get_src_square(),
            dest :best_move.get_dest_square(),
            mtype : best_move.get_mtype().unwrap().to_string()
        };
    }
}

