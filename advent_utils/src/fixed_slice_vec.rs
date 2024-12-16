pub use fixed_slice_vec::*;

///
/// Example of usage.
///
/// ```ignore
/// vec_on_stack! {
///    let (mut boxes_to_move: Vec<usize>, mut slice) = Vec::with_capacity(5);
/// }
/// ```
#[macro_export]
macro_rules! vec_on_stack {
    (let (mut $name:ident, mut $slice_id:ident) = Vec::<$t:ty>::with_capacity($n:expr)$(;)?) => {
        let mut $slice_id: [std::mem::MaybeUninit<u8>; std::mem::size_of::<$t>() * $n] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        let mut $name =
            $crate::fixed_slice_vec::FixedSliceVec::<$t>::from_uninit_bytes(&mut $slice_id);
    };
    (let (mut $name:ident: Vec<$t:ty>, mut $slice_id:ident) = Vec::with_capacity($n:expr)$(;)?) => {
        $crate::vec_on_stack! {
            let (mut $name, mut $slice_id) = Vec::<$t>::with_capacity($n);
        }
    };
}
