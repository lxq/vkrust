/**
 * os 定义与操作系统平台相的辅助操作。
 * 
 * @author XQ Lin
 * @email xqlin@qq.com
 * @date 2019/11/07
 */

use ash::vk;
use ash::version::{EntryV1_1, InstanceV1_1};

use ash::extensions::khr::Surface;
use ash::extensions::ext::DebugUtils;

// NOTE: 平台编译选项。 当前只考虑Windows。
#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;



// required extension name 
// --->

#[cfg(all(windows))]
pub fn req_ext() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

// <---