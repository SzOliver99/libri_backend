CREATE TABLE IF NOT EXISTS `books` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(50) NOT NULL,
  `author` varchar(50) NOT NULL,
  `price` int(5) NOT NULL,
  `description` varchar(255) NOT NULL,
  `imageSrc` varchar(255) NOT NULL DEFAULT '',
  `publishedDate` varchar(255) NOT NULL,
  `isbn` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
INSERT INTO `books` (`id`, `title`, `author`, `price`, `description`, `publishedDate`, `isbn`) VALUES
(1, 'Egri csillagok', 'Gárdonyi Géza', 1000, 'A historical novel set during the 16th century siege of Eger.', '1899-01-01', '9789634058766'),
(2, 'Az ember tragédiája', 'Madách Imre', 800, 'A philosophical drama exploring human history and fate.', '1861-01-01', '9789633754312'),
(3, 'Pál utcai fiúk', 'Molnár Ferenc', 2000, 'A novel about the adventures and struggles of a group of boys in Budapest.', '1906-01-01', '9789634058254'),
(4, 'Tüskevár', 'Fekete István', 1500, 'A coming-of-age story set in the Hungarian wilderness.', '1957-01-01', '9789631188374'),
(5, 'A kőszívű ember fiai', 'Jókai Mór', 2200, 'A novel depicting the Hungarian Revolution of 1848.', '1869-01-01', '9789634056199'),
(6, 'A láthatatlan ember', 'Gárdonyi Géza', 1200, 'Egy regény az ókori Rómában, egy rabszolgáról, aki láthatatlanná válik.', '1901-01-01', '9789631188497'),
(7, 'Aranyember', 'Jókai Mór', 1500, 'Egy romantikus kalandregény, amely a Duna mentén játszódik.', '1872-01-01', '9789633757417'),
(8, 'Légy jó mindhalálig', 'Móricz Zsigmond', 1000, 'Egy történet egy fiatal fiúról, aki erkölcsi dilemmákkal szembesül az iskolában.', '1920-01-01', '9789634056915'),
(9, 'Szindbád', 'Krúdy Gyula', 2000, 'Rövid novellák sorozata, amelyek Szindbád, az álomszerű utazó történetét mesélik el.', '1911-01-01', '9789634057868'),
(10, 'Az aranykoporsó', 'Jókai Mór', 1700, 'Történelmi regény a Római Birodalom bukásáról.', '1875-01-01', '9789631187469'),
(11, 'Abigél', 'Szabó Magda', 1800, 'Egy háborús dráma, amely egy leánynevelő intézetben játszódik.', '1970-01-01', '9789631187681'),
(12, 'Valahol Európában', 'Fábri Zoltán', 1100, 'Második világháború utáni történet árván maradt gyerekekről Magyarországon.', '1947-01-01', '9789634057899'),
(13, 'A Pendragon legenda', 'Szerb Antal', 1300, 'Egy gótikus rejtély ősi családról és természetfeletti eseményekről.', '1934-01-01', '9789634056129'),
(14, 'Iskola a határon', 'Ottlik Géza', 900, 'Egy regény a katonai akadémiai életről és barátságról.', '1959-01-01', '9789631187931'),
(15, 'Szerelem', 'Déry Tibor', 2000, 'Egy novella a szerelemről és a politikai bebörtönzésről.', '1956-01-01', '9789634057554'),
(16, 'A gyertyák csonkig égnek', 'Márai Sándor', 1400, 'Egy történet a barátságról, az árulásról és az idő múlásáról.', '1942-01-01', '9789634057103'),
(17, 'Anna Édes', 'Kosztolányi Dezső', 1000, 'Tragikus történet egy szolgálóról és munkaadóival való kapcsolatáról.', '1926-01-01', '9789631187177'),
(18, 'A kincskereső kisködmön', 'Móra Ferenc', 1600, 'Gyermekregény a szegénységről és a képzelet erejéről.', '1918-01-01', '9789633758375'),
(19, 'Emberszag', 'Németh László', 1900, 'Pszichológiai regény az életről diktatúra alatt.', '1935-01-01', '9789631187399'),
(20, 'Utazás a koponyám körül', 'Karinthy Frigyes', 1500, 'Félig önéletrajzi regény egy agyműtétről.', '1937-01-01', '9789631187474'),
(21, 'Az Isten rabjai', 'Gárdonyi Géza', 1700, 'Regény a hitről és odaadásról a középkori Magyarországon.', '1908-01-01', '9789633757933'),
(22, 'A sátán fattya', 'Sütő András', 1300, 'Második világháború utáni regény, amely Erdélyben játszódik.', '1949-01-01', '9789631187528'),
(23, 'Az utolsó kocsma', 'Békés Pál', 900, 'Különc, szürreális regény egy titokzatos fogadóról.', '1996-01-01', '9789634058320'),
(24, 'A fekete város', 'Mikszáth Kálmán', 1400, 'Történelmi regény a bosszúról és intrikáról a 18. században.', '1910-01-01', '9789634057590'),
(25, 'Ábel a rengetegben', 'Tamási Áron', 1800, 'Felnövéstörténet, amely Erdély hegyeiben játszódik.', '1932-01-01', '9789634057972'),
(26, 'A harag napja', 'Gábor Andor', 1600, 'Politikai dráma az 1848-as magyar forradalom idején.', '1930-01-01', '9789633757377'),
(27, 'Mihály', 'Mészöly Miklós', 1100, 'Lírai regény Mihály belső világáról.', '1963-01-01', '9789631187191'),
(28, 'Esőleső Tóbiás', 'Móra Ferenc', 500, 'Gyermekekről szóló történet a kalandokról és felfedezésekről.', '1930-01-01', '9789634058122'),
(29, 'Isten ostora', 'Móra Ferenc', 1500, 'Történelmi regény Attila királyról, a hunok vezetőjéről.', '1928-01-01', '9789634058313'),
(30, 'A kékszakállú herceg vára', 'Bartók Béla', 700, 'Libretto a rejtélyes Kékszakállúról és új feleségéről.', '1918-01-01', '9789634058283'),
(31, 'Az életet meg kell tanulni', 'Herczeg Ferenc', 1700, 'Regény a szerelemről, árulásról és a társadalmi változásokról.', '1925-01-01', '9789634057835'),
(32, 'A régi ház', 'Fekete István', 1000, 'Nosztalgikus regény a vidéki életről és a változásról.', '1941-01-01', '9789631187917'),
(33, 'Magyar Elektronikus Könyvtár', 'Különböző szerzők', 3000, 'Digitalizált magyar irodalmi művek gyűjteménye.', '2023-01-01', '9789631187214'),
(34, 'A pisztrángok titka', 'Mészöly Miklós', 1300, 'Rövid történetek gyűjteménye szürreális, álomszerű hangulattal.', '1966-01-01', '9789631187832'),
(35, 'A kockásinges nyomozó', 'Zsoldos Péter', 1600, 'Egy űrbéli nyomozóról szóló sci-fi regény.', '1978-01-01', '9789631187559'),
(36, 'Törött virág', 'Bródy Sándor', 1200, 'Tragikus szerelmi történet, amely osztálykülönbségekről szól a vidéki Magyarországon.', '1897-01-01', '9789631187177'),
(37, 'Az utolsó bűn', 'Benedek István', 1000, 'Filozófiai regény, amely a bűntudatot és a megváltást tárgyalja.', '1955-01-01', '9789634057867'),
(38, 'Téli berek', 'Fekete István', 900, 'A Tüskevár folytatása, egy téli kalandról.', '1959-01-01', '9789634057939'),
(39, 'Az aranytó', 'Sütő András', 1800, 'Családi dráma, amely a magyar vidéken játszódik.', '1968-01-01', '9789634057717'),
(40, 'Bors néni', 'Csukás István', 1300, 'Kalandos gyermekregény egy különleges hölgy történetéről.', '1969-01-01', '9789634057540'),
(41, 'Egri csillagok', 'Gárdonyi Géza', 2000, 'Történelmi regény az egri vár védőinek harcáról.', '1901-01-01', '9789631188496'),
(42, 'Mesterségem a halál', 'Merle Robert', 1700, 'Egy SS-tiszt naplószerű vallomása a náci korszakról.', '1952-01-01', '9789634057637'),
(43, 'Tóték', 'Örkény István', 800, 'Groteszk történet a második világháborúról egy kis magyar faluban.', '1967-01-01', '9789634057454'),
(44, 'A fehér folt', 'Gárdonyi Géza', 1500, 'Detektívregény, amely Budapesten játszódik.', '1909-01-01', '9789631187153'),
(45, 'Az ötödik pecsét', 'Sánta Ferenc', 3000, 'Háborús dráma, amely az emberi erkölcsöket vizsgálja.', '1963-01-01', '9789631187849'),
(46, 'Szent Péter esernyője', 'Mikszáth Kálmán', 900, 'Humoros regény egy csodás esernyőről.', '1895-01-01', '9789631187122'),
(47, 'Házasságtörés', 'Zilahy Lajos', 1000, 'Dráma a szerelemről, hűtlenségről és társadalmi elvárásokról.', '1936-01-01', '9789634057783'),
(48, 'A mi utcánk', 'Bohumil Hrabal', 1400, 'Rövid történetek gyűjteménye egy kis magyar faluról.', '1965-01-01', '9789634057219'),
(49, 'A kis herceg', 'Antoine de Saint-Exupéry', 600, 'A híres novella magyar fordítása egy kis hercegről.', '1943-01-01', '9789631187221'),
(50, 'Tükrök', 'Pilinszky János', 700, 'Elmélkedő, filozófiai versek gyűjteménye.', '1956-01-01', '9789631187894'),
(51, 'A fájdalom kapuja', 'Németh László', 1100, 'Egy regény a gyászról és a gyógyulásról.', '1953-01-01', '9789631187719'),
(52, 'Száz év magány', 'Gabriel García Márquez', 1900, 'Az \"Egy száz év magány\" magyar fordítása.', '1967-01-01', '9789631187465');
