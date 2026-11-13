// Copyright (c) 2026 Greg, l1acy

use serenity::secrets::Token;

// токен вшивается в бинарник на этапе компиляции через build.rs
const TOKEN_STR: &str = env!("BRAIN_CRAB_TOKEN");

pub fn token() -> Token {
    TOKEN_STR.parse().expect("BRAIN_CRAB_TOKEN содержит некорректный токен")
}
