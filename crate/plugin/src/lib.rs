


pub trait Plugin {
    fn name(&self) -> &str {
        "plugin"
    }

    fn version(&self) -> &str {
        ""
    }

    fn author(&self) -> &str {
        ""
    }

    fn license(&self) -> &str {
        ""
    }

    fn description(&self) -> &str {
        ""
    }

    fn handle_message(&self, message: String);



}

/// 从原始指针中获取字符串
///
/// # Safety
///
/// 调用者必须确保指针指向有效的UTF-8数据，且长度正确
pub fn get_string_from_pointer(pointer: *const u8, length: usize) -> String {
    unsafe {
        let slice = std::slice::from_raw_parts(pointer, length);
        let str = String::from_utf8_lossy(slice).to_string();
        str
    }
}

