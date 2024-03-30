use std::thread;
use std::rc::Rc;
use std::marker::PhantomData;

trait PrintToValidate {
    fn ptv(&self) {
        println!("valid");
    }
}

struct BasicStruct {
    v: i32,
}

impl BasicStruct {
    fn new(v: i32) -> Self {
        BasicStruct{v}
    }
}

impl PrintToValidate for BasicStruct {}

struct GenericStruct<T> {
    v: T,
}

impl<T> GenericStruct<T> {
    fn new(v: T) -> Self {
        GenericStruct{v}
    }
}

impl<T> PrintToValidate for GenericStruct<T> {}

struct PtrStruct {
    v: *const i32,
}

impl PtrStruct {
    fn new(v: *const i32) -> Self {
        PtrStruct{v}
    }
}

impl PrintToValidate for PtrStruct {}

struct SendPtrStruct {
    v: *const i32,
}

impl SendPtrStruct {
    fn new(v: *const i32) -> Self {
        SendPtrStruct{v}
    }
}

impl PrintToValidate for SendPtrStruct {}
unsafe impl Send for SendPtrStruct {} // Send is one of the marker traits that does not include any real methods

fn test_send_trait() {
    let bs = BasicStruct::new(1);
    let bs_handle = thread::spawn(move || {
        bs.ptv();
    });
    bs_handle.join().unwrap();

    let gs = GenericStruct::new(1);
    let gs_handle = thread::spawn(move || {
        gs.ptv();
    });
    gs_handle.join().unwrap();

    // not ok, because after the type is inferred to Rc, compiler finds Rc does not implement Send
    let gs2 = GenericStruct::new(Rc::new(1));
    gs2.ptv();
    // let gs2_handle = thread::spawn(move || {
    //     gs2.ptv();
    // });
    // gs2_handle.join().unwrap();

    // raw pointer does not implement send, so the struct does not either
    let x = 1;
    let x_ptr: *const i32 = &x;
    let ps = PtrStruct::new(x_ptr);
    ps.ptv();
    // let ps_handle = thread::spawn(move || {
    //     ps.ptv();
    // });
    // ps_handle.join().unwrap();

    // if we explicitly impl Send, then it can be thread communicated
    let sx = 1;
    let sx_ptr: *const i32 = &sx;
    let sps = SendPtrStruct::new(x_ptr);
    let sps_handle = thread::spawn(move || {
        sps.ptv();
    });
    sps_handle.join().unwrap();
}

fn main() {
    test_send_trait();
}
