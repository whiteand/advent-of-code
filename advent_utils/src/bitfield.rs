/// Example:
/// - `declare_field!(usize, elevator, set_elevator, (N * 2 * 2), 0b11);`
///   - declares elevator bit field at offset N * 2 * 2 with bitmask 0b11 and input/output type to be usize
#[macro_export]
macro_rules! declare_field {
    ($out:ty, $f:ident,$set_f:ident, $offset:expr, $mask:expr) => {
        #[inline(always)]
        fn $f(&self) -> $out {
            (self.0 >> $offset) & $mask
        }
        #[inline(always)]
        fn $set_f(&self, value: $out) -> Self {
            Self((!($mask << $offset) & self.0) | (value << $offset))
        }
    };
}
/// Example:
/// - declare_array!(usize, microchip, set_microchip, 0, 2, 0b11);
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
}
