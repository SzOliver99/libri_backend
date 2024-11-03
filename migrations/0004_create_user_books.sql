CREATE TABLE IF NOT EXISTS `purchase_history` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `userId` int(11) NOT NULL,
  `status` varchar(50) NOT NULL DEFAULT 'Processing',
  `price` int(5) NOT NULL,
  `purchaseDate` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `userId` (`userId`),
  CONSTRAINT `user_books_ibfk_1` FOREIGN KEY (`userId`) REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `purchased_books` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `bookId` int(11) NOT NULL,
  `price` int(5) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `bookId` (`bookId`),
  CONSTRAINT `purchase_history_ibfk_1` FOREIGN KEY (`bookId`) REFERENCES `books` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
