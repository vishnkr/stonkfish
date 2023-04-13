
fn main() {
    let mut engine: stonkfish::Engine = stonkfish::Engine::new("10/5p1k1/9/p2p1P3/5q3/P1PbN2p1/7P1/2Q3K2/10/10 w - - 1 44".to_string());
    let depth:u8 = 3;
    engine.get_best_move_depth(depth);
}

#[cfg(test)]
mod engine_tests{
    use stonkfish::Engine;

    #[test]
    pub fn test_perft(){
        //expected - 1: 20, 2: 400, 3: 8907, 4: 197281, 5: 4865609
        let depth = 4;
        
        let mut engine = Engine::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
        println!("Generated {} moves at depth {}, Color {:#?}",engine.perft(depth),depth, engine.position.turn);
    }
}
