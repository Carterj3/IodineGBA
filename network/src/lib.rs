use ::b64::{FromBase64, ToBase64};

use ::backtrace::Backtrace;

use ::serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EncodingError {
    kind: EncodingErrorKind,
    stack: Backtrace,
}

impl EncodingError {
    fn new(kind: EncodingErrorKind) -> Self {
        EncodingError {
            kind,
            stack: Backtrace::new(),
        }
    }
}

impl From<::bincode::Error> for EncodingError {
    fn from(error: ::bincode::Error) -> Self {
        EncodingError::new(EncodingErrorKind::Bincode(error))
    }
}

impl From<::b64::FromBase64Error> for EncodingError {
    fn from(error: ::b64::FromBase64Error) -> Self {
        EncodingError::new(EncodingErrorKind::Base64(error))
    }
}

#[derive(Debug)]
pub enum EncodingErrorKind {
    Bincode(::bincode::Error),
    Base64(::b64::FromBase64Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrayDelta {
    index: u32,
    value: u8,
}

impl ArrayDelta {
    pub fn new(index: u32, value: u8) -> ArrayDelta {
        ArrayDelta { index, value }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Bios(Vec<u8>),
    Rom(Vec<u8>),
    DeltaSnapshot(Vec<ArrayDelta>),
    Snapshot(Vec<u8>),
}

impl TryInto<Vec<u8>> for &Message {
    type Error = ::bincode::Error;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(self)
    }
}

impl TryInto<Vec<u8>> for Message {
    type Error = ::bincode::Error;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(&self)
    }
}

impl TryFrom<Vec<u8>> for Message {
    type Error = ::bincode::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bincode::deserialize(&value)
    }
}

impl TryInto<String> for &Message {
    type Error = ::bincode::Error;
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(bincode::serialize(self)?.to_base64(b64::STANDARD))
    }
}

impl TryInto<String> for Message {
    type Error = ::bincode::Error;
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(bincode::serialize(&self)?.to_base64(b64::STANDARD))
    }
}

impl TryFrom<&str> for Message {
    type Error = EncodingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(bincode::deserialize(&value.from_base64()?)?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
