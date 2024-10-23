CREATE TABLE IF NOT EXISTS `reset_tokens` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `userId` int(11) NOT NULL,
    `token` varchar(255) NOT NULL,
    `tokenExpires` datetime NOT NULL,
    PRIMARY KEY (`id`),
    FOREIGN KEY (`userId`) REFERENCES `users`(`id`) ON DELETE CASCADE
);
