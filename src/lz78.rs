use std::collections::HashMap;

pub struct Lz78Token
{
    index: u64,
    code: u8
}

impl Lz78Token
{
    pub fn to_bytes(&self) -> [u8; 9] {
        let mut token_bytes = [0u8; 9];

        token_bytes[..8].copy_from_slice(&self.index.to_le_bytes());
        token_bytes[8] = self.code;

        token_bytes
    }

    pub fn from_bytes(bytes: [u8; 9]) -> Self {
        let mut index_bytes = [0u8; 8];
        index_bytes.copy_from_slice(&bytes[..8]);

        Lz78Token {
            index: u64::from_le_bytes(index_bytes),
            code: bytes[8],
        }
    }
}

pub fn compress(input: &[u8]) -> Vec<Lz78Token>
{
    let mut compressed = Vec::new();
    let mut storage: HashMap<Vec<u8>, u64> = HashMap::new();
    storage.insert(Vec::new(), 0);

    let mut current = Vec::new();

    for &byte in input
    {
        current.push(byte);
        if !storage.contains_key(&current)
        {
            let prefix = &current[..current.len() - 1];
            let index = *storage.get(prefix).unwrap();

            compressed.push(Lz78Token {
                index: index as u64,
                code: byte
            });

            let new_index = storage.len() as u64;
            storage.insert(current.clone(), new_index);

            current.clear();
        }
    }

    if !current.is_empty()
    {
        let prefix = &current[..current.len() - 1];
        let index = *storage.get(prefix).unwrap();
        let last_byte = *current.last().unwrap();

        compressed.push(Lz78Token {
            index,
            code: last_byte
        });
    }
    compressed
}

pub fn decompress(data: &Vec<Lz78Token>) -> Vec<u8>
{
    let mut storage = Vec::new();
    storage.push(Vec::new());

    let mut decompressed = Vec::new();

    for &Lz78Token { index, code } in data
    {
        let mut entry = storage[index as usize].clone();
        entry.push(code);

        decompressed.extend(&entry);
        storage.push(entry);
    }

    decompressed
}