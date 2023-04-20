use clap::{Arg,Command};
use std::time::Instant;

fn main() {
    // cargo run --
    let args:Vec<String> = std::env::args().collect();
    let depth:u8 = 3;
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
    let play_until_ply = 60;
    let matches = Command::new("Stonkfish CLI")
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
    
    let mut engine: stonkfish::Engine = stonkfish::Engine::new(fen);
    let start = Instant::now();
    for ply in 0..play_until_ply{

    }
    //engine.get_best_move_depth(depth);
}

#[cfg(test)]
mod engine_tests{
    use stonkfish::{Engine, time_it};
    #[test]
    pub fn test_perft(){
        //expected - 1: 20, 2: 400, 3: 8907, 4: 197281, 5: 4865609
        //rnbqkbnr/pppppppp/8/5p2/4P3/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        let depth = 2;
        
        let mut engine = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
        
        println!("Generated {} moves at depth {}, Color {:#?}",time_it!("perft",{engine.perft(depth,1)}),depth, engine.position.turn);
        time_it!("print",{engine.stats.move_gen_stats.display_stats()});
    }
}
