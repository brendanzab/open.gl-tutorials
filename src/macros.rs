{

/* applies a type cast to an array of values */
macro_rules! map_cast(
    ( [$($elems:expr),+] : $T:ty) => ( [$($elems as $T),+]);
    (~[$($elems:expr),+] : $T:ty) => (~[$($elems as $T),+]);
    (@[$($elems:expr),+] : $T:ty) => (@[$($elems as $T),+]);
    ($arr:expr : $T:ty)           => ($arr.map(|a| *a as $T));
)

}