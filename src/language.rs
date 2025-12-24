use std::collections::HashMap;

use smol_str::SmolStr;

pub struct Language {
    pub symbols: HashMap<SymbolToken, SymbolData>,
}

impl Language {
    pub fn maybe_symbol_name(&self, symbol: Symbol) -> Option<SymbolName<'_>> {
        match symbol {
            Symbol::Error => Some(SymbolName::Error),
            Symbol::ErrorRepeat => Some(SymbolName::ErrorRepeat),
            Symbol::Symbol(symbol) => self
                .symbols
                .get(&symbol)
                .map(|symbol_data| SymbolName::Known(&symbol_data.name)),
        }
    }

    pub fn symbol_name(&self, symbol: Symbol) -> SymbolName<'_> {
        self.maybe_symbol_name(symbol).unwrap()
    }

    pub fn is_symbol_visible(&self, symbol: Symbol) -> bool {
        match symbol {
            Symbol::Error => true,
            Symbol::ErrorRepeat => false,
            Symbol::Symbol(symbol) => self.symbols[&symbol].is_visible,
        }
    }
}

pub struct SymbolData {
    pub name: SmolStr,
    pub is_visible: bool,
}

pub enum SymbolName<'a> {
    Error,
    ErrorRepeat,
    Known(&'a str),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    Error,
    ErrorRepeat,
    Symbol(SymbolToken),
}

pub type SymbolToken = u16;
