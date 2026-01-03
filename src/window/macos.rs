use objc2::define_class;
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{ClassType, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApp, NSApplication, NSApplicationActivationPolicy, NSBackingStoreType, NSWindow,
    NSWindowDelegate, NSWindowStyleMask,
};

use objc2_foundation::{
    NSAutoreleasePool, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    NSString,
};

#[derive(Debug, Default)]
pub struct AppDelegate;

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppDelegate]
    pub struct WindowDelegate;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for WindowDelegate {}

    // SAFETY: `NSWindowDelegate` has no safety requirements.
    unsafe impl NSWindowDelegate for WindowDelegate {
        #[unsafe(method(windowWillClose:))]
        fn window_will_close(&self, _notification: &NSNotification) {
            // Quit the application when the window is closed.
            unsafe { NSApplication::sharedApplication(self.mtm()).terminate(None) };
        }
    }
);

impl WindowDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppDelegate::default());
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        unsafe { msg_send![super(this), init] }
    }
}

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
            delegate: None,
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

        // Add Delegates
        // let delegate: Retained<WindowDelegate> = Retained::from(WindowDelegate::alloc(mtm));
        // window.setDelegate(Some(delegate.as_ref().cast::<dyn NSWindowDelegate>()));
        //
        let delegate = WindowDelegate::new(mtm);
        window.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

        // std::mem::forget(delegate);

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
