use std::ptr;
use std::ffi::CString;
use std::sync::Arc;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::raw::HMODULE;
use std::os::windows::raw::FARPROC;
use std::os::windows::ffi::OsStrExt;
use winapi::um::libloaderapi::{LoadLibraryW, GetProcAddress, FreeLibrary};
use winapi::um::winnt::LPCSTR;

// Logger and utility modules would need to be implemented in Rust
use crate::util::log::Logger;
use crate::util::util_string::str_format;
use crate::util::util_win32_compat::LoadLibraryA;

type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn() -> *const c_void;
type PFN_vkVoidFunction = unsafe extern "system" fn() -> c_void;
type VkInstance = *const c_void;
type VkDevice = *const c_void;

pub struct LibraryLoader {
    library: Option<HMODULE>,
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
}

impl LibraryLoader {
    fn load_vulkan_library() -> Option<(HMODULE, PFN_vkGetInstanceProcAddr)> {
        let dll_names = ["winevulkan.dll", "vulkan-1.dll"];

        for dll_name in &dll_names {
            let library = unsafe { LoadLibraryA(CString::new(*dll_name).unwrap().as_ptr()) };

            if library.is_null() {
                continue;
            }

            let proc = unsafe { GetProcAddress(library, "vkGetInstanceProcAddr\0".as_ptr() as LPCSTR) };

            if proc.is_null() {
                unsafe {
                    FreeLibrary(library);
                }
                continue;
            }

            Logger::info(str_format(
                "Vulkan: Found vkGetInstanceProcAddr in {} @ 0x{:x}",
                dll_name,
                proc as usize,
            ));
            return Some((library, unsafe { std::mem::transmute(proc) }));
        }

        Logger::err("Vulkan: vkGetInstanceProcAddr not found");
        None
    }

    pub fn new() -> Self {
        if let Some((library, get_instance_proc_addr)) = Self::load_vulkan_library() {
            LibraryLoader {
                library: Some(library),
                get_instance_proc_addr,
            }
        } else {
            LibraryLoader {
                library: None,
                get_instance_proc_addr: unsafe { std::mem::zeroed() },
            }
        }
    }

    pub fn from_proc(loader_proc: PFN_vkGetInstanceProcAddr) -> Self {
        LibraryLoader {
            library: None,
            get_instance_proc_addr: loader_proc,
        }
    }

    pub fn sym(&self, instance: VkInstance, name: &str) -> PFN_vkVoidFunction {
        let cname = CString::new(name).unwrap();
        unsafe { std::mem::transmute((self.get_instance_proc_addr)(instance, cname.as_ptr())) }
    }

    pub fn sym_global(&self, name: &str) -> PFN_vkVoidFunction {
        self.sym(ptr::null(), name)
    }

    pub fn valid(&self) -> bool {
        !self.get_instance_proc_addr.is_null()
    }
}

impl Drop for LibraryLoader {
    fn drop(&mut self) {
        if let Some(library) = self.library {
            unsafe {
                FreeLibrary(library);
            }
        }
    }
}

pub struct InstanceLoader {
    library: Arc<LibraryLoader>,
    instance: VkInstance,
    owned: bool,
}

impl InstanceLoader {
    pub fn new(library: Arc<LibraryLoader>, owned: bool, instance: VkInstance) -> Self {
        InstanceLoader {
            library,
            instance,
            owned,
        }
    }

    pub fn sym(&self, name: &str) -> PFN_vkVoidFunction {
        self.library.sym(self.instance, name)
    }
}

pub struct DeviceLoader {
    library: Arc<InstanceLoader>,
    get_device_proc_addr: PFN_vkGetInstanceProcAddr,
    device: VkDevice,
    owned: bool,
}

impl DeviceLoader {
    pub fn new(library: Arc<InstanceLoader>, owned: bool, device: VkDevice) -> Self {
        let get_device_proc_addr = unsafe {
            std::mem::transmute(library.sym("vkGetDeviceProcAddr"))
        };
        DeviceLoader {
            library,
            get_device_proc_addr,
            device,
            owned,
        }
    }

    pub fn sym(&self, name: &str) -> PFN_vkVoidFunction {
        unsafe {
            std::mem::transmute((self.get_device_proc_addr)(self.device, CString::new(name).unwrap().as_ptr()))
        }
    }
}

pub struct LibraryFn(LibraryLoader);

impl LibraryFn {
    pub fn new() -> Self {
        LibraryFn(LibraryLoader::new())
    }

    pub fn from_proc(loader_proc: PFN_vkGetInstanceProcAddr) -> Self {
        LibraryFn(LibraryLoader::from_proc(loader_proc))
    }
}

pub struct InstanceFn(InstanceLoader);

impl InstanceFn {
    pub fn new(library: Arc<LibraryLoader>, owned: bool, instance: VkInstance) -> Self {
        InstanceFn(InstanceLoader::new(library, owned, instance))
    }
}

impl Drop for InstanceFn {
    fn drop(&mut self) {
        if self.0.owned {
            unsafe {
                self.vkDestroyInstance(self.0.instance, ptr::null());
            }
        }
    }
}

pub struct DeviceFn(DeviceLoader);

impl DeviceFn {
    pub fn new(library: Arc<InstanceLoader>, owned: bool, device: VkDevice) -> Self {
        DeviceFn(DeviceLoader::new(library, owned, device))
    }
}

impl Drop for DeviceFn {
    fn drop(&mut self) {
        if self.0.owned {
            unsafe {
                self.vkDestroyDevice(self.0.device, ptr::null());
            }
        }
    }
}
