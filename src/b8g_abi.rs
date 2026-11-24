//! Awesome-OS B8G ABI v3 (`B8G_ABI_MAGIC_V3`) & 64-byte aligned `B8G_ARENA_V1`
//! bridge for `chonkstep` <-> `libsectium_display` zero-copy surface descriptors.

/// Canonical Awesome-OS B8G ABI v3 magic (`"BGV3"` in little-endian).
pub const B8G_ABI_MAGIC_V3: u32 = 0x33564742u32;

/// Canonical `B8G_ARENA_V1` header version.
pub const B8G_ARENA_VERSION_V1: u32 = 1;

/// Required cache-line alignment for all `B8G_ARENA_V1` descriptors.
pub const B8G_ARENA_ALIGN: usize = 64;

/// 64-byte aligned `B8G_ARENA_V1` shared-memory arena header.
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct B8gArenaHeaderV1 {
    pub magic: u32,
    pub version: u32,
    pub flags: u32,
    pub reserved0: u32,
    pub arena_bytes: u64,
    pub generation: u64,
    pub write_index: u64,
    pub read_index: u64,
    pub surface_table_offset: u64,
    pub surface_capacity: u32,
    pub surface_stride: u32,
}

impl B8gArenaHeaderV1 {
    /// Construct a validated `B8G_ARENA_V1` header stamped with `B8G_ABI_MAGIC_V3`.
    pub const fn new(arena_bytes: u64, surface_capacity: u32) -> Self {
        Self {
            magic: B8G_ABI_MAGIC_V3,
            version: B8G_ARENA_VERSION_V1,
            flags: 0,
            reserved0: 0,
            arena_bytes,
            generation: 1,
            write_index: 0,
            read_index: 0,
            surface_table_offset: std::mem::size_of::<Self>() as u64,
            surface_capacity,
            surface_stride: std::mem::size_of::<LibsectiumDisplaySurfaceV1>() as u32,
        }
    }

    /// Returns `true` if the header carries the canonical `B8G_ABI_MAGIC_V3` magic and version.
    pub const fn is_valid(&self) -> bool {
        self.magic == B8G_ABI_MAGIC_V3 && self.version == B8G_ARENA_VERSION_V1
    }
}

/// 64-byte aligned `libsectium_display` zero-copy Wayland surface descriptor.
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LibsectiumDisplaySurfaceV1 {
    pub surface_id: u64,
    pub dmabuf_offset: u64,
    pub dmabuf_fd: i32,
    pub drm_fourcc: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub scale_milli: u32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub z_order: i32,
    pub surface_flags: u32,
    pub frame_seq: u64,
    pub damage_gen: u64,
}

impl LibsectiumDisplaySurfaceV1 {
    /// Construct a new `libsectium_display` surface descriptor.
    pub const fn new(surface_id: u64, width: u32, height: u32, stride: u32, drm_fourcc: u32) -> Self {
        Self {
            surface_id,
            dmabuf_offset: 0,
            dmabuf_fd: -1,
            drm_fourcc,
            width,
            height,
            stride,
            scale_milli: 1000,
            pos_x: 0,
            pos_y: 0,
            z_order: 0,
            surface_flags: 0,
            frame_seq: 0,
            damage_gen: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn b8g_abi_magic_and_alignment() {
        assert_eq!(B8G_ABI_MAGIC_V3, 0x33564742u32);
        assert_eq!(&B8G_ABI_MAGIC_V3.to_le_bytes(), b"BGV3");

        assert_eq!(align_of::<B8gArenaHeaderV1>(), B8G_ARENA_ALIGN);
        assert_eq!(size_of::<B8gArenaHeaderV1>(), 64);
        assert_eq!(align_of::<LibsectiumDisplaySurfaceV1>(), B8G_ARENA_ALIGN);
        assert_eq!(size_of::<LibsectiumDisplaySurfaceV1>(), 64);
    }

    #[test]
    fn arena_header_and_surface_descriptor_roundtrip() {
        let header = B8gArenaHeaderV1::new(4096, 16);
        assert!(header.is_valid());
        assert_eq!(header.magic, B8G_ABI_MAGIC_V3);
        assert_eq!(header.surface_table_offset, 64);
        assert_eq!(header.surface_stride, 64);

        let surface = LibsectiumDisplaySurfaceV1::new(1, 2560, 1600, 10240, 0x34325258);
        assert_eq!(surface.surface_id, 1);
        assert_eq!(surface.width, 2560);
        assert_eq!(surface.height, 1600);
        assert_eq!(surface.scale_milli, 1000);
    }
}
