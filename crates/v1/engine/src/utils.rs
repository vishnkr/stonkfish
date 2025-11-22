
#[macro_export]
macro_rules! time_it {
    ($context:literal, $code:block) => {{
        let timer = std::time::Instant::now();
        let result = { $code };
        let elapsed = timer.elapsed();
        println!("{}: {:?}", $context, elapsed);
        result
    }};
}


pub fn get_rank_attacks(is_right:bool,pos:u16)->u16{
    if is_right{
        if pos != 0 {
            return pos-1
        }
        return 0u16
    }
    !pos & !get_rank_attacks(true,pos)
}