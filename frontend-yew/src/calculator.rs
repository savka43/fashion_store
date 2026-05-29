use serde::{Deserialize, Serialize};

use crate::models::PRODUCTS;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutfitCalcState {
    pub budget: i64,
    pub discount: i64,
    pub months: i64,
    pub chest_cm: i64,
    pub selected: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutfitCalcResult {
    pub subtotal: i64,
    pub total: i64,
    pub monthly: i64,
    pub balance: i64,
    pub size: &'static str,
}

impl Default for OutfitCalcState {
    fn default() -> Self {
        Self {
            budget: 50_000,
            discount: 10,
            months: 6,
            chest_cm: 96,
            selected: vec!["coat".to_string(), "hoodie".to_string()],
        }
    }
}

pub fn calculate(state: &OutfitCalcState) -> OutfitCalcResult {
    let subtotal = PRODUCTS
        .iter()
        .filter(|product| state.selected.iter().any(|id| id == product.id))
        .map(|product| product.price)
        .sum::<i64>();

    let discount = state.discount.clamp(0, 70);
    let months = state.months.clamp(1, 24);
    let total = subtotal - subtotal * discount / 100;
    let monthly = if total <= 0 { 0 } else { (total + months - 1) / months };

    OutfitCalcResult {
        subtotal,
        total,
        monthly,
        balance: state.budget - total,
        size: recommend_size(state.chest_cm),
    }
}

pub fn recommend_size(chest_cm: i64) -> &'static str {
    match chest_cm {
        value if value < 86 => "XS",
        value if value < 94 => "S",
        value if value < 102 => "M",
        value if value < 110 => "L",
        _ => "XL",
    }
}
