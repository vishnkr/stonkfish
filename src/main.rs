
fn main() {
    let mut engine: stonkfish::Engine = stonkfish::Engine::new("10/5p1k1/9/p2p1P3/5q3/P1PbN2p1/7P1/2Q3K2/10/10 w - - 1 44".to_string());
    let depth:u8 = 3;
    engine.get_best_move_depth(depth);
}
