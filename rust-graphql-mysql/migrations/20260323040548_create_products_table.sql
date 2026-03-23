CREATE TABLE `products` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `category_id` bigint NOT NULL,  
  `alertstocks` int DEFAULT '0',
  `category` varchar(255) DEFAULT NULL,
  `costprice` decimal(10,0) DEFAULT '0',
  `created_at` timestamp DEFAULT NOW(),
  `criticalstocks` int DEFAULT '0',
  `descriptions` varchar(255) NOT NULL,
  `productpicture` varchar(255) DEFAULT NULL,
  `qty` int DEFAULT '0',
  `saleprice` decimal(10,0) DEFAULT '0',
  `sellprice` decimal(10,0) DEFAULT '0',
  `unit` varchar(255) DEFAULT NULL,
  `updated_at` timestamp DEFAULT NOW(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `index_products_on_descriptions` (`descriptions`),
  CONSTRAINT `fk_products_category` 
    FOREIGN KEY (`category_id`) 
    REFERENCES `categories` (`id`) 
    ON DELETE CASCADE
) ENGINE=InnoDB;  
