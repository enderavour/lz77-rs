#[derive(Clone, Copy)]
pub struct LzToken
{
    offset: u16,
    match_length: u16,
    code: u8
}

impl LzToken
{
    pub fn new() -> Self
    {
        LzToken { offset: 0, match_length: 0, code: 0 }
    }

    pub fn to_bytes(&self) -> [u8; 5]
    {
        let mut token_bytes = [0u8; 5];
        token_bytes[0] = (self.offset & 0xFF) as u8;
        token_bytes[1] = ((self.offset >> 8) & 0xFF) as u8;
        token_bytes[2] = (self.match_length & 0xFF) as u8;
        token_bytes[3] = ((self.match_length >> 8) & 0xFF) as u8;
        token_bytes[4] = self.code;
        token_bytes
    }

    pub fn from_bytes(bytes: [u8; 5]) -> Self
    {
        LzToken { 
            offset: u16::from_le_bytes([bytes[0], bytes[1]]),
            match_length: u16::from_le_bytes([bytes[2], bytes[3]]),
            code: bytes[4]
        }
    }
}

pub fn compress(input: &[u8], buffer: usize, look_ahead: i32) -> Vec<LzToken>
{
    let mut pos = 0;
    let mut data = Vec::new();

    while pos < input.len()
    {
        let mut tok = LzToken::new();
        tok.code = input[pos];

        let max_off = if pos < buffer { pos } else { buffer };
        let max_search_len = if (pos + look_ahead as usize) > input.len() { input.len() - pos } else { look_ahead as usize };

        for offset in 1..=max_off
        {
            let mut len = 0;
            while (len < max_search_len) && (input[pos - offset + len] == input[pos + len])
            {
                len += 1;
            }

            if len > tok.match_length as usize
            {
                tok.offset = offset as u16;
                tok.match_length = len as u16;
               
                if pos + len < input.len()
                {
                    tok.code = input[pos + len];
                }
                else { tok.code = 0; }
            }
        }
        data.push(tok);
        pos += (tok.match_length + 1) as usize;
    }
    data
}

pub fn decompress(compressed: &Vec<LzToken>) -> Vec<u8>
{
    let mut decompressed = Vec::new();

    for tok in compressed
    {
        if tok.offset == 0
        {
            decompressed.push(tok.code);
        }
        else 
        {
            let start_pos = decompressed.len() - tok.offset as usize;
            let end_pos = start_pos + tok.match_length as usize;

            for i in start_pos..end_pos
            {
                decompressed.push(decompressed[i]);
            }

            decompressed.push(tok.code);
        }
    }
    decompressed
}