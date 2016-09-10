pub fn run() {
    unsafe { hello(); }
}

extern { fn hello(); }
