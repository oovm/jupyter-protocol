use hmac::{Hmac, Mac};
use sha2::Sha256;

pub(crate) type HmacSha256 = Hmac<Sha256>;

pub(crate) trait SignComputable {
    fn signature<M>(&self, auth: M) -> String
    where
        M: Mac;
}

impl SignComputable for Vec<Vec<u8>> {
    fn signature<M>(&self, mut auth: M) -> String
    where
        M: Mac,
    {
        for msg in self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}

impl<'a> SignComputable for Vec<&'a [u8]> {
    fn signature<M>(&self, mut auth: M) -> String
    where
        M: Mac,
    {
        for msg in self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}
impl<'a> SignComputable for &'a [&'a [u8]] {
    fn signature<M>(&self, mut auth: M) -> String
    where
        M: Mac,
    {
        for msg in *self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}

impl<'a> SignComputable for &'a [Vec<u8>] {
    fn signature<M>(&self, mut auth: M) -> String
    where
        M: Mac,
    {
        for msg in *self {
            auth.input(msg);
        }
        let result = auth.result();
        let code = result.code();
        let encoded = hex::encode(code);
        encoded
    }
}

pub(crate) fn sign<S, M>(msg_list: S, auth: M) -> String
where
    S: SignComputable,
    M: Mac,
{
    msg_list.signature(auth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signing() {
        let auth = HmacSha256::new_varkey(b"foobar").unwrap();
        let data = vec![&b"a"[..], b"b"];
        let signature = sign(data, auth);
        assert_eq!(
            signature,
            "77d67cc5dee7cc59a379f373432c9eb6d4183225f384ee84494cad997fd22c2a"
        );
    }
}
