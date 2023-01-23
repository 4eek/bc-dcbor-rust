use super::{cbor::{CBOREncode, AsCBOR, CBOR, IntoCBOR}, varint::{VarIntEncode, MajorType}};

impl<T> CBOREncode for Vec<T> where T: CBOREncode {
    fn cbor_encode(&self) -> Vec<u8> {
        let mut buf = self.len().varint_encode(MajorType::Array);
        for item in self {
            buf.extend(item.cbor_encode());
        }
        buf
    }
}

impl<T> AsCBOR for Vec<T> where T: AsCBOR {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

impl<T> IntoCBOR for Vec<T> where T: IntoCBOR {
    fn into_cbor(self) -> CBOR {
        CBOR::Array(self.into_iter().map(|x| x.into_cbor()).collect())
    }
}

impl AsCBOR for Vec<Box<dyn AsCBOR>> {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

impl<T, const N: usize> AsCBOR for [T; N] where T: AsCBOR {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

impl<T, const N: usize> IntoCBOR for [T; N] where T: IntoCBOR {
    fn into_cbor(self) -> CBOR {
        CBOR::Array(self.into_iter().map(|x| x.into_cbor()).collect())
    }
}

impl<T, const N: usize> AsCBOR for &[T; N] where T: AsCBOR {
    fn as_cbor(&self) -> CBOR {
        CBOR::Array(self.iter().map(|x| x.as_cbor()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::AsCBOR};

    #[test]
    fn encode() {
        test_cbor(vec![1, 2, 3], "Array([Uint(1), Uint(2), Uint(3)])", "83010203");
        test_cbor([1, 2, 3], "Array([Uint(1), Uint(2), Uint(3)])", "83010203");
        test_cbor(&[1, -2, 3], "Array([Uint(1), Nint(-2), Uint(3)])", "83012103");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", [1, 2, 3].as_cbor()), "[1, 2, 3]");
    }
}
