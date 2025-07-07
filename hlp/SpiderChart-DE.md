## Spinnennetzdiagramm

Kleines Werkzeug zur Erstellung eines Netz-Zusammenfassungsdiagramms (Spinnennetzdiagramm) gemäß ASTM E2597/E2597M-22:

![image-20250703163914941](assets/image-20250703163914941.png)

### Benutzeroberfläche

Einfach die Messwerte von sechs Parametern eingeben – diese werden in Qualitätskennzahlen umgewandelt und als Spinnennetzdiagramm dargestellt (nicht vergessen, den IQI-Typ auszuwählen):

![image-20250704122743585](assets/image-20250704122743585.png)

### Diagramme speichern und laden

Sie können Ihre Daten in TOML-Dateien speichern (**TOML** steht für **„Tom's Obvious, Minimal Language.“**) TOML wird häufig in Projekten verwendet, die in Sprachen wie Rust geschrieben sind. Es ist ähnlich wie JSON, YAML oder INI-Dateien und leicht lesbar:

```toml
iqi = 0
detector = "Detector XYZ, Mode zyx"
isrb = 240.0
csa = 0.05
lag = 0.3
snrn = 820.0
smtr = 125.0
mtl = 90.0
```

Sie können diese Dateien über das Dateimenü oder mit diesen beiden Schaltflächen speichern und laden:

![image-20250704125501623](assets/image-20250704125501623.png)

Sie können \*.toml-Dateien auch per Drag & Drop auf das Frontpanel ziehen oder häufig verwendete Diagramme im Favoritenordner speichern. Wenn Sie Ihre \*.toml-Datei auf die Favoritenliste ziehen, wird die Datei in den Favoritenordner kopiert.

### Diagramme exportieren

Sie können Diagramme als PNG-Bilddatei exportieren:

![image-20250704125830396](assets/image-20250704125830396.png)

Dies erstellt eine PNG-Datei mit 970x700 Pixeln.

Sie können das Bild auch direkt aus der Software in die Zwischenablage kopieren:

![image-20250704130032570](assets/image-20250704130032570.png)

### Darstellung anpassen

Sie können im Ansichtsmenü zwischen hellem und dunklem Design mit oder ohne Achsen wählen:

![image-20250704130431084](assets/image-20250704130431084.png)

Außerdem können Sie die Sprache ändern – derzeit werden nur Englisch und Deutsch unterstützt.

### Qualitätskennzahlen

Sie können die Qualitätskennzahlen über das Hilfemenü einsehen:

![image-20250704130713084](assets/image-20250704130713084.png)

Viel Spaß!