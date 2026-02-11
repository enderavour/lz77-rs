use std::collections::HashMap;

pub struct Lz78Token
{
    index: u16,
    code: u8
}

impl Lz78Token
{
    pub fn new() -> Self
    {
        Lz78Token { index: 0, code: 0 }
    }

    pub fn to_bytes(&self) -> [u8; 3]
    {
        let mut token_bytes = [0u8; 3];
        token_bytes[0] = (self.index & 0xFF) as u8;
        token_bytes[1] = ((self.index >> 8) & 0xFF) as u8;
        token_bytes[2] = self.code;
        token_bytes
    }

    pub fn from_bytes(bytes: [u8; 3]) -> Self
    {
        Lz78Token 
        {
            index: u16::from_le_bytes([bytes[0], bytes[1]]),
            code: bytes[2]
        }
    }
}

pub fn compress(input: &[u8]) -> Vec<Lz78Token>
{
    let mut compressed = Vec::new();
    let mut storage: HashMap<Vec<u8>, u16> = HashMap::new();
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
                index: index as u16,
                code: byte
            });

            let new_index = storage.len() as u16;
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