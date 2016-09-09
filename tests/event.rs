#![cfg(windows)]

extern crate futures;
extern crate kernel32;
extern crate tokio_core;
extern crate tokio_signal;
extern crate winapi;

use std::sync::mpsc::channel;
use std::sync::{Once, ONCE_INIT, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use futures::Future;
use futures::stream::Stream;
use tokio_core::reactor::{Core, Timeout};
use tokio_signal::windows::Event;

static INIT: Once = ONCE_INIT;
static mut LOCK: *mut Mutex<()> = 0 as *mut _;

fn lock() -> MutexGuard<'static, ()> {
    unsafe {
        INIT.call_once(|| {
            LOCK = Box::into_raw(Box::new(Mutex::new(())));
            let (tx, rx) = channel();
            thread::spawn(move || {
                let mut lp = Core::new().unwrap();
                let handle = lp.handle();
                let _events = lp.run(Event::ctrl_c(&handle)).unwrap();
                tx.send(()).unwrap();
                drop(lp.run(futures::empty::<(), ()>()));
            });
            rx.recv().unwrap();
        });
        (*LOCK).lock().unwrap()
    }
}

#[test]
fn simple() {
    let _lock = lock();

    let mut lp = Core::new().unwrap();
    let handle = lp.handle();
    let event = lp.run(Event::ctrl_break(&handle)).unwrap();
    unsafe {
        assert!(kernel32::GenerateConsoleCtrlEvent(winapi::CTRL_BREAK_EVENT, 0) != 0);
    }
    lp.run(event.into_future()).ok().unwrap();
}

// #[test]
// fn notify_both() {
//     let _lock = lock();
//
//     let mut lp = Core::new().unwrap();
//     let handle = lp.handle();
//     let event1 = lp.run(Event::new(libc::SIGUSR2, &handle)).unwrap();
//     let event2 = lp.run(Event::new(libc::SIGUSR2, &handle)).unwrap();
//     unsafe {
//         assert_eq!(libc::kill(libc::getpid(), libc::SIGUSR2), 0);
//     }
//     lp.run(event1.into_future().join(event2.into_future())).ok().unwrap();
// }
//
// #[test]
// fn drop_then_get_a_event() {
//     let _lock = lock();
//
//     let mut lp = Core::new().unwrap();
//     let handle = lp.handle();
//     let event = lp.run(Event::new(libc::SIGUSR1, &handle)).unwrap();
//     drop(event);
//     unsafe {
//         assert_eq!(libc::kill(libc::getpid(), libc::SIGUSR1), 0);
//     }
//     let timeout = Timeout::new(Duration::from_millis(1), &lp.handle()).unwrap();
//     lp.run(timeout).unwrap();
// }
//
// #[test]
// fn twice() {
//     let _lock = lock();
//
//     let mut lp = Core::new().unwrap();
//     let handle = lp.handle();
//     let event = lp.run(Event::new(libc::SIGUSR1, &handle)).unwrap();
//     unsafe {
//         assert_eq!(libc::kill(libc::getpid(), libc::SIGUSR1), 0);
//     }
//     let (num, event) = lp.run(event.into_future()).ok().unwrap();
//     assert_eq!(num, Some(libc::SIGUSR1));
//     unsafe {
//         assert_eq!(libc::kill(libc::getpid(), libc::SIGUSR1), 0);
//     }
//     lp.run(event.into_future()).ok().unwrap();
// }
