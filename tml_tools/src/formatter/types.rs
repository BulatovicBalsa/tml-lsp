use tml_parser::tml_actions::*;
use super::Format;

impl Format for TypeSpec {
    fn format(&self, indent: usize) -> String {
        match self {
            TypeSpec::SimpleType(t) => t.format(indent),
            TypeSpec::DerivedType(t) => t.format(indent),
            TypeSpec::TensorConstructor(t) => t.format(indent),
        }
    }
}

impl Format for SimpleType {
    fn format(&self, indent: usize) -> String {
        self._type.format(indent)
    }
}

impl Format for SimpleTypeSpec {
    fn format(&self, _indent: usize) -> String {
        match self {
            SimpleTypeSpec::IntT(_)  => "int".to_string(),
            SimpleTypeSpec::UintT(_) => "uint".to_string(),
            SimpleTypeSpec::RealT(_) => "real".to_string(),
            SimpleTypeSpec::BoolT(_) => "bool".to_string(),
            SimpleTypeSpec::StrT(_)  => "str".to_string(),
            SimpleTypeSpec::CharT(_) => "char".to_string(),
        }
    }
}

impl Format for DerivedType {
    fn format(&self, indent: usize) -> String {
        let brackets = count_brackets(&self.brackets);
        format!(
            "{}{}{}",
            self.name.format(indent),
            "[]".repeat(brackets),
            ".type"
        )
    }
}

fn count_brackets(b: &SquareBrackets0) -> usize {
    match b {
        None => 0,
        Some(inner) => 1 + count_brackets_inner(inner),
    }
}

fn count_brackets_inner(b: &SquareBrackets1) -> usize {
    match b {
        SquareBrackets1::SquareBrackets => 0,
        SquareBrackets1::SquareBrackets1(inner) => 1 + count_brackets_inner(inner),
    }
}

impl Format for TensorConstructor {
    fn format(&self, indent: usize) -> String {
        let dims = self.dimensions
            .iter()
            .map(|d| d.format(indent))
            .collect::<Vec<_>>()
            .join(", ");
        format!("tensor<{}, {}>", self._type.format(indent), dims)
    }
}

impl Format for IoConstructor {
    fn format(&self, indent: usize) -> String {
        let direction = match self.direction {
            IoDirection::InT(_)  => "in",
            IoDirection::OutT(_) => "out",
        };
        let flags = match &self.io_flags {
            None => String::new(),
            Some(f) => format!(", {}", f.format(indent)),
        };
        format!(
            "{}<{}, {}{}>",
            direction,
            self._type.format(indent),
            self.address.format(indent),
            flags
        )
    }
}

impl Format for IoFlagsSpec {
    fn format(&self, indent: usize) -> String {
        match self {
            IoFlagsSpec::C1 => "dma".to_string(),
            IoFlagsSpec::C2(c) => format!("dma, {}", c.io_range.format(indent)),
            IoFlagsSpec::C3(c) => c.io_range.format(indent),
            IoFlagsSpec::C4(c) => format!("{}, dma", c.io_range.format(indent)),
        }
    }
}

impl Format for IoRangeSpec {
    fn format(&self, _indent: usize) -> String {
        match self {
            IoRangeSpec::Hil    => "hil".to_string(),
            IoRangeSpec::Ao     => "ao".to_string(),
            IoRangeSpec::Abs    => "abs".to_string(),
            IoRangeSpec::Shared => "shared".to_string(),
            IoRangeSpec::Ext    => "ext".to_string(),
            IoRangeSpec::Sink   => "sink".to_string(),
        }
    }
}