# Kozatsky Business (Solana + Anchor)

## Опис

Цей проєкт реалізує спрощену версію гри **"Козацький бізнес"** на блокчейні Solana з використанням Anchor Framework.

Смартконтракт дозволяє:
- створювати акаунт гравця (PDA)
- шукати ресурси з обмеженням у часі (cooldown 60 секунд)
- мінтити SPL токени через CPI


## Технології

- Rust
- Anchor Framework (v0.31.1)
- Solana CLI
- TypeScript (тести)
- @coral-xyz/anchor


## Архітектура

### Акаунт гравця (PDA)

Зберігає:
- owner (Pubkey)
- last_search_timestamp (час останнього пошуку)
- bump


## Інструкції

### 1. Initialize Player
Створює PDA-акаунт для гравця.

### 2. Search Resources
- можна викликати раз на 60 секунд
- оновлює timestamp
- (опціонально) мінтить ресурси

### 3. Mint Resource
- використовує CPI до Token Program
- мінтить токени на акаунт користувача


## Тестування

Запуск тестів:
anchor test