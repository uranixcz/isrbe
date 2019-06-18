
/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
DROP TABLE IF EXISTS `location`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `location` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `lat` decimal(10,7) NOT NULL,
  `lon` decimal(10,7) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `location` WRITE;
/*!40000 ALTER TABLE `location` DISABLE KEYS */;
INSERT INTO `location` VALUES (1,0.0000000,0.0000000),(2,0.0017000,0.0008000);
/*!40000 ALTER TABLE `location` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `param`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `param` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8_unicode_ci NOT NULL,
  `type` tinyint(3) unsigned NOT NULL,
  `qty_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `param` WRITE;
/*!40000 ALTER TABLE `param` DISABLE KEYS */;
INSERT INTO `param` VALUES (1,'amount',0,0),(2,'weight',0,2);
/*!40000 ALTER TABLE `param` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `param_float`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `param_float` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `res_qty_id` bigint(20) unsigned NOT NULL,
  `val` double NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `param_float` WRITE;
/*!40000 ALTER TABLE `param_float` DISABLE KEYS */;
INSERT INTO `param_float` VALUES (1,1,1);
/*!40000 ALTER TABLE `param_float` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `param_res`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `param_res` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `res_qty_id` bigint(20) unsigned NOT NULL,
  `val` bigint(20) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `param_res` WRITE;
/*!40000 ALTER TABLE `param_res` DISABLE KEYS */;
/*!40000 ALTER TABLE `param_res` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `param_text`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `param_text` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `res_qty_id` bigint(20) unsigned NOT NULL,
  `val` varchar(255) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `param_text` WRITE;
/*!40000 ALTER TABLE `param_text` DISABLE KEYS */;
/*!40000 ALTER TABLE `param_text` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `quantity`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `quantity` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8_unicode_ci NOT NULL,
  `unit` varchar(255) COLLATE utf8_unicode_ci NOT NULL,
  `def` varchar(255) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=30 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `quantity` WRITE;
/*!40000 ALTER TABLE `quantity` DISABLE KEYS */;
INSERT INTO `quantity` VALUES (1,'Délka','m','m'),(2,'Hmotnost','g','g'),(3,'Čas','s','s'),(4,'Elektrický proud','A','A'),(5,'Termodynamická teplota','K','K'),(6,'Látkové množství','mol','mol'),(7,'Svítivost','cd','cd'),(8,'absorbovaná dávka (ionizujícího záření) ','Gy ','J/kg '),(9,'Celsiova teplota ','°C ','K (t/°C = T/K − 273,15) '),(10,'dávkový ekvivalent (ionizujícího záření) ','Sv ','J/kg '),(11,'elektrická kapacita ','F ','C/V '),(12,'elektrická vodivost ','S ','1/Ω '),(13,'elektrické napětí, elektrický potenciál ','V ','W/A = J/C '),(14,'elektrický náboj ','C ','s·A '),(15,'elektrický odpor, impedance, reaktance ','Ω ','V/A '),(16,'energie, práce, teplo ','J ','N·m = C·V = W·s '),(17,'frekvence ','Hz ','1/s '),(18,'indukčnost ','H ','V·s/A = Wb/A '),(19,'intenzita osvětlení ','lx ','lm/m2 '),(20,'katalytická aktivita ','kat ','mol/s '),(21,'magnetická indukce ','T ','V·s/m2 = Wb/m2 = N/(A·m) '),(22,'magnetický tok ','Wb ','J/A '),(23,'prostorový úhel ','sr ','m2·m−2 '),(24,'radioaktivita (počet rozpadů částic za sekundu) ','Bq ','1/s '),(25,'síla, váha ','N ','kg·m/s2 '),(26,'světelný tok ','lm ','lx·m2 '),(27,'tlak, napětí ','Pa ','N/m2 '),(28,'úhel ','rad ','m·m−1 '),(29,'výkon, zářivý tok ','W ','J/s = V·A ');
/*!40000 ALTER TABLE `quantity` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `quantity_10exp`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `quantity_10exp` (
  `qty_10exp_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `qty_10exp_smbl` varchar(50) COLLATE utf8_unicode_ci NOT NULL,
  `qty_10exp_f` tinyint(4) NOT NULL DEFAULT '0',
  PRIMARY KEY (`qty_10exp_id`)
) ENGINE=InnoDB AUTO_INCREMENT=23 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `quantity_10exp` WRITE;
/*!40000 ALTER TABLE `quantity_10exp` DISABLE KEYS */;
INSERT INTO `quantity_10exp` VALUES (1,'Y',24),(2,'Z',21),(3,'E',18),(4,'P',15),(5,'T',12),(6,'G',9),(7,'M',6),(8,'k',3),(9,'h',2),(10,'da',1),(11,'',0),(12,'d',-1),(13,'c',-2),(14,'m',-3),(15,'µ',-6),(16,'n',-9),(17,'p',-12),(18,'f',-15),(19,'a',-18),(20,'z',-21),(21,'y',-24);
/*!40000 ALTER TABLE `quantity_10exp` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `resource`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `resource` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
  `type_id` bigint(20) unsigned NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `resource` WRITE;
/*!40000 ALTER TABLE `resource` DISABLE KEYS */;
INSERT INTO `resource` VALUES (1,'brambor',4),(2,'hranol dřevěný',4),(3,'hřebík ocelový',4),(4,'water',1),(5,'acqacq',1),(6,'acqacq3',2),(8,'Flint',1),(9,'flour',1),(10,'pizza',4);
/*!40000 ALTER TABLE `resource` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `resource_location`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `resource_location` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `res_param_id` bigint(20) unsigned NOT NULL COMMENT 'prejmenovano z res_qty_id',
  `loc_id` bigint(20) unsigned NOT NULL,
  `loc_radius` decimal(10,0) unsigned NOT NULL,
  `loc_val` double NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `resource_location` WRITE;
/*!40000 ALTER TABLE `resource_location` DISABLE KEYS */;
INSERT INTO `resource_location` VALUES (1,1,1,0,555);
/*!40000 ALTER TABLE `resource_location` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `resource_param`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `resource_param` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT COMMENT 'upraveno MM',
  `res_id` bigint(20) unsigned NOT NULL,
  `param_id` bigint(20) unsigned NOT NULL,
  `qty_10exp_id` bigint(20) unsigned NOT NULL DEFAULT '11',
  `is_movable` tinyint(1) NOT NULL COMMENT 'upraveno MM',
  PRIMARY KEY (`id`),
  UNIQUE KEY `res_id` (`res_id`,`param_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci COMMENT='prejemenovano z resource_quantity';
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `resource_param` WRITE;
/*!40000 ALTER TABLE `resource_param` DISABLE KEYS */;
INSERT INTO `resource_param` VALUES (1,1,2,11,1);
/*!40000 ALTER TABLE `resource_param` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `resource_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `resource_type` (
  `res_type_id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `res_type_name` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
  `res_type_def` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`res_type_id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `resource_type` WRITE;
/*!40000 ALTER TABLE `resource_type` DISABLE KEYS */;
INSERT INTO `resource_type` VALUES (1,'Natural',''),(2,'Transport',''),(3,'Energy',''),(4,'Production','');
/*!40000 ALTER TABLE `resource_type` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `transform_hdr`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `transform_hdr` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `type_id` bigint(20) unsigned NOT NULL,
  `ref` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `transform_hdr` WRITE;
/*!40000 ALTER TABLE `transform_hdr` DISABLE KEYS */;
/*!40000 ALTER TABLE `transform_hdr` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `transform_line`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `transform_line` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `transform_hdr_id` bigint(20) unsigned NOT NULL,
  `res_loc_id` bigint(20) unsigned NOT NULL,
  `val` double NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `transform_line` WRITE;
/*!40000 ALTER TABLE `transform_line` DISABLE KEYS */;
/*!40000 ALTER TABLE `transform_line` ENABLE KEYS */;
UNLOCK TABLES;
DROP TABLE IF EXISTS `transform_type`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `transform_type` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(100) COLLATE utf8_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

LOCK TABLES `transform_type` WRITE;
/*!40000 ALTER TABLE `transform_type` DISABLE KEYS */;
INSERT INTO `transform_type` VALUES (1,'User order'),(2,'Natural'),(3,'Manufacturing'),(4,'Degradation');
/*!40000 ALTER TABLE `transform_type` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

