#[cfg(unix)]
use std::ffi::c_void;
#[cfg(windows)]
use winapi::ctypes::c_void;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct PageSize(pub usize);

impl From<usize> for PageSize {
    fn from(i: usize) -> Self {
        Self(i)
    }
}

impl PageSize {
    #[cfg(unix)]
    #[inline]
    pub fn from_system() -> Self {
        use nix::unistd::{sysconf, SysconfVar};

        let page_size = sysconf(SysconfVar::PAGE_SIZE).unwrap().unwrap() as usize;
        Self(page_size)
    }

    #[cfg(windows)]
    #[inline]
    pub fn from_system() -> Self {
        use std::alloc::{alloc, dealloc, Layout};
        use winapi::um::sysinfoapi;

        let size = std::mem::size_of::<sysinfoapi::SYSTEM_INFO>();
        let layout = Layout::from_size_align(size, 8).unwrap();
        let sysinfo = unsafe { alloc(layout) } as *mut sysinfoapi::SYSTEM_INFO;

        let page_size = unsafe {
            sysinfoapi::GetSystemInfo(sysinfo);
            (*sysinfo).dwPageSize as usize
        };

        unsafe {
            dealloc(sysinfo as *mut u8, layout);
        }
        Self(page_size)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PageHandle {
    ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl PageHandle {
    #[inline]
    pub fn from(page_size: PageSize, src: &[u8]) -> Self {
        let r = Self::new(src.len(), page_size);
        unsafe {
            std::ptr::copy(src.as_ptr(), r.ptr, r.len);
        }
        r.make_page_executable();
        r
    }

    #[inline]
    pub fn store(&self, src: &[u8]) {
        if src.len() > self.len {
            panic!("store length invilid");
        }
        unsafe {
            std::ptr::copy(src.as_ptr(), self.ptr, self.len);
        }
    }

    #[inline]
    pub fn get_ptr(&self) -> *const u8 {
        self.ptr
    }
}

#[cfg(unix)]
impl PageHandle {
    #[inline]
    pub fn new(size: usize, page_size: PageSize) -> Self {
        use nix::sys::mman::{mmap, MapFlags, ProtFlags};
        unsafe {
            let page_size = page_size.0;
            let size_mod = size % page_size;
            let alloc_size = if size_mod == 0 {
                size
            } else {
                size + page_size - size_mod
            };
            let ptr = mmap(
                std::ptr::null_mut(),
                alloc_size,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS,
                -1,
                0,
            )
            .unwrap();
            if ptr.is_null() {
                panic!("VirtualAlloc failed");
            }
            PageHandle {
                ptr: ptr as *mut u8,
                len: size,
                cap: alloc_size,
            }
        }
    }

    #[inline]
    pub fn make_page_executable(&self) {
        use nix::sys::mman::{mprotect, ProtFlags};
        unsafe {
            mprotect(
                self.ptr as *mut c_void,
                self.cap,
                ProtFlags::PROT_READ | ProtFlags::PROT_EXEC,
            )
            .unwrap();
        }
    }
}

#[cfg(windows)]
impl PageHandle {
    #[inline]
    pub fn new(size: usize, page_size: PageSize) -> Self {
        use winapi::um::{memoryapi, winnt};
        unsafe {
            let page_size = page_size.0;
            let size_mod = size % page_size;
            let alloc_size = if size_mod == 0 {
                size
            } else {
                size + page_size - size_mod
            };
            let ptr = memoryapi::VirtualAlloc(
                std::ptr::null_mut(),
                alloc_size,
                winnt::MEM_COMMIT,
                winnt::PAGE_READWRITE,
            );
            if ptr.is_null() {
                panic!("VirtualAlloc failed");
            }
            PageHandle {
                ptr: ptr as *mut u8,
                len: size,
                cap: alloc_size,
            }
        }
    }

    #[inline]
    pub fn make_page_executable(&self) {
        use winapi::um::{memoryapi, winnt};
        unsafe {
            let mut flag = winnt::PAGE_READWRITE;
            let r = memoryapi::VirtualProtect(
                self.ptr as *mut c_void,
                self.cap,
                winnt::PAGE_EXECUTE_READ,
                &mut flag,
            );
            if r == 0 {
                panic!("VirtualProtect failed");
            }
        }
    }
}

#[cfg(unix)]
impl Drop for PageHandle {
    #[inline]
    fn drop(&mut self) {
        use nix::sys::mman::munmap;
        unsafe {
            munmap(self.ptr as *mut c_void, self.cap).unwrap();
        }
    }
}

#[cfg(windows)]
impl Drop for PageHandle {
    #[inline]
    fn drop(&mut self) {
        use winapi::um::{memoryapi, winnt};
        unsafe {
            memoryapi::VirtualFree(self.ptr as *mut c_void, 0, winnt::MEM_RELEASE);
        }
    }
}
