CREATE TABLE `sales` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `salesamount` decimal(10,0) DEFAULT '0',
  `salesdate` timestamp DEFAULT NOW(),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=35 DEFAULT CHARSET=utf8mb3;