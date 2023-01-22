
pub fn get_rank_attacks(is_right:bool,pos:u16)->u16{
    if is_right{
        if pos != 0 {
            return pos-1
        }
        return 0u16
    }
    !pos & !get_rank_attacks(true,pos)
}