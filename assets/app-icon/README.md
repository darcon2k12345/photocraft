# Patronus Photo app icon

The Patronus shield with a "P", in blue for Patronus Photo (green for Patronus Design, red for Patronus
PDF, silver for the Patronus Launcher), on a dark rounded tile so that it reads on light and dark
backgrounds.

`source/patronus-photo-shield.png` is the transparent shield master. The other files in this directory
(`patronusphoto-1024.png`, `patronusphoto.ico`, `patronusphoto.icns`, `patronusphoto.svg`,
`patronusphoto-small.svg` and `hicolor/`) are generated from it by `packaging/make-icons.py` (needs
Pillow) and are committed, so builds never need Pillow:

```
python3 packaging/make-icons.py assets/app-icon/source/patronus-photo-shield.png assets/app-icon patronusphoto com.patronus.patronusphoto
```

The master was cut from the logo supplied by the project owner, so its resolution is limited: a vector
original would give sharper large sizes.
