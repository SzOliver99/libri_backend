CREATE TABLE IF NOT EXISTS `books` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `title` varchar(50) NOT NULL,
  `author` varchar(50) NOT NULL,
  `price` int(5) NOT NULL,
  `description` varchar(1000) NOT NULL,
  `image_src` varchar(255) NOT NULL DEFAULT '',
  `published_date` varchar(255) NOT NULL,
  `isbn` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

INSERT INTO `books` (`id`, `title`, `author`, `price`, `description`, `image_src`, `published_date`, `isbn`) VALUES
(1, 'Egri csillagok', 'Gárdonyi Géza', 1000, 'Az Egri csillagok alcíme: Bornemissza Gergely élete. Az ő életét, sorsának alakulását követhetjük végig a műben: a gyermekkortól megannyi kalandon, a szerelem beteljesülésén át a várvédő katona tetteiig. A regény két részre tagolódik: az ostrom előtti évek és a néhány hetes ostrom. Rengeteg szereplőt, történelmi alakot és írói képzelet által megformált figurát mutat be az író. S bár a középpontban Bornemissza Gergely áll, az Egri csillagok elsősorban nem neki, hanem a török ellen küzdő magyarságnak állít emléket. Az önzetlen hazaszeretet és hazafiság példájával. Hiszen az egri vár védői szinte reménytelen helyzetben vállalták a küzdelmet, tudván azt, hogy csupán önmagukra támaszkodhatnak. A félelmetes török túlerővel szemben csodának tetszik a győzelem. De a történelem valósága volt ez a csoda, melyet valóságos férfiak és nők - vagyis hősként viselkedő emberek értek el.', 'https://cdn.antikvarium.hu/foto/eredeti/44813976.jpg', '1899-01-01', '9789634058766'),
(2, 'Az ember tragédiája', 'Madách Imre', 800, 'A philosophical drama exploring human history and fate.', 'https://s01.static.libri.hu/cover/25/4/2231071_4.jpg', '1861-01-01', '9789633754312'),
(3, 'Pál utcai fiúk', 'Molnár Ferenc', 2000, 'A novel about the adventures and struggles of a group of boys in Budapest.', 'https://s01.static.libri.hu/cover/ba/e/9339701_4.jpg', '1906-01-01', '9789634058254'),
(4, 'Tüskevár', 'Fekete István', 1500, 'A coming-of-age story set in the Hungarian wilderness.', 'https://media.regikonyvek.hu/media/1034598/conversions/fekete-istvan-tuskevar-4_137382c8-jpg.jpg', '1957-01-01', '9789631188374'),
(5, 'A kőszívű ember fiai', 'Jókai Mór', 2200, 'A novel depicting the Hungarian Revolution of 1848.', 'https://lira.erbacdn.net/upload/M_28/rek1/324/2784324.jpg', '1869-01-01', '9789634056199'),
(6, 'A láthatatlan ember', 'Gárdonyi Géza', 1200, 'Egy regény az ókori Rómában, egy rabszolgáról, aki láthatatlanná válik.', 'https://lira.erbacdn.net/upload/M_28/rek1/261/3989261.jpg', '1901-01-01', '9789631188497'),
(7, 'Aranyember', 'Jókai Mór', 1500, 'Egy romantikus kalandregény, amely a Duna mentén játszódik.', 'https://m.media-amazon.com/images/M/MV5BNDUwNzY4NzAtOTVlYi00MDJlLTg0NTEtNzUwNGZhZDFkNTFjXkEyXkFqcGc@._V1_.jpg', '1872-01-01', '9789633757417'),
(8, 'Légy jó mindhalálig', 'Móricz Zsigmond', 1000, 'Egy történet egy fiatal fiúról, aki erkölcsi dilemmákkal szembesül az iskolában.', 'https://mora.hu/content/2018/7/Product/300/Legy-jo-mindhalalig.jpg', '1920-01-01', '9789634056915'),
(9, 'Szindbád', 'Krúdy Gyula', 2000, 'Rövid novellák sorozata, amelyek Szindbád, az álomszerű utazó történetét mesélik el.', 'https://s01.static.libri.hu/cover/5a/b/7976935_4.jpg', '1911-01-01', '9789634057868'),
(10, 'Az aranykoporsó', 'Jókai Mór', 1700, 'Történelmi regény a Római Birodalom bukásáról.', 'https://lira.erbacdn.net/upload/M_28/rek1/285/770285.jpg', '1875-01-01', '9789631187469'),
(11, 'Abigél', 'Szabó Magda', 1800, 'Egy háborús dráma, amely egy leánynevelő intézetben játszódik.', 'https://s01.static.libri.hu/cover/1e/9/615914_4.jpg', '1970-01-01', '9789631187681'),
(12, 'Európa elrablása', 'Fábri Zoltán', 1100, '', 'https://marvin.bline.hu/product_images/348/0669000385465P.JPG', '1947-01-01', '9789634057899'),
(13, 'A Pendragon legenda', 'Szerb Antal', 1300, 'Egy gótikus rejtély ősi családról és természetfeletti eseményekről.', 'https://upload.wikimedia.org/wikipedia/commons/d/dc/Pendragon.jpg', '1934-01-01', '9789634056129'),
(14, 'Iskola a határon', 'Ottlik Géza', 900, 'Egy regény a katonai akadémiai életről és barátságról.', 'https://marvin.bline.hu/product_images/518/0669000356427P.JPG', '1959-01-01', '9789631187931'),
(15, 'Szerelem', 'Déry Tibor', 2000, 'Egy novella a szerelemről és a politikai bebörtönzésről.', 'https://dibook.hu/storage/books/pr_2115/cover-big.webp', '1956-01-01', '9789634057554'),
(16, 'A gyertyák csonkig égnek', 'Márai Sándor', 1400, 'Egy történet a barátságról, az árulásról és az idő múlásáról.', 'https://s01.static.libri.hu/cover/b5/0/615849_4.jpg', '1942-01-01', '9789634057103'),
(17, 'Édes Anna', 'Kosztolányi Dezső', 1000, 'Tragikus történet egy szolgálóról és munkaadóival való kapcsolatáról.', 'https://www.atlantiszkiado.hu/file/2f8a39857312_4343-1.jpg', '1926-01-01', '9789631187177'),
(18, 'A kincskereső kisködmön', 'Móra Ferenc', 1600, 'Gyermekregény a szegénységről és a képzelet erejéről.', 'https://download.voiz.hu/9ee280250c1ad24b5162e7392ca3cc76/kincskeresokiskodmon.jpg', '1918-01-01', '9789633758375'),
(19, 'Emberszag', 'Szép Ernő', 1900, 'Pszichológiai regény az életről diktatúra alatt.', 'https://lira.erbacdn.net/upload/M_28/rek1/357/1495357.jpg', '1935-01-01', '9789631187399'),
(20, 'Utazás a koponyám körül', 'Karinthy Frigyes', 1500, 'Félig önéletrajzi regény egy agyműtétről.', 'https://upload.wikimedia.org/wikipedia/commons/3/36/Utaz%C3%A1sels%C5%91.jpg', '1937-01-01', '9789631187474'),
(21, 'Az Isten rabjai', 'Gárdonyi Géza', 1700, 'Regény a hitről és odaadásról a középkori Magyarországon.', 'https://s01.static.libri.hu/cover/87/c/8879672_4.jpg', '1908-01-01', '9789633757933'),
(22, 'A sátán fattya', 'Nagy Zoltán Mihály', 1300, 'Második világháború utáni regény, amely Erdélyben játszódik.', 'https://s01.static.libri.hu/cover/c2/5/860125_4.jpg', '1949-01-01', '9789631187528'),
(23, 'Az utolsó kocsma', 'Kertész Imre', 900, 'Különc, szürreális regény egy titokzatos fogadóról.', 'https://moly.hu/system/covers/big/covers_315868.jpg', '1996-01-01', '9789634058320'),
(24, 'A fekete város', 'Mikszáth Kálmán', 1400, 'Történelmi regény a bosszúról és intrikáról a 18. században.', 'https://s01.static.libri.hu/cover/80/2/615765_4.jpg', '1910-01-01', '9789634057590'),
(25, 'Ábel a rengetegben', 'Tamási Áron', 1800, 'Felnövéstörténet, amely Erdély hegyeiben játszódik.', 'https://marvin.bline.hu/product_images/208/B1226152.JPG', '1932-01-01', '9789634057972'),
(26, 'A harag napja', 'Gábor Andor', 1600, 'Politikai dráma az 1848-as magyar forradalom idején.', '', '1930-01-01', '9789633757377'),
(27, 'Mihály', 'Mészöly Miklós', 1100, 'Lírai regény Mihály belső világáról.', '', '1963-01-01', '9789631187191'),
(28, 'Esőleső Tóbiás', 'Móra Ferenc', 500, 'Gyermekekről szóló történet a kalandokról és felfedezésekről.', '', '1930-01-01', '9789634058122'),
(29, 'Isten ostora', 'Móra Ferenc', 1500, 'Történelmi regény Attila királyról, a hunok vezetőjéről.', '', '1928-01-01', '9789634058313'),
(30, 'A kékszakállú herceg vára', 'Bartók Béla', 700, 'Libretto a rejtélyes Kékszakállúról és új feleségéről.', '', '1918-01-01', '9789634058283'),
(31, 'Az életet meg kell tanulni', 'Herczeg Ferenc', 1700, 'Regény a szerelemről, árulásról és a társadalmi változásokról.', '', '1925-01-01', '9789634057835'),
(32, 'A régi ház', 'Fekete István', 1000, 'Nosztalgikus regény a vidéki életről és a változásról.', '', '1941-01-01', '9789631187917'),
(33, 'Magyar Elektronikus Könyvtár', 'Különböző szerzők', 3000, 'Digitalizált magyar irodalmi művek gyűjteménye.', '', '2023-01-01', '9789631187214'),
(34, 'A pisztrángok titka', 'Mészöly Miklós', 1300, 'Rövid történetek gyűjteménye szürreális, álomszerű hangulattal.', '', '1966-01-01', '9789631187832'),
(35, 'A kockásinges nyomozó', 'Zsoldos Péter', 1600, 'Egy űrbéli nyomozóról szóló sci-fi regény.', '', '1978-01-01', '9789631187559'),
(36, 'Törött virág', 'Bródy Sándor', 1200, 'Tragikus szerelmi történet, amely osztálykülönbségekről szól a vidéki Magyarországon.', '', '1897-01-01', '9789631187177'),
(37, 'Az utolsó bűn', 'Benedek István', 1000, 'Filozófiai regény, amely a bűntudatot és a megváltást tárgyalja.', '', '1955-01-01', '9789634057867'),
(38, 'Téli berek', 'Fekete István', 900, 'A Tüskevár folytatása, egy téli kalandról.', '', '1959-01-01', '9789634057939'),
(39, 'Az aranytó', 'Sütő András', 1800, 'Családi dráma, amely a magyar vidéken játszódik.', '', '1968-01-01', '9789634057717'),
(40, 'Bors néni', 'Csukás István', 1300, 'Kalandos gyermekregény egy különleges hölgy történetéről.', '', '1969-01-01', '9789634057540'),
(41, 'Mesterségem a halál', 'Merle Robert', 1700, 'Egy SS-tiszt naplószerű vallomása a náci korszakról.', '', '1952-01-01', '9789634057637'),
(42, 'Tóték', 'Örkény István', 800, 'Groteszk történet a második világháborúról egy kis magyar faluban.', '', '1967-01-01', '9789634057454'),
(43, 'A fehér folt', 'Gárdonyi Géza', 1500, 'Detektívregény, amely Budapesten játszódik.', '', '1909-01-01', '9789631187153'),
(44, 'Az ötödik pecsét', 'Sánta Ferenc', 3000, 'Háborús dráma, amely az emberi erkölcsöket vizsgálja.', '', '1963-01-01', '9789631187849'),
(45, 'Szent Péter esernyője', 'Mikszáth Kálmán', 900, 'Humoros regény egy csodás esernyőről.', '', '1895-01-01', '9789631187122'),
(46, 'Házasságtörés', 'Zilahy Lajos', 1000, 'Dráma a szerelemről, hűtlenségről és társadalmi elvárásokról.', '', '1936-01-01', '9789634057783'),
(47, 'A mi utcánk', 'Bohumil Hrabal', 1400, 'Rövid történetek gyűjteménye egy kis magyar faluról.', '', '1965-01-01', '9789634057219'),
(48, 'A kis herceg', 'Antoine de Saint-Exupéry', 600, 'A híres novella magyar fordítása egy kis hercegről.', '', '1943-01-01', '9789631187221'),
(49, 'Tükrök', 'Pilinszky János', 700, 'Elmélkedő, filozófiai versek gyűjteménye.', '', '1956-01-01', '9789631187894'),
(50, 'A fájdalom kapuja', 'Németh László', 1100, 'Egy regény a gyászról és a gyógyulásról.', '', '1953-01-01', '9789631187719');
