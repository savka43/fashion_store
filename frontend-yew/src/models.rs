use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq)]
pub struct Product {
    pub id: &'static str,
    pub title: &'static str,
    pub search: &'static str,
    pub tag: &'static str,
    pub price: i64,
    pub image: &'static str,
    pub alt: &'static str,
    pub text: &'static str,
}

pub const PRODUCTS: &[Product] = &[
    Product {
        id: "coat",
        title: "Oversize Coat",
        search: "oversize пальто black coat outerwear",
        tag: "Outerwear",
        price: 24_900,
        image: "https://images.unsplash.com/photo-1515886657613-9f3515b0c78f?auto=format&fit=crop&w=900&q=80",
        alt: "Чёрное пальто",
        text: "Длинное пальто с прямым силуэтом и минимальной фурнитурой.",
    },
    Product {
        id: "hoodie",
        title: "Heavy Hoodie",
        search: "худи hoodie streetwear",
        tag: "Streetwear",
        price: 9_800,
        image: "https://images.unsplash.com/photo-1556821840-3a63f95609a7?auto=format&fit=crop&w=900&q=80",
        alt: "Худи",
        text: "Плотное худи свободного кроя для многослойных образов.",
    },
    Product {
        id: "trousers",
        title: "Wide Trousers",
        search: "брюки trousers wide pants tailoring",
        tag: "Tailoring",
        price: 12_400,
        image: "https://images.unsplash.com/photo-1541099649105-f69ad21f3246?auto=format&fit=crop&w=900&q=80",
        alt: "Брюки",
        text: "Широкие брюки со стрелками, подходящие для офиса и подиума.",
    },
    Product {
        id: "jacket",
        title: "Sharp Jacket",
        search: "куртка jacket leather",
        tag: "Jackets",
        price: 19_600,
        image: "https://images.unsplash.com/photo-1520975954732-35dd22299614?auto=format&fit=crop&w=900&q=80",
        alt: "Куртка",
        text: "Акцентная куртка с жёсткой формой и городским настроением.",
    },
    Product {
        id: "bag",
        title: "Metal Bag",
        search: "сумка bag аксессуары accessories",
        tag: "Accessories",
        price: 7_900,
        image: "https://images.unsplash.com/photo-1590874103328-eac38a683ce7?auto=format&fit=crop&w=900&q=80",
        alt: "Сумка",
        text: "Компактная сумка с металлической деталью и строгой геометрией.",
    },
    Product {
        id: "boots",
        title: "Urban Boots",
        search: "ботинки boots shoes обувь",
        tag: "Shoes",
        price: 14_300,
        image: "https://images.unsplash.com/photo-1542291026-7eec264c27ff?auto=format&fit=crop&w=900&q=80",
        alt: "Обувь",
        text: "Массивная обувь для контрастного завершения образа.",
    },
];

pub fn format_price(value: i64) -> String {
    let mut digits = value.abs().to_string();
    let mut chunks = Vec::new();
    while digits.len() > 3 {
        let tail = digits.split_off(digits.len() - 3);
        chunks.push(tail);
    }
    chunks.push(digits);
    chunks.reverse();
    let sign = if value < 0 { "-" } else { "" };
    format!("{sign}{} ₽", chunks.join(" "))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub email: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutfitPayload {
    pub title: String,
    pub content: String,
    pub occasion: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outfit {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub occasion: String,
    pub is_published: bool,
    pub created_at: String,
    pub author_email: String,
}
