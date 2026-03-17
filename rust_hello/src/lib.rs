use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::ArrayBuffer;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026-03-17
///
/// https://napi.rs/docs/concepts/values

/// 测试布尔类型的参数传输
/// @return 取反
#[napi]
pub fn test_bool(value: bool) -> bool {
    !value
}

/// 测试整数类型的参数传输
#[napi]
pub fn test_int(value: i32) -> i32 {
    value
}

/// 测试双精度浮点类型的参数传输
#[napi]
pub fn test_double(value: f64) -> f64 {
    value
}

/// 测试字符串类型的参数传输
#[napi]
pub fn test_string(value: String) -> String {
    String::from(value.to_string())
}

/// 测试字节类型的参数传输
#[napi]
pub fn test_bytes(value: Vec<u8>) -> Vec<u8> {
    Vec::from(value.to_vec())
}

/// 测试字节缓冲类型的参数传输
/// - https://ohos.rs/docs/more/buffer
#[napi]
pub fn test_buffer(buf: ArrayBuffer) -> ArrayBuffer {
    ArrayBuffer::from(buf)
}
/*#[napi]
pub fn test_buffer(buf: Buffer) -> Buffer {
    let buf: Vec<u8> = buf.into();
    Buffer::from(buf)
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func() {
        println!("test_bool->{}", test_bool(true));
        println!("test_int->{}", test_int(1));
        println!("test_double->{}", test_double(1.1));
        println!("test_string->{}", test_string(String::from("test_string")));
        println!("test_bytes->{}", test_bytes(Vec::from(vec![1, 2, 3])).len());
    }
}
