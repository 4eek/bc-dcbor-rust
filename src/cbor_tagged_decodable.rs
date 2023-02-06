use crate::{CBOR, CBORDecodable, tag::Tag, decode_error::DecodeError, decode_cbor};

/// A type that can be decoded from CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `from_untagged_cbor` function.
pub trait CBORTaggedDecodable: CBORDecodable {
    const CBOR_TAG: Tag;

    fn from_untagged_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError>;

    /// Creates an instance of this type by decoding it from tagged CBOR.
    fn from_tagged_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        match cbor {
            CBOR::Tagged(tag, item) => {
                if *tag == Self::CBOR_TAG {
                    return Ok(Self::from_untagged_cbor(item)?);
                } else {
                    return Err(DecodeError::WrongTag(Self::CBOR_TAG, tag.clone()));
                }
            },
            _ => return Err(DecodeError::WrongType)
        }
    }

    /// Creates an instance of this type by decoding it from binary encoded tagged CBOR.
    fn from_tagged_cbor_data(data: &[u8]) -> Result<Box<Self>, DecodeError> {
        let cbor = decode_cbor(data)?;
        Self::from_tagged_cbor(&cbor)
    }

    /// Creates an instance of this type by decoding it from binary encoded untagged CBOR.
    fn from_untagged_cbor_data(data: &[u8]) -> Result<Box<Self>, DecodeError> {
        let cbor = decode_cbor(data)?;
        Self::from_untagged_cbor(&cbor)
    }

    /// This override specifies that default CBOR encoding will be tagged.
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        Self::from_tagged_cbor(cbor)
    }
}
