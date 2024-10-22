use std::ptr;

#[derive(Debug)]
pub struct PlanarDataPointer {
    pub data: Option<Vec<u8>>,
    pub number_of_planes: usize,
    pub plane_bytes_per_row: Vec<usize>,
    pub plane_width: Vec<usize>,
    pub plane_height: Vec<usize>,
    pub base_addresses: Vec<Vec<u8>>,
}

impl PlanarDataPointer {
    pub fn new(
        data: Option<Vec<u8>>,
        plane_bytes_per_row: Vec<usize>,
        plane_width: Vec<usize>,
        plane_height: Vec<usize>,
        base_addresses: Vec<Vec<u8>>,
    ) -> PlanarDataPointer {
        PlanarDataPointer {
            data,
            number_of_planes: base_addresses.len(),
            plane_bytes_per_row,
            plane_width,
            plane_height,
            base_addresses,
        }
    }
    pub fn number_of_planes(&self) -> usize {
        self.number_of_planes
    }
    pub fn as_ptr(&self) -> *mut u8 {
        self.data
            .as_ref()
            .map(|v| v.as_ptr())
            .unwrap_or(ptr::null_mut())
            .cast_mut()
    }

    pub fn data_size(&self) -> usize {
        self.data.as_ref().map(|v| v.len()).unwrap_or(0)
    }

    pub fn raw_base_addresses(&self) -> *const *const u8 {
        self.base_addresses.as_ptr().cast()
    }
    pub fn plane_bytes_per_row(&self) -> *const usize {
        self.plane_bytes_per_row.as_ptr()
    }
    pub fn plane_width(&self) -> *const usize {
        self.plane_width.as_ptr()
    }
    pub fn plane_height(&self) -> *const usize {
        self.plane_height.as_ptr()
    }
}
