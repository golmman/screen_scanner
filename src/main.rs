use std::os::raw::c_int;
use std::os::raw::c_uint;
use std::thread::sleep;
use std::time::Duration;

use x11::xlib::Button1;
use x11::xlib::ButtonPress;
use x11::xlib::ButtonPressMask;
use x11::xlib::ButtonRelease;
use x11::xlib::ButtonReleaseMask;
use x11::xlib::Display;
use x11::xlib::PointerWindow;
use x11::xlib::XAllPlanes;
use x11::xlib::XButtonEvent;
use x11::xlib::XDefaultRootWindow;
use x11::xlib::XEvent;
use x11::xlib::XFlush;
use x11::xlib::XGetImage;
use x11::xlib::XOpenDisplay;
use x11::xlib::XQueryPointer;
use x11::xlib::XSendEvent;
use x11::xlib::XWarpPointer;
use x11::xlib::ZPixmap;

// https://stackoverflow.com/questions/57078155/draw-border-frame-using-xlib
// https://stackoverflow.com/questions/42013047/difference-between-xtestfakebuttonevent-xsendevent?noredirect=1&lq=1
// https://stackoverflow.com/questions/51406276/simulating-mouse-and-keyboard-input-on-wayland-and-x11

fn click(display: *mut Display, window: u64) {
    let mut window_: u64 = 0;
    let mut root: u64 = 0;
    let mut subwindow: u64 = window;
    let mut x_root = 0;
    let mut y_root = 0;
    let mut x = 0;
    let mut y = 0;
    let mut state: u32 = 0;

    while subwindow != 0 {
        window_ = subwindow;
        unsafe {
            XQueryPointer(
                display,
                window_,
                &mut root as *mut u64,
                &mut subwindow as *mut u64,
                &mut x_root as *mut i32,
                &mut y_root as *mut i32,
                &mut x as *mut i32,
                &mut y as *mut i32,
                &mut state as *mut u32,
            );
        }
    }

    let button_event = XButtonEvent {
        type_: 0,
        serial: 0,
        send_event: 1,
        display,
        window: window_,
        root,
        subwindow,
        time: 0,
        x,
        y,
        x_root,
        y_root,
        state,
        button: Button1,
        same_screen: 1,
    };
    let mut event = XEvent {
        button: button_event,
    };

    unsafe {
        event.button.type_ = ButtonPress;
        XSendEvent(
            display,
            PointerWindow as u64,
            1,
            ButtonPressMask,
            &mut event as *mut XEvent,
        );
        XFlush(display);

        sleep(Duration::from_millis(25));

        event.button.type_ = ButtonRelease;
        XSendEvent(
            display,
            PointerWindow as u64,
            1,
            ButtonReleaseMask,
            &mut event as *mut XEvent,
        );
        XFlush(display);

        sleep(Duration::from_millis(25));
    }
}

fn main() {
    println!("Hello, world!");

    unsafe {
        let display = XOpenDisplay(0 as *const i8);
        let window = XDefaultRootWindow(display);

        const IMAGE_X: c_int = 593;
        const IMAGE_Y: c_int = 267;
        const IMAGE_W: c_uint = 400;
        const IMAGE_H: c_uint = 400;
        const IMAGE_BYTES: usize = (4 * IMAGE_W * IMAGE_H) as usize;

        //click(display, window);
        //return;

        //loop {
        //    click(display, window);
        //    sleep(Duration::from_millis(1000));
        //}
        //return;

        loop {
            let image = XGetImage(
                display,
                window,
                IMAGE_X,
                IMAGE_Y,
                IMAGE_W,
                IMAGE_H,
                XAllPlanes(),
                ZPixmap,
            );
            let pixels1 = std::slice::from_raw_parts((*image).data, IMAGE_BYTES);

            sleep(Duration::from_millis(100));

            let image = XGetImage(
                display,
                window,
                IMAGE_X,
                IMAGE_Y,
                IMAGE_W,
                IMAGE_H,
                XAllPlanes(),
                ZPixmap,
            );
            let pixels2 = std::slice::from_raw_parts((*image).data, IMAGE_BYTES);

            let mut sum_x: i32 = 0;
            let mut sum_y: i32 = 0;
            let mut hits: i32 = 0;

            for i in 0..IMAGE_BYTES / 4 {
                let sub0 = pixels1[4 * i + 0] as i32 - pixels2[4 * i + 0] as i32;
                let sub1 = pixels1[4 * i + 1] as i32 - pixels2[4 * i + 1] as i32;
                let sub2 = pixels1[4 * i + 2] as i32 - pixels2[4 * i + 2] as i32;
                let sub3 = pixels1[4 * i + 3] as i32 - pixels2[4 * i + 3] as i32;

                if sub0 != 0 || sub1 != 0 || sub2 != 0 || sub3 != 0 {
                    let x = i as u32 % IMAGE_W;
                    let y = i as u32 / IMAGE_W;
                    //println!("{} {}", x, y);
                    sum_x += x as i32;
                    sum_y += y as i32;
                    hits += 1;
                }
            }

            let result_x = sum_x / hits + IMAGE_X;
            let result_y = sum_y / hits + IMAGE_Y;

            XWarpPointer(
                display, window, window, 0, 0, 1920, 1080, result_x, result_y,
            );
            XFlush(display);

            sleep(Duration::from_millis(100));

            if hits < 300 || hits > 500 {
                println!("{} hits, not in [300, 500] range, skipping click", hits);
            } else {
                click(display, window);
            }

            sleep(Duration::from_millis(500));
        }
    }
}
