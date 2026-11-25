#!/usr/bin/env python3
"""Build every icon file from the transparent shield masters (assets/app-icon/source/*.png).

Each icon is the shield on a dark rounded tile, so it reads on light and dark backgrounds.
Needs Pillow. Usage: python3 packaging/make-icons.py <master.png> <outdir> <basename> <app-id>
"""
import base64, io, os, sys
from PIL import Image, ImageDraw

def tile(master, size, mac=False):
    S = 1024
    body = int(S * (824 / 1024)) if mac else S          # macOS: Apple's 824/1024 grid with a margin
    img = Image.new('RGBA', (S, S), (0, 0, 0, 0))
    t = Image.new('RGBA', (body, body), (0, 0, 0, 0))
    ImageDraw.Draw(t).rounded_rectangle((0, 0, body - 1, body - 1), radius=int(body * 0.225), fill=(11, 11, 15, 255))
    sh = master.resize((int(body * 0.80),) * 2, Image.LANCZOS)
    t.alpha_composite(sh, ((body - sh.width) // 2, (body - sh.height) // 2))
    img.alpha_composite(t, ((S - body) // 2, (S - body) // 2))
    return img.resize((size, size), Image.LANCZOS)

def main(master_path, out, base, app_id):
    master = Image.open(master_path).convert('RGBA')
    os.makedirs(out, exist_ok=True)
    tile(master, 1024).save(f'{out}/{base}-1024.png')
    tile(master, 512, mac=True).save(f'{out}/{base}-macos-512.png')
    for s in (16, 24, 32, 48, 64, 128, 256, 512):
        d = f'{out}/hicolor/{s}x{s}/apps'; os.makedirs(d, exist_ok=True)
        tile(master, s).save(f'{d}/{app_id}.png')
    big = tile(master, 256)
    big.save(f'{out}/{base}.ico', sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
    tile(master, 1024).save(f'{out}/{base}.icns')
    buf = io.BytesIO(); tile(master, 512).save(buf, 'PNG'); b64 = base64.b64encode(buf.getvalue()).decode()
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512"><image width="512" height="512" xlink:href="data:image/png;base64,{b64}"/></svg>'
    open(f'{out}/{base}.svg', 'w').write(svg); open(f'{out}/{base}-small.svg', 'w').write(svg)
    os.makedirs(f'{out}/hicolor/scalable/apps', exist_ok=True); open(f'{out}/hicolor/scalable/apps/{app_id}.svg', 'w').write(svg)
    for s in (64, 96):                                        # raw RGBA for the launcher window
        open(f'{out}/{base}-{s}.rgba', 'wb').write(tile(master, s).tobytes())

if __name__ == '__main__':
    main(*sys.argv[1:5])
