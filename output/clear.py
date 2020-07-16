import os
import shutil

d = './'
subfolders = [ f.path for f in os.scandir(d) if f.is_dir() ]

for dir in subfolders:
    shutil.rmtree(dir)

images = [ f.path for f in os.scandir(d) if f.is_file() and f.name.endswith('.png') ]

for image in images:
    os.remove(image);
