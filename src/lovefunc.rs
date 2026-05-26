/// Timmy & Sarah think they are in love, but around where they live, they will only know once they pick a flower each.
/// If one of the flowers has an even number of petals and the other has an odd number of petals it means they are in love.
/// Write a function that will take the number of petals of each flower and return true if they are in love and false if they aren't.
pub fn lovefunc(flower1: u16, flower2: u16) -> bool {
    !flower1.overflowing_sub(flower2).0 % 2 == 0
}
