use crypto_mac::MacResult;
use digest::generic_array::typenum::U64;
use generic_array::GenericArray;
use hmac::Mac;

#[derive(Debug, Clone)]
pub(crate) struct FakeAuth;

static KEY: &[u8] = b"foobar0000000000000000000000000000000000000000000000000000000000";

impl Mac for FakeAuth {
    type OutputSize = U64;
    type KeySize = U64;

    fn new(_keys: &GenericArray<u8, Self::KeySize>) -> Self {
        FakeAuth {}
    }

    fn input(&mut self, _data: &[u8]) {}
    fn reset(&mut self) {}
    fn result(self) -> MacResult<Self::OutputSize> {
        MacResult::new(GenericArray::clone_from_slice(KEY))
    }
}

impl FakeAuth {
    pub(crate) fn create() -> FakeAuth {
        FakeAuth::new_varkey(KEY).expect("creating fake auth object")
    }
}

pub(crate) fn expected_signature() -> String {
    let auth = FakeAuth::create();
    let res = auth.result();
    let code = res.code();
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
