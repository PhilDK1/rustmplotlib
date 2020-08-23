pub struct Normalize {
    pub vmin: Option<usize>,
    pub vmax: Option<usize>,
    pub clip: bool,
}

impl Normalize {
    pub fn new(vmin: Option<usize>, vmax: Option<usize>) -> Normalize {
        Normalize {
            vmin,
            vmax,
            clip: false,
        }
    }

    pub fn clip(&mut self, clip: bool) {
        self.clip = clip;
    }
}
