use std::hash::Hasher;

/// A hasher that only accepts integers and use their raw values as indices.
#[derive(Copy, Clone, Debug, Default)]
pub struct PrimitiveHasher(Option<u64>);

impl Hasher for PrimitiveHasher {
    fn finish(&self) -> u64 {
        match self.0 {
            Some(value) => value,
            None => panic!("PrimitiveHasher have not hashed any values"),
        }
    }

    fn write(&mut self, _: &[u8]) {
        panic!("PrimitiveHasher can only hash primitive unsigned values");
    }

    fn write_u64(&mut self, i: u64) {
        match self.0 {
            Some(_) => panic!("PrimitiveHasher can only hash single values"),
            None => self.0 = Some(i),
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64)
    }

    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64)
    }

    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64)
    }

    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64)
    }
}
