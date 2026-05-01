//! Symbol interning for efficient atom representation
//!
//! This module provides a symbol table that ensures each unique atom
//! is stored only once, allowing atoms to be represented as lightweight IDs
//! rather than full strings.

use std::collections::HashMap;
use std::sync::RwLock;

/// Unique identifier for an interned symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

/// Interned symbol - a lightweight reference to a string
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub id: SymbolId,
}

impl Symbol {
    pub const fn new(id: usize) -> Self {
        Self { id: SymbolId(id) }
    }
}

/// Symbol table for interning atoms
pub struct SymbolTable {
    symbols: RwLock<HashMap<String, SymbolId>>,
    next_id: RwLock<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: RwLock::new(HashMap::new()),
            next_id: RwLock::new(0),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            symbols: RwLock::new(HashMap::with_capacity(capacity)),
            next_id: RwLock::new(0),
        }
    }

    pub fn intern(&self, s: &str) -> Symbol {
        {
            let symbols = self.symbols.read().unwrap();
            if let Some(id) = symbols.get(s) {
                return Symbol::new(id.0);
            }
        }

        let mut symbols = self.symbols.write().unwrap();
        let id = *self.next_id.read().unwrap();
        *self.next_id.write().unwrap() = id + 1;
        
        symbols.insert(s.to_string(), SymbolId(id));
        Symbol::new(id)
    }

    pub fn len(&self) -> usize {
        self.symbols.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.read().unwrap().is_empty()
    }

    pub fn clear(&self) {
        self.symbols.write().unwrap().clear();
        *self.next_id.write().unwrap() = 0;
    }

    pub fn stats(&self) -> SymbolTableStats {
        let symbols = self.symbols.read().unwrap();
        let next_id = self.next_id.read().unwrap();
        SymbolTableStats {
            unique_symbols: symbols.len(),
            max_id: *next_id,
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTableStats {
    pub unique_symbols: usize,
    pub max_id: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern() {
        let table = SymbolTable::new();
        let sym1 = table.intern("foo");
        let sym2 = table.intern("bar");
        let sym3 = table.intern("foo");
        assert_eq!(sym1, sym3);
        assert_ne!(sym1, sym2);
    }

    #[test]
    fn test_len() {
        let table = SymbolTable::new();
        assert_eq!(table.len(), 0);
        table.intern("foo");
        assert_eq!(table.len(), 1);
    }
}
