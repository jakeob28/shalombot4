use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Operation {
    pub(crate) op_type: OperationType,
    pub(crate) num1: usize,
    pub(crate) num2: usize,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} = {}",
            self.num1,
            self.op_type.symbol(),
            self.num2,
            self.op_type
                .operate(self.num1, self.num2)
                .expect("cannot print bad operation")
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl OperationType {
    pub(crate) fn operate(self, num1: usize, num2: usize) -> Option<usize> {
        match &self {
            OperationType::Add => Some(num1 + num2),
            OperationType::Subtract => {
                if num1 > num2 {
                    Some(num1 - num2)
                } else {
                    None
                }
            }
            OperationType::Multiply => Some(num1 * num2),
            OperationType::Divide => {
                if num1 % num2 == 0 {
                    Some(num1 / num2)
                } else {
                    None
                }
            }
        }
    }
    fn symbol(self) -> &'static str {
        match &self {
            OperationType::Add => "+",
            OperationType::Subtract => "-",
            OperationType::Multiply => "*",
            OperationType::Divide => "/",
        }
    }
}
