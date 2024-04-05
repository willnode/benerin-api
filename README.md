# Benerin

> Benerin ini dong!

One-stop library untuk berbagai task tentang NLP dalam bahasa Indonesia untuk production, meliputi...

- Stemming -> Indexing dan Search Engine
- Spellcheck -> Cek typo dan (opsional) grammar  
- Addressing -> Pecah alamat dan cari pelengkap (e.g. kode zip)
- Tokenization -> Pecah teks menjadi token dan sebaliknya
- Grammar -> Identifikasi jenis kata (benda/kerja/nama, dll)

Satu fitur saling melengkapi sama lain, makanya dijadikan satu di library ini.

Output dari repo ini bisa jadi C library, bisa jadi executable yang berbentuk CLI command `benerin` ataupun gRPC server `benerind`.

Ditulis pakai Rust biar kenceng dan portabel (bisa dicompile ke wasm atau package di bahasa lain kedepannya).

Dataset yang dipakai berada diluar repo ini, lihat willnode/benerin-data.

## Depedency Tree

```
daemon
|-- api
cmd
|-- api
api
|-+ stemming
| |-  graph
| |-  data
|-+ spellcheck
| |-  graph
| |-  data
|-+ grammar
| |-  graph
| |-  data
|-+ addressing
|-+ tokenizer
| |-  graph
```
