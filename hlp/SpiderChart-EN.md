## Spider Chart Diagram

Small Tool to draw Net Summary Plot (Spider Chart Diagram) according to ASTM E2597/E2597M-22:

![image-20250703163914941](assets/image-20250703163914941.png)

### User Interface

Just enter readings of six measured parameters, they will be turned into Quality Numbers and drawn as Spider Chart Diagram (don't forget to select IQI Type):

![image-20250704122743585](assets/image-20250704122743585.png)

### Save and load your Diagrams

You can save your data to the TOML Files (**TOML** stands for **"Tom's Obvious, Minimal Language."** ) TOML is often used in projects written in languages like Rust. It is similar to JSON, YAML, or INI files and easy to read:

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

You can save and load these files from File Menu or using these two buttons:

![image-20250704125501623](assets/image-20250704125501623.png)

You can also drag \*.toml files to the Front Panel or save most used diagrams into favorites folder. If you will drag your \*.toml file on the Favorites List, then the file will be copied to the favorites folder.

### Export Diagrams

You can export Diagrams to PNG image File:

![image-20250704125830396](assets/image-20250704125830396.png)

This will create PNG File 970x700 pixels.

You can also copy this image to the Clipboard directly from the software:

![image-20250704130032570](assets/image-20250704130032570.png)

### Changing Appearance

You can choose Dark or Light Theme with Axis or without from View Menu:

![image-20250704130431084](assets/image-20250704130431084.png)

Also you can change language, currently the only English and German are supported.

### Quality Numbers

You can see Quality Numbers from Help Menu:

![image-20250704130713084](assets/image-20250704130713084.png)

Enjoy!
