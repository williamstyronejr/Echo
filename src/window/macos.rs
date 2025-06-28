use objc2::msg_send;
use objc2::rc::Retained;
use objc2::{declare_class, ClassType};
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApp, NSApplication, NSApplicationActivationPolicy, NSBackingStoreType, NSWindow,
    NSWindowStyleMask,
};
use objc2_foundation::{
    NSAutoreleasePool, NSNotification, NSObject, NSPoint, NSRect, NSSize, NSString,
};

declare_class!(
    struct AppDelegate;

    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
    }
);

// extern_methods!(
//     unsafe impl AppDelegate {
//         #[method(windowWillClose:)]
//         fn window_will_close(&self, _notification: &NSNotification) {
//             unsafe {
//                 let app: *mut NSApplication = msg_send![NSApplication::class(), sharedApplication];
//                 let _: () = msg_send![app, terminate: None::<&NSObject>];
//             }
//         }
//     }
// )

//     fn setup_delegate(window: &NSWindow) {
//         unsafe {
//             let delegate: Retained<AppDelegate> = AppDelegate::alloc().init();
//             window.setDelegate(Some(&*delegate));

//             //
//         }
//     }

pub struct Window<'a> {
    title: &'a str,
    app: Retained<NSApplication>,
    mtm: MainThreadMarker,
    delegate: Option<Retained<AppDelegate>>,
}

impl<'a> Window<'a> {
    pub fn new(title: &'a str) -> Self {
        let mtm = MainThreadMarker::new().expect("Error running MacOS App on Main Thread");
        let app = NSApp(mtm);

        Window {
            title,
            app,
            mtm,
            None,
        }
    }

    pub fn show(&self) {
        let window = create_window(self.mtm);
        let title = NSString::from_str(self.title);

        set_window_title(&window, &title);
        open_window(&window);

        self.app
            .setActivationPolicy(NSApplicationActivationPolicy::Regular);
        self.app.activateIgnoringOtherApps(true);

        self.app.run();
    }
}

fn create_window(mtm: MainThreadMarker) -> Retained<NSWindow> {
    unsafe {
        let frame = NSRect::new(NSPoint::new(100.0, 100.0), NSSize::new(800.0, 600.0));

        let window = NSWindow::initWithContentRect_styleMask_backing_defer(
            NSWindow::alloc(mtm),
            frame,
            NSWindowStyleMask::Titled,
            NSBackingStoreType::Buffered,
            false,
        );

        window
    }
}

fn set_window_title(window: &NSWindow, title: &NSString) {
    unsafe {
        let _: () = msg_send![window, setTitle: title];
    }
}

fn open_window(window: &NSWindow) {
    unsafe {
        // let _: () = msg_send![window, makeKeyAndOrderFront: std::ptr::null::<std::ffi::c_void>()];
        // let _: () = msg_send![window, makeKeyAndOrderFront: nil];
        let sender: Option<&NSObject> = None;
        let _: () = msg_send![window, makeKeyAndOrderFront: sender];
    }
}
