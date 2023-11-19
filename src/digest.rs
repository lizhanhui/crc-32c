use crate::algorithm::Algorithm;

pub struct Digest<'a> {
    crc: u32,
    algorithm: &'a Algorithm,
}

impl<'a> Digest<'a> {
    pub fn new(algorithm: &'a Algorithm) -> Digest<'_> {
        Self::with_initial(algorithm, algorithm.init)
    }

    pub fn with_initial(algorithm: &'a Algorithm, initial: u32) -> Digest<'_> {
        let crc = if algorithm.refin {
            initial.reverse_bits()
        } else {
            initial
        };

        Digest { crc, algorithm }
    }

    pub fn update(&mut self, buffer: &[u8]) {
        let _ = buffer.len();
    }

    pub fn finalized(&mut self) -> u32 {
        let mut crc = self.crc;
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = crc.reverse_bits();
        }
        if !self.algorithm.refout {
            crc >>= 32u8 - self.algorithm.width;
        }
        crc ^ self.algorithm.xorout
    }
}

#[cfg(test)]
mod tests {

    use super::Digest;

    #[test]
    fn test_digest() {
        let mut digest = Digest::new(&crate::algorithm::CRC_32_ISCSI);
        let buffer = b"123456789";
        digest.update(buffer);
        let crc = digest.finalized();
        assert_eq!(crc, crate::algorithm::CRC_32_ISCSI.check);
    }

    #[test]
    fn test_digest_with_initial() {
        let seed = 1234;
        let mut digest = Digest::with_initial(&crate::algorithm::CRC_32_ISCSI, seed);
        let buffer = b"123456789";
        digest.update(buffer);
        let crc = digest.finalized();

        let castagnoli = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
        let mut digest = castagnoli.digest_with_initial(seed);
        digest.update(buffer);
        let expected = digest.finalize();
        assert_eq!(crc, expected);
    }
}
