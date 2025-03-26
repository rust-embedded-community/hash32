use hash32::Murmur3Hasher;
use std::hash::Hasher;

fn testcase(data: &[u8], expected_hash_value: u64) {
    let mut hasher: Murmur3Hasher = Default::default();
    hasher.write(data);
    assert_eq!(hasher.finish(), expected_hash_value);
}

#[test]
fn murmurhash3_vectors() {
    // Test vectors are adapted from this gist
    // https://gist.github.com/vladimirgamalyan/defb2482feefbf5c3ea25b14c557753b

    // with zero data and zero seed, everything becomes zero
    testcase(&[], 0);
    // make sure 4-byte chunks use unsigned math
    testcase(&[0xff, 0xff, 0xff, 0xff], 0x76293B50);
    // Endian order. UInt32 should end up as 0x87654321
    testcase(&[0x21, 0x43, 0x65, 0x87], 0xF55B516B);
    // Only three bytes. Should end up as 0x654321
    testcase(&[0x21, 0x43, 0x65], 0x7E4A8634);
    // Only two bytes. Should end up as 0x4321
    testcase(&[0x21, 0x43], 0xA0F7B07A);
    // Only one byte. Should end up as 0x21
    testcase(&[0x21], 0x72661CF4);
    // Make sure compiler doesn't see zero and convert to null
    testcase(&[0x00, 0x00, 0x00, 0x00], 0x2362F9DE);
    testcase(&[0x00, 0x00, 0x00], 0x85F0B427);
    testcase(&[0x00, 0x00], 0x30F4C306);
    testcase(&[0x00], 0x514E28B7);

    testcase(
        "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_bytes(),
        0xEE925B90,
    );
}
