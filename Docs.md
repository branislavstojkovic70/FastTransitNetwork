# Analiza Grafova za Dijagnostiku Kvarova na Mreži *FastTransitNetwork*

**Predmet:** Paralelni i distribuirani algoritmi i jezici (PDAJ)  
**Akademska godina:** 2025/26  
**Student:** Branislav Stojković  
**Broj indeksa:** SV64/2020
**Datum:** 09.02.2026.

---

## Sadržaj

1. [Uvod](#1-uvod)  
2. [Opis problema](#2-opis-problema)  
3. [Implementirani algoritmi](#3-implementirani-algoritmi)  
   - 3.1. BFS (Breadth-First Search)  
   - 3.2. WCC (Weakly Connected Components)  
   - 3.3. PageRank  
4. [Reprezentacija grafa](#4-reprezentacija-grafa)  
5. [Strategija verifikacije](#5-strategija-verifikacije)  
6. [Rezultati merenja](#6-rezultati-merenja)  
7. [Analiza performansi](#7-analiza-performansi)  
8. [Uska grla i optimizacije](#8-uska-grla-i-optimizacije)  
9. [Zaključak](#9-zaključak)  

---

## 1. Uvod

Ovaj projekat predstavlja implementaciju sistema za analitiku grafova velikih razmera u programskom jeziku **Rust**, sa ciljem dijagnostike kvarova i analize strukture mreže *FastTransitNetwork*. Sistem omogućava efikasnu obradu grafova korišćenjem **sekvencijalnih i paralelnih algoritama**, uz detaljnu verifikaciju tačnosti i merenje performansi.

Implementirana su tri fundamentalna algoritma analize grafova:

- **Breadth-First Search (BFS)** – analiza dostupnosti i udaljenosti čvorova,
- **Weakly Connected Components (WCC)** – identifikacija povezanih komponenti mreže,
- **PageRank** – procena važnosti čvorova radi prioritizacije monitoringa.

Poseban fokus stavljen je na **paralelizaciju**, **skalabilnost** i **analizu performansi** na grafovima sa do **100 miliona čvorova i 500 miliona ivica**.

---

## 2. Opis problema

FastTransitNetwork upravlja složenom infrastrukturnom mrežom koja se sastoji od velikog broja čvorova (stanice, kontrolni sistemi) i veza. Tokom incidenata, kao što su kvarovi čvorova ili prekidi veza, neophodno je brzo odgovoriti na sledeća pitanja:

1. **Dostupnost** – koji su čvorovi dostižni iz datog izvora i na kojoj udaljenosti?
2. **Povezanost** – koji delovi mreže ostaju međusobno povezani ako se zanemari smer veza?
3. **Važnost** – koji čvorovi imaju najveći uticaj na stabilnost sistema?

### Sistemski zahtevi

- Obrada grafova velikih razmera (100M+ čvorova),
- Sekvencijalne i paralelne implementacije algoritama,
- Verifikacija korektnosti rezultata,
- Detaljna analiza performansi i skalabilnosti.

---

## 3. Implementirani algoritmi

### 3.1. BFS (Breadth-First Search)

**BFS** je algoritam obilaska grafa koji pronalazi najkraće putanje (u broju ivica) od izvornog čvora do svih dostižnih čvorova. U kontekstu mrežne dijagnostike koristi se za proveru dostupnosti sistema i identifikaciju izolovanih segmenata.

Sekvencijalna verzija ima vremensku složenost **O(V + E)**, dok paralelna verzija koristi **level-synchronous** pristup, gde se svi čvorovi na istom nivou obrađuju paralelno. Paralelizacija zahteva sinhronizaciju između nivoa, što uvodi dodatni overhead.

---

### 3.2. WCC (Weakly Connected Components)

Algoritam **WCC** identifikuje grupe čvorova koje su međusobno povezane kada se zanemari smer ivica. Implementacija se zasniva na **Union-Find (Disjoint Set Union)** strukturi podataka, uz optimizacije **path compression** i **union by rank**.

Paralelna verzija koristi **lock-free atomic operacije**, čime se postiže visoka skalabilnost i minimalna sinhronizacija. Vremenska složenost je efektivno **O(E)**.

---

### 3.3. PageRank

**PageRank** algoritam procenjuje važnost čvorova na osnovu strukture grafa. Implementiran je metodom **power iteration**, uz damping faktor α = 0.85.

Paralelizacija je ostvarena korišćenjem **fold + reduce** obrasca, sa thread-lokalnim baferima, čime se izbegavaju lock-ovi i postiže izuzetna skalabilnost. PageRank pokazuje najbolje performanse među implementiranim algoritmima.

---

## 4. Reprezentacija grafa

Graf je predstavljen korišćenjem **CSR (Compressed Sparse Row)** formata, koji je posebno pogodan za retke grafove i paralelnu obradu.

Struktura obezbeđuje:

- memorijsku složenost **O(V + E)**,
- sekvencijalni pristup susedima (cache-friendly),
- thread-safe čitanje bez sinhronizacije.

Ukupna memorijska potrošnja za graf sa 100M čvorova i 500M ivica iznosi približno **5.5 GB**.

---

## 5. Strategija verifikacije

Korektnost implementacije proverena je kroz:

- **56 unit testova** na malim i srednjim grafovima,
- poređenje sekvencijalnih i paralelnih verzija,
- toleranciju greške za PageRank < 1e-4.

Svi testovi prolaze uspešno u `--release` režimu.

---

## 6. Rezultati merenja

Testiranja su izvršena na grafovima različitih veličina i topologija (random, scale-free, grid, chain), uz merenje:

- vremena izvršavanja,
- speedup-a,
- efikasnosti paralelizacije.

Svi benchmark testovi izvršeni su u kontrolisanom okruženju na Linux platformi.

---

## 7. Analiza performansi

- **BFS** pokazuje ograničen speedup zbog nivo-sinhronizacije.
- **WCC** ostvaruje skoro linearnu skalabilnost.
- **PageRank** postiže najbolji speedup (do **13×** sa 32 threada).

Svi algoritmi su **memory-bound**, što dovodi do saturacije performansi pri većem broju threadova.

---

## 8. Uska grla i optimizacije

Identifikovana su sledeća uska grla:

- sinhronizacija po nivoima kod BFS-a,
- random pristupi memoriji kod WCC-a,
- veliki broj iteracija kod PageRank-a.

Primenjene optimizacije uključuju lock-free pristup, atomic operacije sa relaxed ordering-om i automatski fallback na sekvencijalne verzije za male grafove.

---

## 9. Zaključak

Implementiran je kompletan sistem za analizu grafova velikih razmera, sa:

- visokom korektnošću,
- dobrim skaliranjem na velikim grafovima,
- jasnom analizom performansi i ograničenja.

Rezultati pokazuju da su **embarrassingly parallel** algoritmi (WCC, PageRank) znatno pogodniji za paralelnu obradu u shared-memory okruženju od algoritama sa čestom sinhronizacijom (BFS).

---
