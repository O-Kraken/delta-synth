pub trait AudioModule: Sync + Send{
    fn process(&mut self, output: &mut [f32]);
}

