CREATE TABLE IF NOT EXISTS `purchase_history` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `userId` int(11) NOT NULL,
  `status` varchar(50) NOT NULL DEFAULT 'Processing',
  `price` int(5) NOT NULL,
  `purchaseDate` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  FOREIGN KEY (`userId`) REFERENCES `users`(`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;