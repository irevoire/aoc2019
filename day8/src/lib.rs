pub struct Image {
    pub width: usize,
    pub height: usize,
    pub mem: Vec<u8>,
}

impl Image {
    pub fn new(mem: Vec<u8>, width: usize, height: usize) -> Self {
        Image { width, height, mem }
    }

    pub fn layer(&self) -> IterLayer {
        IterLayer {
            image: self,
            layer: 0,
        }
    }

    pub fn layer_n(&self, n: usize) -> &[u8] {
        let start = n * self.width * self.height;
        let end = (n + 1) * self.width * self.height;
        &self.mem[start..end]
    }
}

pub struct IterLayer<'i> {
    image: &'i Image,
    layer: usize,
}

impl<'i> Iterator for IterLayer<'i> {
    type Item = &'i [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if (self.layer * self.image.width * self.image.height) >= self.image.mem.len() {
            None
        } else {
            self.layer += 1;
            Some(self.image.layer_n(self.layer - 1))
        }
    }
}
