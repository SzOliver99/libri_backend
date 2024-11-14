CREATE TABLE IF NOT EXISTS `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(50) NOT NULL,
  `email` varchar(50) NOT NULL, -- Need to hash it also and display like oli**@****.com
  `password` varchar(255) NOT NULL,
  `group` varchar(50) NOT NULL DEFAULT 'User',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

SET SESSION sql_require_primary_key = 0;
CREATE TABLE IF NOT EXISTS `user_info` (
  `user_id` int(11) NOT NULL,
  `first_name` varchar(50) NOT NULL DEFAULT '',
  `last_name` varchar(50) NOT NULL DEFAULT '',
  `phone_number` varchar(20) NOT NULL DEFAULT '',
  `billing_address` varchar(100) NOT NULL DEFAULT '',
  `city` varchar(50) NOT NULL DEFAULT '',
  `state_province` varchar(50) NOT NULL DEFAULT '',
  `postal_code` varchar(10) NOT NULL DEFAULT '',
  FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- Admin User
INSERT INTO `users`(`id`, `username`, `email`, `password`, `group`) VALUES(1, 'szoliver', 'oliver.szvetnyik@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$dzZf99FSn3oMySiKtHKuqA$E2W5bM4Crij9zg4vSuvGqPJ6so5GiVK67SjwcNiXxVA', 'Admin');
INSERT INTO `user_info`(`user_id`, `first_name`, `last_name`, `phone_number`, `billing_address`, `city`, `state_province`, `postal_code`) VALUES(1, 'Olivér', 'Szvetnyik', '+36 30 526 5492', 'Petőfi Sándor u. 2', 'Kiskunhalas', '', '6400')