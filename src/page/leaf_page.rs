/// LeafPageElement 结构体，表示叶子页面上的一个节点
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LeafPageElement {
    pub flags: u32,
    pub pos: u32,
    pub ksize: u32,
    pub vsize: u32,
}

impl LeafPageElement {
    /// 获取节点的键
    pub fn key<'a>(&self, page_data: &'a [u8]) -> Option<&'a [u8]> {
        let start = self.pos as usize;
        let end = start.checked_add(self.ksize as usize)?;
        page_data.get(start..end)
    }

    /// 获取节点的值
    pub fn value<'a>(&self, page_data: &'a [u8]) -> Option<&'a [u8]> {
        let start = (self.pos + self.ksize) as usize;
        let end = start.checked_add(self.vsize as usize)?;
        page_data.get(start..end)
    }
}
