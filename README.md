# Benerin

> Benerin ini dong!

One-stop library dan API untuk berbagai task tentang NLP dalam bahasa Indonesia yang efisien dan kencang, meliputi...

### Tokenisasi `/tokenize`

Memproses teks indonesia menjadi token yang dapat di-index. Proses ini juga dapat melakukan normalisasi, stemming, spellcheck, deteksi*, ekstrasi* dan analisis*. Use case meliputi:

- Membangun fitur pencarian di website dengan akurat
- Membangun fitur spell-checking untuk mendeteksi typo di teks editor
- Mendeteksi perbedaan antara dua teks dengan pendekatan token
- Analisis sentimen, kalimat spam, tidak pantas, dll (coming soon!)
- Mendeteksi kata non formal, kata slang, singkatan (coming soon!)
- Ekstraksi data biografis seperti alamat, nama, kontak, dll (coming soon!)

*) Coming soon.

### Postal `/postal`

Memproses teks alamat indonesia menjadi entity yang terpisah seperti nama jalan, kota, daerah. Use case meliputi:

- Memproses data alamat yang tidak terstruktur menjadi rapi
- Membangun input alamat yang efisien (menghindari input terpisah dari provinsi s/d desa)
- Mendapatkan data geolokasi atau kode wilayah dari teks alamat (coming soon!)

## Demo dan Repo

API ini bersifat publik pada endpoint berikut [api.benerin.web.id](https://api.benerin.web.id). Membuka laman API berikut akan membuka laman Swagger API.

Terdapat pula contoh demo website yang dapat menggunakan API di laman [benerin.web.id](https://benerin.web.id).

Repo untuk dataset dan demo website merupakan repo terpisah dan dapat dijumpai di [willnode/benerin-data](https://github.com/willnode/benerin-data) dan [willnode/benerin-web](https://github.com/willnode/benerin-web).

Benerin API publik di hosting dengan [DOM Cloud](https://domcloud.co). Saat ini jika endpoint tidak di hit >15 menit, app nya perlu dinyalain lagi dengan load data yang berat lagi (sekitar 1 GB) dan kadang bisa sampai timeout karena libpostal butuh banyak memori untuk load data. Masalah ini akan dioptimasi lagi nanti.

## Kontribusi

Kami terbuka untuk kontribusi! Anda juga dapat memberi ide use case baru yang menarik untuk dijadikan API melalui GitHub issues.
