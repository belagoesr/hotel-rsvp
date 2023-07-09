#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CustomerType {
    Regular,
    Rewards,
}

#[derive(Debug, PartialEq)]
pub struct Customer {
    pub customer_type: CustomerType,
}
