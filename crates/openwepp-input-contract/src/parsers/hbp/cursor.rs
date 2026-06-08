pub(super) struct Cursor<'a> {
    pub(super) data: &'a [u8],
    pub(super) pos: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos }
    }

    pub(super) fn require(&self, count: usize) -> Result<(), &'static str> {
        if self.pos + count > self.data.len() {
            return Err("truncated payload");
        }
        Ok(())
    }

    pub(super) fn u8(&mut self) -> Result<u8, &'static str> {
        self.require(1)?;
        let value = self.data[self.pos];
        self.pos += 1;
        Ok(value)
    }

    pub(super) fn u16(&mut self) -> Result<u16, &'static str> {
        self.require(2)?;
        let value = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    pub(super) fn u32(&mut self) -> Result<u32, &'static str> {
        self.require(4)?;
        let value = u32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    pub(super) fn i32(&mut self) -> Result<i32, &'static str> {
        self.require(4)?;
        let value = i32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    pub(super) fn u64(&mut self) -> Result<u64, &'static str> {
        self.require(8)?;
        let value = u64::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    pub(super) fn i64(&mut self) -> Result<i64, &'static str> {
        self.require(8)?;
        let value = i64::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    pub(super) fn f64(&mut self) -> Result<f64, &'static str> {
        self.require(8)?;
        let value = f64::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    pub(super) fn raw(&mut self, count: usize) -> Result<&'a [u8], &'static str> {
        self.require(count)?;
        let start = self.pos;
        let end = start + count;
        self.pos = end;
        Ok(&self.data[start..end])
    }

    pub(super) fn string(&mut self) -> Result<String, &'static str> {
        let len = self.u32()? as usize;
        let raw = self.raw(len)?;
        std::str::from_utf8(raw)
            .map(|value| value.to_string())
            .map_err(|_| "invalid utf8 string")
    }
}
