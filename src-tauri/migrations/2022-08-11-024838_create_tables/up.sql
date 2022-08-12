-- Your SQL goes here

-- Your SQL goes here

CREATE TABLE IF NOT EXISTS `items` (
  `id` INTEGER PRIMARY KEY,
  `barcode` TEXT NOT NULL UNIQUE,
  `name` TEXT NOT NULL,
  `stock` INTEGER NOT NULL,
  `sold` INTEGER NOT NULL DEFAULT '0',
  `sell_price` NUMERIC NOT NULL,
  `buy_price` NUMERIC NULL,
  `profit` NUMERIC NOT NULL DEFAULT '0',
  `created_at` BIGINT NOT NULL,
  `updated_at` BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS `users` (
  `id` INTEGER PRIMARY KEY,
  `username` TEXT NOT NULL,
  `password` TEXT NOT NULL,
  `created_at` BIGINT NOT NULL,
  `updated_at` BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS `borrowers` (
  `id` INTEGER PRIMARY KEY,
  `name` TEXT NOT NULL,
  `created_at` BIGINT NOT NULL,
  `updated_at` BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS `borrowed_payments` (
  `id` INTEGER PRIMARY KEY,
  `amount` NUMERIC NOT NULL,
  `borrower_id` INTEGER NULL,
  `created_at` BIGINT NOT NULL,
  `updated_at` BIGINT NOT NULL,
  FOREIGN KEY (`borrower_id`) REFERENCES `borrowers` (`id`)
);

CREATE TABLE IF NOT EXISTS `receipts` (
  `id` INTEGER PRIMARY KEY,
  `readable_id` TEXT NOT NULL,
  `user_id` INTEGER NOT NULL,
  `borrower_id` INTEGER NULL,
  `items` TEXT NOT NULL,
  `created_at` BIGINT NOT NULL,
  `updated_at` BIGINT NOT NULL,
  FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
  FOREIGN KEY (`borrower_id`) REFERENCES `borrowers` (`id`)
);

CREATE TABLE IF NOT EXISTS `financial_statements` (
  `id` INTEGER PRIMARY KEY,
  `value` NUMERIC NOT NULL,
  `user_id` INTEGER NOT NULL,
  `description` TEXT NOT NULL,
  `metadata` TEXT NULL,
  `created_at` BIGINT NOT NULL,
  `updated_at` BIGINT NOT NULL,
  FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);