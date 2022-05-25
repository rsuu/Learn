pub enum BTNode<T> {
    Leaf(T),
    Branch {
        left: Box<BTNode<T>>,
        right: Box<BTNode<T>>,
        op: Box<dyn Fn(T, T) -> T>,
    },
}

impl<T> BTNode<T>
where
    T: std::ops::Add<Output = T>,
{
    pub fn add<L, R>(left: L, right: R) -> Self
    where
        L: Into<BTNode<T>>,
        R: Into<BTNode<T>>,
    {
        BTNode::Branch {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l + r),
        }
    }
}

impl<T> BTNode<T>
where
    T: std::ops::Sub<Output = T>,
{
    pub fn sub<L, R>(left: L, right: R) -> Self
    where
        L: Into<BTNode<T>>,
        R: Into<BTNode<T>>,
    {
        BTNode::Branch {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l - r),
        }
    }
}

impl<T> BTNode<T>
where
    T: std::ops::Mul<Output = T>,
{
    pub fn mul<L, R>(left: L, right: R) -> Self
    where
        L: Into<BTNode<T>>,
        R: Into<BTNode<T>>,
    {
        BTNode::Branch {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l * r),
        }
    }
}

impl<T> BTNode<T>
where
    T: std::ops::Div<Output = T>,
{
    pub fn div<L, R>(left: L, right: R) -> Self
    where
        L: Into<BTNode<T>>,
        R: Into<BTNode<T>>,
    {
        BTNode::Branch {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l / r),
        }
    }
}

impl<T> BTNode<T> {
    pub fn new(t: T) -> Self {
        BTNode::Leaf(t)
    }

    pub fn value(self) -> T {
        match self {
            Self::Leaf(t) => t,
            Self::Branch { left, right, op } => op(left.value(), right.value()),
        }
    }
}

impl<T> From<T> for BTNode<T> {
    fn from(t: T) -> BTNode<T> {
        BTNode::Leaf(t)
    }
}

fn main() {
    // (10 - (2 * 2)) + (8 + (10 / 2))
    let bt: BTNode<i32> = BTNode::add(
        BTNode::sub(10, BTNode::mul(2, 2)),
        BTNode::add(8, BTNode::div(10, 2)),
    );

    println!("{}", bt.value());
}
