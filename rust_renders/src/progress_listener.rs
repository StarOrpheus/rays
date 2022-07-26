pub trait ProgressListener {
    fn update(&mut self, done: usize, total: usize);
}

pub struct StderrListener {}

impl ProgressListener for StderrListener {
    fn update(&mut self, done: usize, total: usize) {
        eprint!("\rLines remaining: {}/{}      ", done, total);
    }
}

pub struct VoidListener {}

impl ProgressListener for VoidListener {
    fn update(&mut self, _done: usize, _total: usize) {}
}
