extern crate cfg_if;
use cfg_if::cfg_if;
use chesscore::{Variant, VariantActions};
use chesscore::Color;
use wasm_bindgen::prelude::*;

pub mod evaluation;
pub mod position;
pub mod stats;
pub mod transposition;
pub mod move_generation;
pub mod utils;
pub mod bitboard;

use crate::{
    evaluation::Evaluator,
    move_generation::{generate_legal_moves, moves::*, MoveGenerator},
    position::Position,
    stats::*,
    transposition::*,
};
use serde::Serialize;
extern crate console_error_panic_hook;


cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
         pub fn set_panic_hook() {}
    }
}


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

 
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub struct Stonkfish {
    engine: Engine,
}

#[derive(Serialize, Debug)]
struct EngineMoveJSON {
    src: u8,
    dest: u8,
    mtype: String,
}


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Hello, console!");
}


#[wasm_bindgen]
pub struct ChessCoreLib {
    chesscore: ChessCore,
}

#[wasm_bindgen]
impl ChessCoreLib {
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(constructor)]
    pub fn new(config: String) -> Self {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());
        ChessCoreLib {
            chesscore: ChessCore::new(config),
        }
    }

    #[wasm_bindgen(js_name= getLegalMoves)]
    pub fn get_legal_moves(&mut self) -> JsValue {
        let moves = &self.chesscore.variant.get_legal_moves();
        serde_wasm_bindgen::to_value(&moves).unwrap()
    }

    #[wasm_bindgen(js_name= getPseudoLegalMoves)]
    pub fn get_pseudo_legal_moves(&self) -> JsValue {
        let moves = &self.chesscore.variant.get_pseudo_legal_moves(Color::WHITE);
        serde_wasm_bindgen::to_value(&moves).unwrap()
    }
}

#[wasm_bindgen]
impl Stonkfish {
    #[wasm_bindgen(constructor)]
    pub fn new(fen: JsValue) -> Self {
        Stonkfish {
            engine: Engine::new(fen.as_string().unwrap()),
        }
    }
    pub fn get_best_move(&mut self, depth: u8) {
        let best_move = self.engine.get_best_move_depth(depth).unwrap();
        let json = EngineMoveJSON {
            src: best_move.get_src_square(),
            dest: best_move.get_dest_square(),
            mtype: best_move.get_mtype().unwrap().to_string(),
        };
    }
}

pub struct Engine {
    pub move_generator: MoveGenerator,
    pub evaluator: Evaluator,
    pub position: Position,
    transposition_table: TranspositionTable,
    pub stats: Stats,
}

pub struct ChessCore {
    pub variant: Variant,
}

impl ChessCore {
    pub fn new(config: String) -> Self {
        let variant = serde_json::from_str(&config).unwrap();
        ChessCore { variant }
    }
}

const TEST_TT_SIZE: usize = 254288000;

impl Engine {
    pub fn new(fen: String) -> Engine {
        let mut position: Position = Position::new(fen);
        Engine {
            move_generator: MoveGenerator::new(
                position.dimensions.clone(),
                position.get_jump_offets(),
            ),
            evaluator: Evaluator::new(&mut position),
            position: position,
            transposition_table: TranspositionTable::new(TEST_TT_SIZE),
            stats: Stats::new(),
        }
    }

    pub fn alphabeta(&mut self, depth: u8, mut alpha: isize, beta: isize) -> isize {
        if depth == 0 {
            return 0; // self.evaluator.evaluate(&mut self.position) //(quiescense here later)
        }

        let cur_best_move: Move = Move::encode_move(0, 0, MType::None, None);
        let best_score = 0;
        let old_alpha = alpha;

        self.stats.search_stats.node_count += 1;
        let turn = self.position.turn;
        let in_check = self
            .move_generator
            .is_king_under_check(&mut self.position, turn);
        let mut legal_move_count = 0;

        let move_list = self
            .move_generator
            .generate_pseudolegal_moves(&mut self.position);
        for (_, mv) in move_list.enumerate() {
            if !self.move_generator.is_legal_move(&mut self.position, &mv) {
                continue;
            }

            self.position.make_move(&mv);
            let score = -self.alphabeta(depth - 1, -beta, -alpha);
            self.position.unmake_move(&mv);
            legal_move_count += 1;
            if score > alpha {
                alpha = score;
            }
            if score >= beta {
                return beta;
            } // fail hard beta cutoff
        }

        if legal_move_count == 0 {
            if in_check {
                return isize::MIN;
            }
            return 0;
        }
        // better move found
        if old_alpha != alpha {
            let best_move = cur_best_move;
            self.transposition_table.insert(TableEntry {
                key: self.position.get_zobrist_hash(),
                node_type: NodeType::Exact,
                score: best_score,
                depth: depth,
                best_move: best_move,
            });
        }
        alpha
    }

    pub fn get_best_move_depth(&mut self, depth: u8) -> Option<Move> {
        for _ in 1..depth + 1 {
            self.alphabeta(depth, isize::MIN, isize::MAX);
        }
        let best_move: Option<Move> = match self
            .transposition_table
            .lookup(self.position.get_zobrist_hash())
        {
            Some(x) => Some(x.best_move.to_owned()),
            None => None,
        };
        best_move
    }

    pub fn perft(&mut self, depth: u32, current_depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut nodes = 0;
        let moves = generate_legal_moves(&self.move_generator, &mut self.position); //time_it!("Legal move generation",{ generate_legal_moves(&self.move_generator, &mut self.position) });
        self.stats.move_gen_stats.moves_generated += moves.len();
        //println!("{}({})","---|".repeat(current_depth as usize),current_depth);
        for mv in moves {
            self.stats.move_gen_stats.update_move_type_count(&mv);
            /*println!("{} - depth {}",mv.to_algebraic_notation(
                self.move_generator.dimensions.height,
                self.position.turn,
                &self.position.piece_collections[self.position.turn as usize]
            ),depth);*/
            self.position.make_move(&mv);
            let count = self.perft(depth - 1, current_depth + 1);
            *self
                .stats
                .move_gen_stats
                .moves_per_depth
                .entry(current_depth)
                .or_insert(0) += 1;
            nodes += count;
            self.position.unmake_move(&mv);
        }
        //println!("{}({})","---|".repeat(current_depth as usize),current_depth);
        return nodes;
    }

    pub fn perft_divide(&mut self, depth: u32) -> u64 {
        let mut total = 0;
        if depth == 0 {
            return 1;
        }
        let moves = generate_legal_moves(&self.move_generator, &mut self.position);
        for mv in moves {
            self.position.make_move(&mv);
            let count = self.perft(depth - 1, 1);
            self.position.unmake_move(&mv);
            total += count;
            println!(
                "{} - {}",
                mv.to_algebraic_notation(
                    self.move_generator.dimensions.height,
                    self.position.turn,
                    &self.position.piece_collections[self.position.turn as usize]
                ),
                count
            );
        }
        println!("Found {} moves", total);
        total
    }
}

#[cfg(test)]
mod engine_tests {
    use crate::{time_it, Engine};
    
    #[test]
    pub fn test_standard_pos() {
        let depth = 3;
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut engine = Engine::new(fen.to_string());
        engine.perft(depth, 1);
        let expected: [u64; 3] = [20, 400, 8902];
        for i in 0..depth {
            let res = engine
                .stats
                .move_gen_stats
                .moves_per_depth
                .get(&(i + 1))
                .unwrap();
            assert_eq!(*res, expected[i as usize]);
        }
    }

    // not an actual test, just displays perft results for manual testing
    #[test]
    pub fn test_perft() {
        /*expected ( nodes, total_nodes) :
            1: 20, 20
            2: 400, 420
            3: 8902, 9322
            4: 197281, 206603
            5: 4865609, 5072212,
            6: 119060324, 124132536
            7: 3195901860, 3320034396
        */
        //rnbqkbnr/pppppppp/8/5p2/4P3/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        //"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR" "3r1k2/8/8/8/8/8/3R4/3K4 w - - 0 1"
        let depth = 3;
        let fen = "r3k3/8/8/8/8/8/8/4K3 w q - 0 1";
        let mut engine = Engine::new(fen.to_string());

        println!(
            "Generated {} moves at depth {}, Color {:#?}",
            time_it!("perft", { engine.perft(depth, 1) }),
            depth,
            engine.position.turn
        );
        engine.stats.move_gen_stats.display_stats();
    }

    // not an actual test, just displays perft_divide results for manual testing
    #[test]
    pub fn test_perft_divide() {
        let depth = 1;
        let fen = "r3k3/8/8/8/8/8/8/3K4 b q - 0 1";
        let mut engine = Engine::new(fen.to_string());
        engine.perft_divide(depth);
    }
}
