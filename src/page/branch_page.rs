use crate::page::Pgid;
/// BranchPageElement 结构体，表示分支页面上的一个节点
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BranchPageElement {
    pub pos: u32,
    pub ksize: u32,
    pub pgid: Pgid,
}

impl BranchPageElement {
    /// 获取节点的键
    pub fn key<'a>(&self, page_data: &'a [u8]) -> Option<&'a [u8]> {
        let start = self.pos as usize;
        let end = start.checked_add(self.ksize as usize)?;
        page_data.get(start..end)
    }
}
