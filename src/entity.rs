pub trait Entity{
    // amount is vec so that more information can be given
    // ex: if an entity can only move forward. a vec of [0]
    // is all that's needed
    fn r#move(&mut self, amount: Vec<f32>);
}