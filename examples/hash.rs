#![feature(hashmap_internals)]
use std::time;
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{SipHasher13, SipHasher};

type TTTT = u32;

fn main() {
    let a = time::SystemTime::now();
    let mut m = HashMap::<TTTT, TTTT, _>::with_hasher(
        BuildHasherDefault::<SipHasher13>::default()
    );
    for i in 0..200_000_000 {
        *m.entry(i % 100_000).or_insert(0) += 1;
    }
    println!("{}", m[&40000]);
    println!("SIP13 {:?}", a.elapsed().unwrap());

    let a = time::SystemTime::now();
    let mut m = HashMap::<TTTT, TTTT, _>::with_hasher(
        BuildHasherDefault::<SipHasher>::default()
    );
    for i in 0..200_000_000 {
        *m.entry(i % 100_000).or_insert(0) += 1;
    }
    println!("{}", m[&40000]);
    println!("SIP24 {:?}", a.elapsed().unwrap());

    let a = time::SystemTime::now();
    let mut m = HashMap::<TTTT, TTTT, _>::with_hasher(
        BuildHasherDefault::<fnv::FnvHasher>::default()
    );
    for i in 0..200_000_000 {
        *m.entry(i % 100_000).or_insert(0) += 1;
    }
    println!("{}", m[&40000]);
    println!("FNV {:?}", a.elapsed().unwrap());

    let a = time::SystemTime::now();
    let mut m = HashMap::<TTTT, TTTT, _>::with_hasher(
        BuildHasherDefault::<simple::SimpleHasher>::default()
    );
    for i in 0..200_000_000 {
        *m.entry(i % 100_000).or_insert(0) += 1;
    }
    println!("{}", m[&40000]);
    println!("Simple {:?}", a.elapsed().unwrap());


    let a = time::SystemTime::now();
    let mut m = HashMap::new();
    for i in 0..200_000_000 {
        *m.entry(i % 100_000).or_insert(0) += 1;
    }
    println!("{}", m[&40000]);
    println!("Default {:?}", a.elapsed().unwrap());

}

mod fnv {
    use std::hash::Hasher;
    pub struct FnvHasher(u64);

    impl Default for FnvHasher {

        #[inline]
        fn default() -> FnvHasher {
            FnvHasher(0xcbf29ce484222325)
        }
    }

    impl Hasher for FnvHasher {

        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            let FnvHasher(mut hash) = *self;

            for byte in bytes.iter() {
                hash = hash ^ (*byte as u64);
                hash = hash.wrapping_mul(0x100000001b3);
            }

            *self = FnvHasher(hash);
        }
    }
}

mod simple {
    use std::hash::Hasher;
    pub struct SimpleHasher(u64);

    #[inline]
    fn load_u64_le(buf: &[u8], len: usize) -> u64 {
        use std::ptr;
        debug_assert!(len <= buf.len());
        let mut data = 0u64;
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), &mut data as *mut _ as *mut u8, len);
        }
        data.to_le()
    }


    impl Default for SimpleHasher {

        #[inline]
        fn default() -> SimpleHasher {
            SimpleHasher(0)
        }
    }

    impl Hasher for SimpleHasher {

        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            *self = SimpleHasher(load_u64_le(bytes, bytes.len()));
        }
    }
}