/**
 * https://github.com/unknownue/vulkan-tutorial-rust
 * 对应01_instance_creation.rs，创建vkInstance。
 * @author XQ Lin
 * @email xqlin@qq.com
 * @date 2019/11/06
 */

// winit 使用新的版本,使用方式原git例子有区别。
use winit::{Event, EventsLoop, Window, WindowEvent, KeyboardInput, ControlFlow};

use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use std::ffi::CString;
use std::ptr;

// NOTE: 这里使用的是包内的自定义模块，注意路径。
use vkrust::util;
use vkrust::util::consts::*;

// const
const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 600;
// NOTE: static 说明这个变量生命周期和和运行程序一样长，&'static是对静态变量的引用。
const WIN_TITLE: &'static str = "01 create instance";

struct VkApp {
    _entry: ash::Entry,
    inst: ash::Instance,
}

impl VkApp {
    pub fn new() ->VkApp {
        let entry = ash::Entry::new().unwrap();
        // NOTE: 下面为静态方法调用方式，需要带为结构名称。
        let inst = VkApp::create_inst(&entry);

        // NOTE: 销毁工作，通过实现Drop trait来完成。
        VkApp {
            _entry: entry,
            inst: inst,
        }
    }

    // 主体运行函数
    pub fn run(self, el: &mut EventsLoop) {
        el.run_forever(|event |{
            match event {
                Event::WindowEvent {event, ..} => {
                    match event {
                        WindowEvent::CloseRequested => ControlFlow::Break,
                        WindowEvent::KeyboardInput {input, ..} => {
                            match input {
                                KeyboardInput {virtual_keycode, state, ..} => {
                                    // NOTE: 下面可以同时匹配2项，这个功能很强大。
                                    match (virtual_keycode, state) {
                                        (Some(winit::VirtualKeyCode::Escape), winit::ElementState::Released) => ControlFlow::Break,
                                        _ => ControlFlow::Continue,
                                    }
                                },
                                // | _ => ControlFlow::Continue,
                            }
                        },
                        _ => ControlFlow::Continue,
                    }
                },
                _ => ControlFlow::Continue,
            }
        });
    } 

    // NOTE: 不带self的方法，相当于是类的静态方法。
    // 初始化设置
    fn init(el: &EventsLoop) -> Window {
        // Window::new(el).unwrap()
        winit::WindowBuilder::new()
            .with_title(WIN_TITLE)
            .with_dimensions((WIN_WIDTH, WIN_HEIGHT).into())
            .build(el)
            .expect("创建窗口失败")
    }   

    // 创建vk实例
    fn create_inst(entry: &ash::Entry) -> ash::Instance {
        let app_name = CString::new(WIN_TITLE).unwrap();
        let egn_name = CString::new("Vulkan Engine").unwrap();

        let app_info = vk::ApplicationInfo{
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: APP_VERSION,
            p_engine_name: egn_name.as_ptr(),
            engine_version: EGN_VERSION,
            api_version: API_VERSION,
        };

        let ext_name = util::os::req_ext();

        let create_info = vk::InstanceCreateInfo{
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: ext_name.as_ptr(),
            enabled_extension_count: ext_name.len() as u32,
        };

        let inst: ash::Instance = unsafe {
            // NOTE: create_instance()方法在EntryV1_0中。
            entry.create_instance(&create_info, None).expect("创建VK实例失败！")
        };
        inst
    }    
}

impl Drop for VkApp {
    fn drop(&mut self) {
        unsafe {
            self.inst.destroy_instance(None);
        }
    }
}

fn main() {
    println!("My Vulkan learning with Rust.");

    let mut el = EventsLoop::new();
    // NOTE: 惯例——不需要使用的变量，以下划线开始。
    let _win = VkApp::init(&el);
    let _app = VkApp::new();
    _app.run(&mut el);
}