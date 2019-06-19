use digest::generic_array::typenum::U64;
use digest::{CtOutput, FixedOutputReset, InvalidLength, Key, KeyInit, MacError, Output, OutputSizeUser, Reset};
use generic_array::GenericArray;
use hmac::Mac;

#[derive(Debug, Clone)]
pub(crate) struct FakeAuth;

static KEY: &[u8] = b"foobar0000000000000000000000000000000000000000000000000000000000";

impl OutputSizeUser for FakeAuth { type OutputSize = U64; }

impl Mac for FakeAuth {
    fn new(key: &Key<Self>) -> Self where Self: KeyInit {
        todo!()
    }

    fn new_from_slice(key: &[u8]) -> Result<Self, InvalidLength> where Self: KeyInit {
        todo!()
    }

    fn update(&mut self, data: &[u8]) {
        todo!()
    }

    fn chain_update(self, data: impl AsRef<[u8]>) -> Self {
        todo!()
    }

    fn finalize(self) -> CtOutput<Self> {
        todo!()
    }

    fn finalize_reset(&mut self) -> CtOutput<Self> where Self: FixedOutputReset {
        todo!()
    }

    fn reset(&mut self) where Self: Reset {
        todo!()
    }

    fn verify(self, tag: &Output<Self>) -> Result<(), MacError> {
        todo!()
    }

    fn verify_reset(&mut self, tag: &Output<Self>) -> Result<(), MacError> where Self: FixedOutputReset {
        todo!()
    }

    fn verify_slice(self, tag: &[u8]) -> Result<(), MacError> {
        todo!()
    }

    fn verify_slice_reset(&mut self, tag: &[u8]) -> Result<(), MacError> where Self: FixedOutputReset {
        todo!()
    }

    fn verify_truncated_left(self, tag: &[u8]) -> Result<(), MacError> {
        todo!()
    }

    fn verify_truncated_right(self, tag: &[u8]) -> Result<(), MacError> {
        todo!()
    }
}

impl FakeAuth {
    pub(crate) fn create() -> FakeAuth {
        FakeAuth::new_from_slice(KEY).expect("creating fake auth object")
    }
}

pub(crate) fn expected_signature() -> String {
    let auth = FakeAuth::create();
    let res = auth.finalize();
    let code = res.into_bytes();
    let encoded = hex::encode(code);
    encoded
}

#[macro_export]
macro_rules! compare_bytestrings {
    ($a:expr, $b:expr) => {
        let a = String::from_utf8_lossy($a).into_owned();
        let b = String::from_utf8_lossy($b).into_owned();
        assert_eq!($a, $b, "result {:?} != expected {:?}", a, b);
    };
}
