
#[derive(Clone)]
pub enum State {
    Start,
    Customer,
    Product { customer: String },
    End,
}