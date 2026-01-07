pub trait DataSource {
    type Sample;
    fn next(&mut self) -> Option<Self::Sample>; //source needs to implement type Sample
}

