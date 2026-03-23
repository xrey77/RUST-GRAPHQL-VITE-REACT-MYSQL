-- Add migration script here
CREATE TABLE `users` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `firstname` varchar(255) DEFAULT NULL,
  `lastname` varchar(255) DEFAULT NULL,
  `email` varchar(255) NOT NULL,
  `mobile` varchar(255) DEFAULT NULL,
  `username` varchar(255) NOT NULL,
  `password_digest` varchar(255) DEFAULT NULL,
  `role_id` bigint DEFAULT NULL,
  `userpic` varchar(255) NOT NULL DEFAULT 'pix.png',
  `isactivated` int DEFAULT 1,
  `isblocked` int DEFAULT 0,
  `mailtoken` int DEFAULT 0,
  `qrcodeurl` text,
  `secret` text,
  `created_at` timestamp DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `index_users_on_email` (`email`),
  UNIQUE KEY `index_users_on_username` (`username`),
  CONSTRAINT `fk_users_role` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) 
    ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=19 DEFAULT CHARSET=utf8mb4;
