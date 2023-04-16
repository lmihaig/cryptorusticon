#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_u8() {
        let mut bbs = cryptorusticon::bbs::BBS::default();
        let result = bbs.generate_u8();

        assert_eq!(result, 1);
    }

    #[test]

    fn test_generate_u16() {
        let mut bbs = cryptorusticon::bbs::BBS::default();
        let result = bbs.generate_u16();

        assert_eq!(result, 289);
    }

    #[test]

    fn test_generate_u32() {
        let mut bbs = cryptorusticon::bbs::BBS::default();
        let result = bbs.generate_u32();

        assert_eq!(result, 18990369);
    }

    #[test]

    fn test_generate_u64() {
        let mut bbs = cryptorusticon::bbs::BBS::default();
        let result = bbs.generate_u64();

        assert_eq!(result, 81563015645684253);
    }

    #[test]

    fn test_generate_u128() {
        let mut bbs = cryptorusticon::bbs::BBS::default();
        let result = bbs.generate_u128();

        assert_eq!(result, 1504572075495905439180798159389866542);
    }

    #[test]
    fn test_next() {
        let mut bbs = cryptorusticon::bbs::BBS::default();
        assert_eq!(bbs.next(), false);
        assert_eq!(bbs.next(), false);
    }

    #[test]
    fn test_get_n_bits() {
        let mut bbs = cryptorusticon::bbs::BBS::default();
        let bit_string = bbs.get_n_bits(8);
        assert_eq!(bit_string.len(), 8);

        let mut bbs = cryptorusticon::bbs::BBS::default();
        let bit_string = bbs.get_n_bits(100);
        assert_eq!(bit_string.len(), 100);
    }
}
