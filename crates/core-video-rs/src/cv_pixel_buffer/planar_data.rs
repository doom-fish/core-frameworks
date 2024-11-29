use std::ptr;

#[derive(Debug)]
pub struct PlanarDataPointer {
    pub data: Option<Vec<u8>>,
    pub number_of_planes: u32,
    pub plane_bytes_per_row: Vec<u32>,
    pub plane_width: Vec<u32>,
    pub plane_height: Vec<u32>,
    pub base_addresses: Vec<Vec<u8>>,
}

impl PlanarDataPointer {
    pub fn new(
        data: Option<Vec<u8>>,
        plane_bytes_per_row: Vec<u32>,
        plane_width: Vec<u32>,
        plane_height: Vec<u32>,
        base_addresses: Vec<Vec<u8>>,
    ) -> PlanarDataPointer {
        PlanarDataPointer {
            data,
            number_of_planes: base_addresses.len() as u32,
            plane_bytes_per_row,
            plane_width,
            plane_height,
            base_addresses,
        }
    }
    pub fn number_of_planes(&self) -> u32 {
        self.number_of_planes
    }
    pub fn as_ptr(&self) -> *mut u8 {
        self.data
            .as_ref()
            .map(|v| v.as_ptr())
            .unwrap_or(ptr::null_mut())
            .cast_mut()
    }

    pub fn data_size(&self) -> u32 {
        self.data.as_ref().map(|v| v.len() as u32).unwrap_or(0)
    }

    pub fn raw_base_addresses(&self) -> *const *const u8 {
        self.base_addresses.as_ptr().cast()
    }
    pub fn plane_bytes_per_row(&self) -> *const u32 {
        self.plane_bytes_per_row.as_ptr()
    }
    pub fn plane_width(&self) -> *const u32 {
        self.plane_width.as_ptr()
    }
    pub fn plane_height(&self) -> *const u32 {
        self.plane_height.as_ptr()
    }
}
