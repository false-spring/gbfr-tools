# language-unpack

Unpacks language msgpack files and exports them as CSVs.

Usage:

> language-unpack.exe <file.msg>

CSV Format:

- id (xxhash32 of id_hash\_)
- id_hash\_ (identifier for the translation msg id)
- sub_id_hash\_ (sub-identifer? maybe translated sub-attribute)
- text\_ (actual text)
