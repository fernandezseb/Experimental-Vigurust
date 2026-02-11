pub struct ByteArray {
    pub bytes: Vec<u8>,
    current: usize
}

impl ByteArray {
    pub fn new(bytes: Vec<u8>, current: usize) -> Self {
        Self { bytes, current }
    }
    pub fn read_u32(&mut self) -> u32 {
        let buffer: [u8; 4] = [
            self.bytes[self.current],
            self.bytes[self.current+1],
            self.bytes[self.current+2],
            self.bytes[self.current+3],
        ];
        self.current += 4;
        u32::from_be_bytes(buffer)
    }

    pub fn read_u16(&mut self) -> u16 {
        let buffer: [u8; 2] = [
            self.bytes[self.current],
            self.bytes[self.current+1],
        ];
        self.current += 2;
        u16::from_be_bytes(buffer)
    }    

    pub fn read_u8(&mut self) -> u8 {
        self.current += 1;
        self.bytes[self.current-1]
    }

    pub fn read_bytes(&mut self, size: usize) -> &[u8] {
        let bytes = &self.bytes[self.current..(self.current+size)];
        self.current += size;
        bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

}