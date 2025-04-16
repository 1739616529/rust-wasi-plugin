use macro_plugin::rtools_plugin;
use plugin::Plugin;

#[derive(Default)]
#[rtools_plugin]
pub struct ScreenKit {
    // 可以在这里添加屏幕相关的字段
}

impl ScreenKit {
    pub fn new() -> Self {
        Self {}
    }

    // 添加一些屏幕功能的方法
    pub fn get_screen_dimensions(&self) -> (u32, u32) {
        // 示例实现，实际应用中应返回真实值
        (1920, 1080)
    }
}

impl Plugin for ScreenKit {
    fn name(&self) -> &str {
        "ScreenKit"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn author(&self) -> &str {
        "Your Name"
    }

    fn license(&self) -> &str {
        "MIT"
    }

    fn description(&self) -> &str {
        "Kit for screen-related functionality"
    }

    fn handle_message(&self, message: String) {
        // 在这里处理从主机传递来的消息
        println!("ScreenKit插件收到消息: {}", message);
    }
}
