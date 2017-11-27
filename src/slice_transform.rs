extern crate librocksdb_sys as ffi;

use SliceTransform;

use libc::size_t;

unsafe impl Send for SliceTransform {}
unsafe impl Sync for SliceTransform {}

impl Drop for SliceTransform {
    fn drop(&mut self) {
        unsafe { ffi::rocksdb_slicetransform_destroy(self.inner) }
    }
}

impl Default for SliceTransform {
    fn default() -> SliceTransform { SliceTransform::noop() }
}

impl SliceTransform {
    pub fn fixed_prefix(size: usize) -> SliceTransform {
        SliceTransform{
            inner: unsafe { ffi::rocksdb_slicetransform_create_fixed_prefix(size as size_t) }
        }
    }

    pub fn noop() -> SliceTransform {
        SliceTransform{
            inner: unsafe { ffi::rocksdb_slicetransform_create_noop() }
        }
    }
}

