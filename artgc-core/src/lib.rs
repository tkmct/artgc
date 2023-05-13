pub struct Circuit {}

impl Circuit {
    pub fn new() -> Self {
        Circuit {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_construction() {
        let circuit = Circuit::new();
    }
}
