/// Example:
/// - `declare_field!(usize, elevator, set_elevator, (N * 2 * 2), 0b11);`
///   - declares elevator bit field at offset N * 2 * 2 with bitmask 0b11 and input/output type to be usize
/// - `declare_field!(u8, bool, get_up, set_up, 1);`
///   - Declares a boolean field
/// - `declare_field!(u64, u8, get_player_hp, set_player_hp, 0, 0b0111_1111);`
///   - in a parent struct with inner type u64
///   - declare u8 field
///   - getter has get_player_hp name
///   - setter has get_player_hp name
///   - offset 0
///   - mask 0b0111_1111
#[macro_export]
macro_rules! declare_field {
    ($parent:ty, bool, $f:ident,$set_f:ident, $offset:expr) => {
        #[inline(always)]
        fn $f(&self) -> bool {
            ((self.0 >> $offset) & 0b1) != 0
        }
        #[inline(always)]
        fn $set_f(&self, value: bool) -> Self {
            let bit: $parent = if value { 1 } else { 0 };
            Self((!(1 << $offset) & self.0) | (bit << $offset))
        }
    };
    ($field_ty:ty, $f:ident,$set_f:ident, $offset:expr, $mask:expr) => {
        declare_field!($field_ty, $field_ty, $f, $set_f, $offset, $mask);
    };
    ($parent:ty, $field_ty:ty, $f:ident,$set_f:ident, $offset:expr, $mask:expr) => {
        #[inline(always)]
        fn $f(&self) -> $field_ty {
            ((self.0 >> $offset) & $mask) as $field_ty
        }
        #[inline(always)]
        fn $set_f(&self, value: $field_ty) -> Self {
            debug_assert!(value <= $mask);
            Self((!($mask << $offset) & self.0) | ((value as $parent) << $offset))
        }
    };
}
/// Example:
/// - `declare_array!(usize, microchip, set_microchip, 0, 2, 0b11)`;
///   - declares bit array of elements named microchip at offset 0 with element size = 2 and mask 0b11
#[macro_export]
macro_rules! declare_array {
    ($out:ty, $f:ident, $set_f:ident, $offset:expr, $elem_bits:expr, $elem_mask:expr) => {
        #[inline(always)]
        fn $f(&self, i: usize) -> $out {
            (self.0 >> ((i * $elem_bits) + $offset)) & $elem_mask
        }
        #[inline(always)]
        fn $set_f(&self, i: usize, value: $out) -> Self {
            Self(
                (!($elem_mask << ((i * $elem_bits) + $offset)) & self.0)
                    | (value << ((i * $elem_bits) + $offset)),
            )
        }
    };
    ($parent:ty, $out:ty, $f:ident, $set_f:ident, $offset:expr, $elem_bits:expr, $elem_mask:expr) => {
        #[inline(always)]
        fn $f(&self, i: usize) -> $out {
            ((self.0 >> ((i * $elem_bits) + $offset)) & $elem_mask) as $out
        }
        #[inline(always)]
        fn $set_f(&self, i: usize, value: $out) -> Self {
            Self(
                (!($elem_mask << ((i * $elem_bits) + $offset)) & self.0)
                    | ((value as $parent) << ((i * $elem_bits) + $offset)),
            )
        }
    };
}
