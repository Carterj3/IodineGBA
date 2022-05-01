use ::b64::{FromBase64, ToBase64};

use ::backtrace::Backtrace;

use ::serde::{Deserialize, Serialize};

use ::std::collections::BTreeMap;

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
pub struct ChunkedDelta {
    offset: u8,
    value: u8,
}

impl ChunkedDelta {
    pub fn new(offset: u8, value: u8) -> ChunkedDelta {
        ChunkedDelta { offset, value }
    }

    pub fn offset(&self) -> u32 {
        self.offset as u32
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeltaSnapshot {
    old_hash: [u8; 2],
    chunks: BTreeMap<u8, Vec<ChunkedDelta>>,
}

impl DeltaSnapshot {
    pub fn new(old_snapshot: &[u8], new_snapshot: &[u8]) -> DeltaSnapshot {
        let old_hash = [
            old_snapshot.first().map(|v| *v).unwrap_or(0),
            old_snapshot.last().map(|v| *v).unwrap_or(0),
        ];

        assert_eq!(old_snapshot.len(), new_snapshot.len());

        let mut chunks = BTreeMap::new();

        old_snapshot
            .iter()
            .zip(new_snapshot.iter())
            .enumerate()
            .filter(|(_, (old_value, new_value))| old_value != new_value)
            .for_each(|(index, (_, new_value))| {
                let chunk = (index / (u8::MAX as usize)) as u8;
                let offset = (index % (u8::MAX as usize)) as u8;

                chunks
                    .entry(chunk)
                    .or_insert(Vec::new())
                    .push(ChunkedDelta::new(offset, *new_value));
            });

        DeltaSnapshot { old_hash, chunks }
    }

    pub fn apply(self, old_snapshot: &[u8]) -> Vec<u8> {
        let old_hash = [
            old_snapshot.first().map(|v| *v).unwrap_or(0),
            old_snapshot.last().map(|v| *v).unwrap_or(0),
        ];

        if self.old_hash != old_hash {
            panic!("Hashes did not match!");
        }

        let mut new_snapshot = Vec::from(old_snapshot);

        for (chunk_index, chunk) in self.chunks.into_iter() {
            for (offset_index, new_value) in chunk
                .into_iter()
                .map(|delta| (delta.offset(), delta.value()))
            {
                let index = ((chunk_index as usize) * (u8::MAX as usize)) + (offset_index as usize);

                new_snapshot[index] = new_value;
            }
        }

        new_snapshot
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().map(|(_, chunk)| chunk.len()).sum()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Bios(Vec<u8>),
    Rom(Vec<u8>),
    Play(Vec<u8>),
    DeltaSnapshot(DeltaSnapshot),
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
    use crate::DeltaSnapshot;

    #[test]
    fn test_delta_snapshot() {
        let mut old = Vec::new();
        let mut new = Vec::new();

        for i in 0..u8::MAX {
            old.push(i);
            old.push(u8::MAX - i);
            old.push(u8::MAX - i);
            old.push(u8::MAX - i);
            old.push(i);
            old.push(i);

            new.push(i);
            new.push(i);
            new.push(i);
            new.push(i);
            new.push(u8::MAX - i);
            new.push(u8::MAX - i);
        }

        let snapshot = DeltaSnapshot::new(&old, &new);

        let new2 = snapshot.apply(&old);

        assert_eq!(new, new2);
    }
}
