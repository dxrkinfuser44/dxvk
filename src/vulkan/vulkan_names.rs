use std::fmt;

macro_rules! enum_name {
    ($e:expr, $($name:pat => $value:expr),*) => {
        match $e {
            $($name => write!(f, stringify!($name)),)*
            _ => write!(f, "{:?}", $e),
        }
    };
}

impl fmt::Display for vk::VkPipelineCacheHeaderVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        enum_name!(self,
            vk::VK_PIPELINE_CACHE_HEADER_VERSION_ONE => "VK_PIPELINE_CACHE_HEADER_VERSION_ONE"
        )
    }
}

impl fmt::Display for vk::VkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        enum_name!(self,
            vk::VK_SUCCESS => "VK_SUCCESS",
            vk::VK_NOT_READY => "VK_NOT_READY",
            vk::VK_TIMEOUT => "VK_TIMEOUT",
            vk::VK_EVENT_SET => "VK_EVENT_SET",
            vk::VK_EVENT_RESET => "VK_EVENT_RESET",
            vk::VK_INCOMPLETE => "VK_INCOMPLETE",
            vk::VK_ERROR_OUT_OF_HOST_MEMORY => "VK_ERROR_OUT_OF_HOST_MEMORY",
            vk::VK_ERROR_OUT_OF_DEVICE_MEMORY => "VK_ERROR_OUT_OF_DEVICE_MEMORY",
            vk::VK_ERROR_INITIALIZATION_FAILED => "VK_ERROR_INITIALIZATION_FAILED",
            vk::VK_ERROR_DEVICE_LOST => "VK_ERROR_DEVICE_LOST",
            vk::VK_ERROR_MEMORY_MAP_FAILED => "VK_ERROR_MEMORY_MAP_FAILED",
            vk::VK_ERROR_LAYER_NOT_PRESENT => "VK_ERROR_LAYER_NOT_PRESENT",
            vk::VK_ERROR_EXTENSION_NOT_PRESENT => "VK_ERROR_EXTENSION_NOT_PRESENT",
            vk::VK_ERROR_FEATURE_NOT_PRESENT => "VK_ERROR_FEATURE_NOT_PRESENT",
            vk::VK_ERROR_INCOMPATIBLE_DRIVER => "VK_ERROR_INCOMPATIBLE_DRIVER",
            vk::VK_ERROR_TOO_MANY_OBJECTS => "VK_ERROR_TOO_MANY_OBJECTS",
            vk::VK_ERROR_FORMAT_NOT_SUPPORTED => "VK_ERROR_FORMAT_NOT_SUPPORTED",
            vk::VK_ERROR_FRAGMENTED_POOL => "VK_ERROR_FRAGMENTED_POOL",
            vk::VK_ERROR_SURFACE_LOST_KHR => "VK_ERROR_SURFACE_LOST_KHR",
            vk::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => "VK_ERROR_NATIVE_WINDOW_IN_USE_KHR",
            vk::VK_SUBOPTIMAL_KHR => "VK_SUBOPTIMAL_KHR",
            vk::VK_ERROR_OUT_OF_DATE_KHR => "VK_ERROR_OUT_OF_DATE_KHR",
            vk::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => "VK_ERROR_INCOMPATIBLE_DISPLAY_KHR",
            vk::VK_ERROR_VALIDATION_FAILED_EXT => "VK_ERROR_VALIDATION_FAILED_EXT",
            vk::VK_ERROR_INVALID_SHADER_NV => "VK_ERROR_INVALID_SHADER_NV",
            vk::VK_ERROR_OUT_OF_POOL_MEMORY_KHR => "VK_ERROR_OUT_OF_POOL_MEMORY_KHR",
            vk::VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR => "VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR"
        )
    }
}

// Implement similar Display traits for other Vulkan enums following the pattern above

fn main() {
    // Example usage
    let result = vk::VK_SUCCESS;
    println!("{}", result);
}
