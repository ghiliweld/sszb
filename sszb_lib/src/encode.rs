use bytes::buf::BufMut;

pub mod encode_impls;

// Most of the complexity in implementing ssz macros arises from offset accounting.
// Using the BufMut trait means that moving the buffer cursor is taken care of for us.
pub trait SszEncode {
    fn is_ssz_static() -> bool;

    // all lengths are in number of bytes
    fn ssz_fixed_len() -> usize;
    fn ssz_bytes_len(&self) -> usize;

    // helper function, use when preallocating the max bytes needed to encode this type
    fn ssz_max_len() -> usize;

    // ssz_write_fixed either writes fixed types to the buffer,
    // or writes the offset to the buffer and increases the offset by self.ssz_bytes_len()
    fn ssz_write_fixed(&self, offset: &mut usize, buf: &mut impl BufMut);
    // write self to the buffer if the type is dynamic (variable-sized)
    fn ssz_write_variable(&self, buf: &mut impl BufMut);
    // this function specifies how to write self to the buffer
    // this may create an offset and make calls to ssz_write_fixed and ssz_write_variable
    fn ssz_write(&self, buf: &mut impl BufMut);

    // dev facing helper function for when a buffer is not already allocated
    // ssz_write should be used if there's a spare buffer around to write into
    fn to_ssz(&self) -> Vec<u8> {
        // buf must be appropriately sized
        let mut buf = Vec::with_capacity(self.ssz_bytes_len());
        self.ssz_write(&mut buf);

        buf
    }

    // dev facing helper function for when a buffer is already allocated
    fn to_ssz_with_vec(&self, buf: &mut Vec<u8>) {
        // buf must be appropriately sized before writing to it
        // .reserve_exact reserves the required additional capacity if not already allocated
        buf.reserve_exact(self.ssz_bytes_len());
        self.ssz_write(buf);
    }
}
