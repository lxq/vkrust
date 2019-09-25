/**
 * https://github.com/unknownue/vulkan-tutorial-rust
 * 第一例子，对应：00_base_code.rs
 * @author XQ Lin
 * @email xqlin@qq.com
 * @date 2019/09/25
 */

// winit 使用新的版本,使用方式原git例子有区别。
use winit::{Event, EventsLoop, Window, WindowEvent, KeyboardInput, ControlFlow};

// const
const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 600;
const WIN_TITLE: &'static str = "00 base";

struct VkApp;

impl VkApp {
    // 初始化设置
    fn init(el: &EventsLoop) -> Window {
        // Window::new(el).unwrap()
        winit::WindowBuilder::new()
            .with_title(WIN_TITLE)
            .with_dimensions((WIN_WIDTH, WIN_HEIGHT).into())
            .build(el)
            .expect("创建窗口失败")
    }

    // 主体运行函数
    pub fn run(el: &mut EventsLoop) {
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
}

fn main() {
    println!("My Vulkan learning with Rust.");

    let mut el = EventsLoop::new();
    // 惯例：不需要使用的变量，以下划线开始。
    let _win = VkApp::init(&el);
    VkApp::run(&mut el);

}