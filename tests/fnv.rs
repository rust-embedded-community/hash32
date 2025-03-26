use hash32::FnvHasher;
use std::hash::Hasher;

#[test]
fn fnv1a_vectors() {
    // Test vectors adapted from this public domain repo:
    // https://github.com/drichardson/fnv/blob/cd12926ef306ec32dbef3e65c62662c4eb5b99ed/test_fnv.c

    const FNV1A_TEST_VECTORS: &[(&str, u64)] = &[
        ("", 0x811c9dc5),
        ("a", 0xe40c292c),
        ("b", 0xe70c2de5),
        ("c", 0xe60c2c52),
        ("d", 0xe10c2473),
        ("e", 0xe00c22e0),
        ("f", 0xe30c2799),
        ("fo", 0x6222e842),
        ("foo", 0xa9f37ed7),
        ("foob", 0x3f5076ef),
        ("fooba", 0x39aaa18a),
        ("foobar", 0xbf9cf968),
        ("ch", 0x5f299f4e),
        ("cho", 0xef8580f3),
        ("chon", 0xac297727),
        ("chong", 0x4546b9c0),
        ("chongo", 0xbd564e7d),
        ("chongo ", 0x6bdd5c67),
        ("chongo w", 0xdd77ed30),
        ("chongo wa", 0xf4ca9683),
        ("chongo was", 0x4aeb9bd0),
        ("chongo was ", 0xe0e67ad0),
        ("chongo was h", 0xc2d32fa8),
        ("chongo was he", 0x7f743fb7),
        ("chongo was her", 0x6900631f),
        ("chongo was here", 0xc59c990e),
        ("chongo was here!", 0x448524fd),
        ("chongo was here!\n", 0xd49930d5),
    ];

    for (data, expected) in FNV1A_TEST_VECTORS {
        let mut hasher: FnvHasher = Default::default();
        hasher.write(data.as_bytes());
        assert_eq!(hasher.finish(), *expected);
    }
}
