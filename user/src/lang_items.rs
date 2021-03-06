use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let err = panic_info.message().unwrap();
    if let Some(location) = panic_info.location() {
        error!("Panicked at {}:{}, {}", location.file(), location.line(), err);
    } else {
        error!("Panicked: {}", err);
    }
    loop {}
}
