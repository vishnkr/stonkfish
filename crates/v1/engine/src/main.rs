use clap::{Arg,Command};
use sf_engine::Engine;
use std::time::Instant;

fn main() {
    // cargo run --
    let _args:Vec<String> = std::env::args().collect();
    let _depth:u8 = 3;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
    let play_until_ply = 60;
    let _matches = Command::new("Stonkfish CLI")
        .version("0.1.0")
        .about("CLI to play/test stonkfish")
        .arg(Arg::new("fen")
                 .short('f')
                 .long("fen")
                 .help("[optional] Load custom FEN"))
        .arg(Arg::new("depth")
                 .short('d')
                 .long("depth")
                 .help("[optional] Run engine on specified depth"))
        .get_matches();
    
    let mut _engine: Engine = Engine::new(fen);
    let _start = Instant::now();
    for _ply in 0..play_until_ply{

    }
    //engine.get_best_move_depth(depth);
}

#[cfg(test)]
mod engine_tests{
    use sf_engine::{Engine,time_it};
    #[test]
    pub fn test_perft(){
        /*expected (nodes, total_nodes) :
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
        let depth = 4;
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR - 0 1";
        let mut engine = Engine::new(fen.to_string());
        
        println!("Generated {} moves at depth {}, Color {:#?}",time_it!("perft",{engine.perft(depth,1)}),depth, engine.position.turn);
        engine.stats.move_gen_stats.display_stats();
    }

    #[test]
    pub fn test_perft_divide(){
        let depth = 4;
        let fen = "r3k3/8/8/8/8/8/8/3K4 b q - 0 1";
        let mut engine = Engine::new(fen.to_string());
        engine.perft_divide(depth);
    }
}
