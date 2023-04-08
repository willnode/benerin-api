CREATE TABLE `kbbi` (
  `word` varchar(255) NOT NULL,
  `bword` varchar(255) NOT NULL,
  `kind` varchar(255) NOT NULL,
  `notes` varchar(255) NOT NULL,
  PRIMARY KEY (`word`, `kind`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
