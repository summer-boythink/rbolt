mod branch_page;
mod leaf_page;
pub mod pgids;

use crate::db::Meta;
use crate::page::leaf_page::LeafPageElement;
use std::mem;
use std::slice;

use branch_page::BranchPageElement;

pub const PAGE_HEADER_SIZE: usize = std::mem::size_of::<Page>() - std::mem::size_of::<usize>();
pub const MIN_KEYS_PER_PAGE: usize = 2;

pub const BRANCH_PAGE_FLAG: u16 = 0x01;
pub const LEAF_PAGE_FLAG: u16 = 0x02;
pub const META_PAGE_FLAG: u16 = 0x04;
pub const FREELIST_PAGE_FLAG: u16 = 0x10;

pub const BUCKET_LEAF_FLAG: u32 = 0x01;
/// 页标识类型
pub type Pgid = u64;

/// PageInfo 结构体，表示页面的可读信息
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageInfo {
    pub id: usize,
    pub page_type: String,
    pub count: usize,
    pub overflow_count: usize,
}

/// Page 结构体，表示一个页面
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Page {
    pub id: Pgid,
    pub flags: u16,
    pub count: u16,
    pub overflow: u32,
    pub data: Box<[u8]>, // 使用 Box<[u8]> 安全存储页面数据
}

impl Page {
    /// 返回页面的类型字符串
    pub fn typ(&self) -> String {
        match self.flags {
            BRANCH_PAGE_FLAG => "branch".to_string(),
            LEAF_PAGE_FLAG => "leaf".to_string(),
            META_PAGE_FLAG => "meta".to_string(),
            FREELIST_PAGE_FLAG => "freelist".to_string(),
            _ => format!("unknown<{:02x}>", self.flags),
        }
    }

    /// 获取页面的元数据部分
    pub fn meta(&self) -> Option<&Meta> {
        let meta_size = mem::size_of::<Meta>();
        if self.data.len() >= meta_size {
            Some(unsafe { &*(self.data.as_ptr() as *const Meta) })
        } else {
            None
        }
    }

    /// 获取指定索引的叶子页面元素
    pub fn leaf_page_element(&self, index: usize) -> Option<&LeafPageElement> {
        let element_size = mem::size_of::<LeafPageElement>();
        let offset = index.checked_mul(element_size)?;
        self.data
            .get(offset..offset + element_size)
            .map(|slice| unsafe { &*(slice.as_ptr() as *const LeafPageElement) })
    }

    /// 获取所有叶子页面元素
    pub fn leaf_page_elements(&self) -> &[LeafPageElement] {
        let element_size = mem::size_of::<LeafPageElement>();
        let count = self.count as usize;
        let total_size = count.checked_mul(element_size).unwrap_or(0);
        let slice = &self.data[..total_size];
        unsafe { slice::from_raw_parts(slice.as_ptr() as *const LeafPageElement, count) }
    }

    /// 获取指定索引的分支页面元素
    pub fn branch_page_element(&self, index: usize) -> Option<&BranchPageElement> {
        let element_size = mem::size_of::<BranchPageElement>();
        let offset = index.checked_mul(element_size)?;
        self.data
            .get(offset..offset + element_size)
            .map(|slice| unsafe { &*(slice.as_ptr() as *const BranchPageElement) })
    }

    /// 获取所有分支页面元素
    pub fn branch_page_elements(&self) -> &[BranchPageElement] {
        let element_size = mem::size_of::<BranchPageElement>();
        let count = self.count as usize;
        let total_size = count.checked_mul(element_size).unwrap_or(0);
        let slice = &self.data[..total_size];
        unsafe { slice::from_raw_parts(slice.as_ptr() as *const BranchPageElement, count) }
    }

    /// 将页面数据的前 `n` 字节以十六进制格式打印到标准错误
    pub fn hexdump(&self, n: usize) {
        let slice = &self.data[..n.min(self.data.len())];
        eprintln!("{:x?}", slice);
    }
}
