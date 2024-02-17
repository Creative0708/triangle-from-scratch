
use std::ptr::null;

use std::ffi::*;

#[link(name = "X11")]
extern "C" {
    // idk if half of these are const pointers or mut pointers, i'm just guessing
    fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
    fn XCloseDisplay(display: *mut Display);
    fn XDefaultScreen(display: *const Display) -> c_int;
    fn XCreateSimpleWindow(display: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint, height: c_uint, border_width: c_uint, border_color: c_ulong, background_color: c_ulong) -> Window;
    fn XRootWindow(display: *const Display, screen: c_int) -> Window;
    fn XMapWindow(display: *mut Display, window: Window);
    fn XNextEvent(display: *mut Display, event: *mut XEvent);
}

type XID = c_ulong;

type Display = XID;
type Window = XID;


// TODO find a way to differentiate between events
#[repr(C)]
struct XEvent {
    event_type: c_int,
    
    _pad: [u8; 188]
}

fn main() {
    unsafe {
        let display = XOpenDisplay(null());
        let screen = XDefaultScreen(display);

        let root_window = XRootWindow(display, screen);

        let window = XCreateSimpleWindow(display, root_window, 100, 100, 480, 360, 1, 0, 0);

        XMapWindow(display, window);

        let mut event: XEvent = std::mem::zeroed();

        loop {
            XNextEvent(display, &mut event);
        }
    }
}