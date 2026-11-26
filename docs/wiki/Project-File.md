🌐 **English** · [Polski](Plik-projektu) · [简体中文](Project-File-ZH) · [한국어](Project-File-KO)

# Project File

PenCreator saves **`.cre`**, the project file of baseCreator that LightCreator and MillCreator write too: one JSON file with the drawing, the description and **one section per program** (`laser`, `mill`, `pen`). Each program writes its own section and keeps the sections of the others untouched, so a project passes between programs without losing anything.

The `pen` section holds the 64 layers (name, colour, shown and drawn, the drawing, the pen of the library, pen width, speed, fill gap and angle, crossed lines, how many times to draw, the picture method and dithering, the place in the order of the job), **your own pens that the layers use** (so that a project opened on another computer brings them along; they join **My pens** there) and a note of the plotter the project was made for (its name and sheet size); when you open the project for a device with another sheet size, the status line says so. A layer with a number out of range is replaced by the default layer.

The description (the **Description** tab) is part of the project; PenCreator also writes it into the G-code as `(Note: …)` lines. A project that has a file is saved to it by itself about 20 seconds after a change (see [Settings and Files](Settings-and-Files)). A project made in LightCreator or MillCreator opens here with its drawing and description and the default pens; when you save it, their sections are written back as they were. A file with a newer container format (or a newer `pen` section) than the program knows is refused with a message.

## Password protection

A project can be protected with passwords: in **Project setup…** of LightCreator, MillCreator and PenCreator, and in **File → Page setup…** of BaseCreator, part **Protection**.

* **Owner password**: everything (editing, copying, exporting, changing or removing the protection).
* **Open password** (optional): the project can be opened, looked at, turned into G-code and run on the machine, and the material and the layer settings (speed, power, passes, depth, bit, pen) can be changed, but the drawing cannot be edited, copied, exported as a drawing (SVG, DXF, Illustrator, LightBurn …) or printed. The built-in web page shows no drawing then.
* With only an owner password, nobody else can open the file. Without a password the file cannot be read at all, and a **forgotten password cannot be recovered**.
* A project opened with the open password can be unlocked with the owner password, without opening it again: in the **Protection** part of the setup (**Owner password**, **Unlock**) or with **Unlock with the owner password…** in the menu of its tab (right click). That menu also has **Project information…** and the setup of the project.

The file stays a JSON text with the key `protected_cre`; the project in it is encrypted with **XChaCha20-Poly1305** under a random 256-bit key, which is stored once for every password, wrapped with a key made from the password by **Argon2id** (64 MiB, 3 passes, a salt of its own). Every save uses a new nonce; a changed file does not open. Passwords need at least 8 characters, and the open password must differ from the owner's.

What the open password forbids is enforced by the programs, as with the owner password of a PDF: whoever has it can, in principle, decrypt the project with other tools. Keep the owner password for yourself.
