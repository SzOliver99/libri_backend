CREATE TABLE IF NOT EXISTS `user_cart` (
  `id` INT AUTO_INCREMENT PRIMARY KEY,
  `userId` INT NOT NULL,
  FOREIGN KEY (`userId`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `cart_items` (
  `id` INT AUTO_INCREMENT PRIMARY KEY,
  `cartId` INT NOT NULL,
  `bookId` INT NOT NULL,
  `quantity` INT NOT NULL DEFAULT 1,
  FOREIGN KEY (`cartId`) REFERENCES `user_cart` (`id`) ON DELETE CASCADE,
  FOREIGN KEY (`bookId`) REFERENCES `books` (`id`) ON DELETE CASCADE,
  UNIQUE KEY `cart_book_unique` (`cartId`, `bookId`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

