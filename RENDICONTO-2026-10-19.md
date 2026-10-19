# CADCraft — rendiconto completo della sessione cloud del 19/10/2026

Fork: `EP995LAB/cadcraft` · a monte: `storytold/cadcraft` · main a monte usato: **`0b32c2e`**
(«i18n(hu): the ViewCube pill's key is now WCS (after #1558)»).

Questo file serve a un'altra chat (o a Enrico) per **ricontrollare tutto senza fidarsi di questo
rendiconto**: ogni affermazione ha il commit, il comando o il test che la prova. In fondo ci sono la
lista dei controlli da rifare, i testi completi delle due PR nuove e il codice dei test su Esc (che
esiste solo qui: il ramo non è stato spinto).

---

## 1. Il comando ricevuto (riassunto)

0. Controllare collegamento, identità e remoti: `origin` = `EP995LAB/cadcraft`, aggiungere `upstream`,
   portare `main` del fork su `upstream/main` con fast-forward, identità `EP995LAB`, `gh` = `EP995LAB`.
   Mai force-push, mai fork/repo nuovi, mai commenti su PR e segnalazioni di altri.
1. Riallineare le 8 PR nostre in conflitto (#1200, #1202, #1203, #1218, #1220, #1221, #1222, #1462)
   con `git merge upstream/main`, dopo aver controllato che il main non le abbia già superate;
   per #1201 e #1219 (pulite) solo verificare la CI.
2. Esc non svuota la selezione (0.8.0, macOS): riprodurre con un test UI, trovare la causa, correggere. PR normale.
3. Doppio clic su un blocco: aprire «Edit Block Definition» con quel blocco. PR in bozza, «not yet checked in AutoCAD».
4. Apertura dal Finder / «Apri con» (#158): tipi AutoCAD/LibreOffice nell'Info.plist + evento Apple
   kAEOpenDocuments. PR in bozza con `fixes #158`.
5. Avviso «DWG: N of M object records couldn't be read…»: non toccare.

Catena obbligatoria per ogni ramo: controllo PR/issue a monte → test rosso → correzione →
`cargo xtask ci` con «CI summary» verde → revisione avversaria indipendente da un subagente → correzioni
→ CI → push senza force → PR.

---

## 2. Esito in una tabella

| Punto | Esito | Dove si vede |
|---|---|---|
| 0. Remoti e identità | ✅ fatto | `origin/main` = `0b32c2e` |
| 1. 8 PR in conflitto | ✅ tutte unite al main, CI verde, spinte | §5 |
| 1. #1201, #1219 | ✅ CI verde dopo un merge di prova in locale; non toccate | §5 |
| 2. Esc | ⚠️ **non riprodotto**, nessuna PR | §8 e Allegato C |
| 3. Doppio clic sui blocchi | ✅ ramo `block-double-click` spinto; **PR da aprire a mano** | §9, Allegato B |
| 4. Finder #158 | ✅ ramo `fix-finder-open` spinto; **PR da aprire a mano** | §10, Allegato A |
| 5. Avviso DWG | ✅ non toccato | — |

**Blocco principale:** la sessione aveva accesso all'API GitHub solo per `EP995LAB/cadcraft`.
- `gh api repos/storytold/cadcraft/...` → 403 «GitHub access to this repository is not enabled for this session».
- `gh pr list` → 403: GraphQL non è disponibile nelle sessioni Claude Code.
- `add_repo storytold/cadcraft`, sia in lettura sia in scrittura, è stato rifiutato dai permessi della sessione.

Conseguenze:
- non ho potuto **aprire le due PR nuove**;
- non ho potuto **cercare PR e issue aperte a monte** (passo 1 della catena);
- non ho potuto **leggere lo stato della CI a monte** di #1201 e #1219.

Con git, su repository pubblico, ho potuto scaricare il main a monte e le teste delle PR (`refs/pull/N/head`).

Link per aprire le due PR (in bozza):
- https://github.com/storytold/cadcraft/compare/main...EP995LAB:cadcraft:fix-finder-open?expand=1
- https://github.com/storytold/cadcraft/compare/main...EP995LAB:cadcraft:block-double-click?expand=1

---

## 3. Cronologia

1. **Punto 0.**
   - `git remote -v`: `origin` = `https://github.com/EP995LAB/cadcraft` ✔; `gh api user --jq .login` = `EP995LAB` ✔.
   - `git config user.name EP995LAB`, `user.email 293597958+EP995LAB@users.noreply.github.com`.
     Nel comando la chiocciola era quella larga `＠`; ho usato la `@` normale.
   - `git remote add upstream https://github.com/storytold/cadcraft.git`, con push disabilitato (`pushurl = DISABLED`).
   - `main` era 208 commit dietro e 0 avanti. `git merge --ff-only upstream/main` → `git push origin main`: `a12c85c..0b32c2e`.
2. Tentativo di accesso a storytold via API: negato (vedi §2).
3. Teste delle PR scaricate con `git fetch upstream refs/pull/N/head`. Tutte coincidevano con i rami del fork:

   | PR | Ramo | Testa |
   |---|---|---|
   | #1200 | pick-what-is-drawn | 8fd1185 |
   | #1201 | fix-mirror-text | 940c3d2 |
   | #1202 | fix-join | d5fc41b |
   | #1203 | fix-table-handles | 15bab23 |
   | #1218 | fix-trim-assoc | 0f590ab |
   | #1219 | fix-plot-scale | 4a67071 |
   | #1220 | fix-table-dxf2007 | fec7985 |
   | #1221 | fix-dwg-spline | e020801 |
   | #1222 | fix-unknown-block | dae88da |
   | #1462 | fix-arc-dimension | 4193ccd |

   Basi: 13/10, 14/10 per #1462. Tutti i rami erano 95–96 commit dietro il main.
4. Merge di prova, solo per sapere dove erano i conflitti:
   - pick-what-is-drawn: `cmd/inquiry.rs`, `render/src/lib.rs`
   - fix-join: `cmd/modify.rs`, `docs/geometry-parity.md`
   - fix-trim-assoc: `docs/geometry-parity.md`
   - fix-unknown-block: `engine/tests/audit.rs`, `docs/file-format-parity.md`
   - gli altri: solo `docs/file-format-parity.md`
   - **#1201 e #1219 puliti.**
5. Un worktree per ramo in `/home/user/wt/<ramo>`, `git merge --no-edit upstream/main`, conflitti risolti (§6), un merge commit per ramo con il trailer di attribuzione.
6. In parallelo: studio del punto 2 (Esc), scrittura dei rami dei punti 3 e 4.
7. **Prima build: disco pieno.** Le build di debug con le info di debug superavano i 30 GB disponibili.
   Ho pulito `target` e da lì ho usato `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, `*_STRIP=debuginfo`, `CARGO_INCREMENTAL=0`.
8. Test Esc: verdi sul main e anche sulla v0.8.0 → non riprodotto (§8).
9. Controllo «il main ha già risolto?»: i test nuovi di ogni PR copiati sul main puro → falliscono tutti (§4).
10. Tre revisioni indipendenti con subagenti che non avevano scritto il codice (§7).
11. Correzioni dalle revisioni: pick (compilazione e selezione ciclica), doppio clic (serie associative).
12. **Prima CI completa: falliva per l'ambiente.** Il test `gpu_viewport_stencil` dà «No adapter found» (il container non ha GPU).
    Ho installato `mesa-vulkan-drivers` (lavapipe); poi il test passa anche sul main.
13. CI completa su 12 rami in sequenza nella stessa cartella (§5).
    - pick: `cargo fmt` → corretto.
    - join: clippy `manual_range_contains` nel test della PR → corretto.
    - Rifatta la CI su questi due.
14. Push senza force di ogni ramo verde (§5).

**Errore mio a metà lavoro, poi corretto:** all'inizio ho usato un solo `CARGO_TARGET_DIR` per più
worktree. Cargo riusava artefatti compilati da un'altra cartella, quindi alcuni test giravano su
sorgenti sbagliati. Me ne sono accorto e da lì in poi:
- un `target` separato per ogni worktree dei test mirati;
- la CI completa sempre nell'unica cartella `/home/user/cadcraft`, con `git checkout --detach <ramo>`.

Tutte le righe «CI summary» di §5 vengono da quella cartella. Le conclusioni su Esc reggono lo stesso: la
prima prova sul main e quella sulla v0.8.0 sono state compilate da sorgente pulito, e §11 spiega come rifarle.

---

## 4. Il main aveva già risolto qualcuna delle 8? No

Ho copiato i file di test nuovi di ogni PR sul main puro (`0b32c2e`), li ho eseguiti, poi li ho cancellati.

| PR | File di test della PR eseguito sul main | Risultato sul main |
|---|---|---|
| #1200 | `engine/tests/pick_drawn.rs` | 14 ok, **29 FAILED** |
| #1202 | `engine/tests/join_gaps.rs` | 7 ok, **12 FAILED** |
| #1203 | `io/tests/dim_block_resave_roundtrip.rs` | 7 ok, **3 FAILED** (table/arrow/DWG seed) |
| #1218 | `engine/tests/trim_keeps_object.rs` | 8 ok, **7 FAILED** |
| #1220 | `io/tests/table_cell_value_2007.rs` | 3 ok, **5 FAILED** |
| #1221 | `engine/tests/dwg_fit_spline.rs` | 0 ok, **3 FAILED** |
| #1222 | `engine/tests/unknown_dim_block_kept.rs` e `io/tests/unknown_dim_block_kept.rs` | **non compilano** (mancano `Unknown::block_names` / `is_dimension`) |
| #1462 | `io/tests/arc_dimension.rs` e `engine/tests/dimarc_dxf.rs` | **12 FAILED** e **4 FAILED** |

Nei messaggi dei commit del main dal 12/10 nessuno cita #1161, #1162, #1164, #1166, #1169, #1174, #1177 o #1179.
**Nessuna PR nostra è superata dal main.**

---

## 5. Rami: prima → dopo, CI, push

CI = riga «CI summary» di `cargo xtask ci`, eseguita su Linux nella cartella `/home/user/cadcraft`.

| PR | Ramo | Testa prima | Testa dopo (spinta) | Commit aggiunti | CI summary |
|---|---|---|---|---|---|
| [#1200](https://github.com/storytold/cadcraft/pull/1200) | pick-what-is-drawn | 8fd1185 | **f64fd2c** | 263132b merge · b4dede9 correzione dopo la revisione · f64fd2c fmt | all 6 steps passed (fmt, clippy, test, assets, layers, wasm). Prima corsa su b4dede9: FAIL fmt |
| [#1202](https://github.com/storytold/cadcraft/pull/1202) | fix-join | d5fc41b | **2db1c91** | 08be163 merge · 2db1c91 clippy nel test | all 6 steps passed. Prima corsa su 08be163: FAIL clippy |
| [#1203](https://github.com/storytold/cadcraft/pull/1203) | fix-table-handles | 15bab23 | **57fcadf** | 57fcadf merge | all 6 steps passed |
| [#1218](https://github.com/storytold/cadcraft/pull/1218) | fix-trim-assoc | 0f590ab | **6c1db60** | 6c1db60 merge | all 6 steps passed |
| [#1220](https://github.com/storytold/cadcraft/pull/1220) | fix-table-dxf2007 | fec7985 | **92ff353** | 92ff353 merge | all 6 steps passed |
| [#1221](https://github.com/storytold/cadcraft/pull/1221) | fix-dwg-spline | e020801 | **f9ed4c7** | f9ed4c7 merge | all 6 steps passed |
| [#1222](https://github.com/storytold/cadcraft/pull/1222) | fix-unknown-block | dae88da | **a351ccf** | a351ccf merge | all 6 steps passed |
| [#1462](https://github.com/storytold/cadcraft/pull/1462) | fix-arc-dimension | 4193ccd | **7256a5c** | 7256a5c merge | all 6 steps passed |
| [#1201](https://github.com/storytold/cadcraft/pull/1201) | fix-mirror-text | 940c3d2 | 940c3d2 (non toccato) | — | all 6 steps passed, su un merge di prova locale `e2fce99`, non spinto |
| [#1219](https://github.com/storytold/cadcraft/pull/1219) | fix-plot-scale | 4a67071 | 4a67071 (non toccato) | — | all 6 steps passed, su un merge di prova locale `58f0f0b`, non spinto |
| nuova (fixes #158) | fix-finder-open | — | **4081a2a** | 4081a2a | all 6 steps passed |
| nuova | block-double-click | — | **055c783** | 3b279d7 · 26bf005 · 055c783 | all 6 steps passed |

- Ogni push è stato un fast-forward (`vecchia..nuova` nell'output di `git push`, mai `+`).
- Ogni ramo vecchio contiene esattamente un merge di `0b32c2e`.
- Nessuno squash, nessun rebase.

Output dei push, per riferimento:
```
15bab23..57fcadf  fix-table-handles     fec7985..92ff353  fix-table-dxf2007
e020801..f9ed4c7  fix-dwg-spline        4193ccd..7256a5c  fix-arc-dimension
dae88da..a351ccf  fix-unknown-block     0f590ab..6c1db60  fix-trim-assoc
8fd1185..f64fd2c  pick-what-is-drawn    d5fc41b..2db1c91  fix-join
* [new branch] block-double-click       * [new branch] fix-finder-open
a12c85c..0b32c2e  main
```

---

## 6. Come ho risolto i conflitti

- **Documenti** (`docs/file-format-parity.md`, `docs/geometry-parity.md`), su tutti i rami:
  - riga di stato: quella del main, con «Last updated: 2026-10-19» e il «Change» della nostra PR;
  - cronologia: la nostra riga sopra quella del main, con data rinnovata da 2026-10-14 a 2026-10-19.
  - Fatto con uno script che si ferma se trova un conflitto di forma diversa.
- **fix-join, `crates/engine/src/cmd/modify.rs`:**
  - tenuto il percorso nuovo del main: `join_now`, `join_source_input`, cLose su un arco singolo;
  - nella riga «N objects joined into 1 …» aggiunti i nomi `line`, `arc`, `circle` della nostra PR;
  - `join()` chiama ancora `join_on_one_line_or_circle` dopo il controllo dei layer bloccati.
  - Nel doc ho aggiornato la prima riga JOIN. Il main ha già righe BREAK e JOIN **duplicate**: la seconda riga JOIN, vecchia, non l'ho toccata.
- **pick-what-is-drawn:**
  - `inquiry.rs`: tenuti sia il filtro `selected` del main sia il nostro `TextCx`;
  - `render/src/lib.rs`: tenuto il `hairline` del main e il nostro `match lt.as_ref().filter(|_| b.opts.linetypes)`.
- **fix-unknown-block, `engine/tests/audit.rs`:** tenuti entrambi i test (il nostro e `auditctl_generates_adt_report_file` del main).

---

## 7. Revisioni indipendenti (subagenti che non avevano scritto il codice; sola lettura)

| Revisione | Rilievi veri | Cosa ho fatto |
|---|---|---|
| Merge degli 8 rami | **1.** pick: `crates/engine/src/cmd/inquiry/shapes.rs:69`, file nuovo del main (SHAPES #1507), chiamava `select::in_window` con 6 argomenti, ma la nostra PR ne richiede 7 → **non compilava**. **2.** pick: `pick_all` (selezione ciclica #1149 del main) misurava i blocchi con tutto il testo, a differenza di `pick` → un riferimento scelto dal clic poteva mancare dalla lista ciclica. Gli altri 7 rami: **nessun rilievo**. Verificato in particolare fix-trim-assoc contro il DIMASSOC per segmenti del main (632c2d4, #1064): in memoria resta solo `AssocSnap::Vertex`, quindi la PR è ancora corretta | Commit **b4dede9**: `shapes.rs` passa `&mut cx`; nuova funzione `pick_distance` usata sia da `pick_with` sia da `pick_all`. CI rifatta → verde |
| Doppio clic sui blocchi | **1.** Una serie associativa (riferimento a un blocco anonimo `*U`) apriva la finestra su «*U2» come blocco nuovo. **2.** Sul foglio in paper space il doppio clic non arriva mai al ramo nuovo (succede già per i tratteggi): la riga del doc non lo diceva | Commit **26bf005**: filtro con `blockedit::refusal`, che esclude xref e blocchi anonimi; test nuovo `an_associative_array_is_not_a_block_reference_to_open`, **rosso prima e verde dopo**; riga del doc corretta. 055c783: fmt |
| Finder #158 | **Nessun errore bloccante.** Verificati: ordine di registrazione dell'evento all'avvio, eventi in arrivo prima di `connect()`, niente doppio prestito del RefCell, Cmd+Q non cambiato, Info.plist, checksum in Cargo.lock = sha256 del .crate, MSRV (eframe 0.36 chiede già 1.95). Rilievi minori: riaprire dal Finder un disegno già aperto crea una seconda scheda (come File ▸ Open); un AppleScript `open` con un solo file viene ignorato dal crate; righe U13/P3 di `gaps.md` non aggiornate; da provare sul Mac `NSTreatUnknownArgumentsAsOpen` (argv più evento = due schede) | Nessuna modifica: tutto scritto nel testo della PR, parte «Not tested» |

---

## 8. Punto 2 — Esc: non riprodotto

**Ipotesi controllate nel codice:**
- `cmdline::keyboard` esce se `ctx.egui_wants_keyboard_input()`. Però egui 0.36 toglie il focus con Esc in `Focus::begin_pass`, e nessun nostro widget cattura Esc nel filtro degli eventi (cercati `escape: true` e `EventFilter`: nessuno).
- Nessun `consume_key(Escape)`.
- Il menu nativo (muda) non ha acceleratori su Esc.
- `request_focus` compare solo in campi di rinomina, quickcalc e schede di navigazione.
- `cmdline.rs` è identico tra v0.8.0 e main.

**Test scritti** (nel ramo locale `fix-esc-selection`, commit `edd5f13`, **non spinto**; codice nell'Allegato C):

| Test | main (0b32c2e) | v0.8.0 |
|---|---|---|
| clic sul canvas, Esc | ✅ passa | ✅ passa |
| riga di comando nascosta | ✅ | ✅ |
| puntatore su un grip | ✅ | ✅ |
| pressione e rilascio in frame separati, OS Mac, menu di sistema, titolo integrato, puntatore spostato su un pannello prima di Esc | ✅ | — (aggiunto dopo) |
| Quick Properties attivo | ✅ | — |
| Ctrl+A poi Esc | ✅ | — |

C'era anche un test con la finestra di selezione trascinata, ma l'ho tolto: nel banco di prova la selezione stessa non riusciva, quindi non diceva niente su Esc.

**Conclusione:** senza un test rosso non si pubblica una correzione. Da Enrico servono i passi esatti:
- cosa era selezionato e come (clic, finestra, comando);
- dov'era il puntatore;
- se un pannello o un campo aveva il focus;
- se era attivo Dynamic Input;
- se il problema c'è anche nella build del main.

---

## 9. Punto 3 — Doppio clic sui blocchi (ramo `block-double-click`)

- **Controllo sul main:** dopo la v0.8.0 nessun commit aggiunge il doppio clic sui blocchi; `canvas.rs` ha solo viewport, TEXTEDIT e Hatch Edit.
  #1213 (editor degli attributi) **non è unita** (`git merge-base --is-ancestor` falso), quindi gli attributi restano fuori.
- **Modifiche:**
  - `crates/engine/src/blockedit.rs`: `pub fn reference_block(d, h)`, il blocco che BEDIT apre per un riferimento (risolve i blocchi dinamici `*U`).
  - `crates/engine/src/cmd/blocks/bedit.rs`: `block_of` diventa `pub(crate)`.
  - `crates/ui-egui/src/bedit.rs`: `pub fn open_on(app, name)`.
  - `crates/ui-egui/src/canvas.rs`:
    - ramo del doppio clic subito dopo quello dei tratteggi; vale con nessun comando in corso e senza Block Editor aperto;
    - `block_reference_at` esclude i blocchi che `blockedit::refusal` rifiuta e i nomi con `|`;
    - 4 test nel modulo `block_double_click_tests`.
  - `docs/ui-parity.md`: riga di stato e una riga di cronologia con «not yet checked in AutoCAD».
- **Rosso e verde:**
  - `double_clicking_a_block_reference_opens_edit_block_definition_on_its_block` fallisce senza la chiamata `open_on`;
  - `an_associative_array_is_not_a_block_reference_to_open` fallisce senza il filtro `refusal`;
  - entrambi passano con la correzione.

---

## 10. Punto 4 — Finder #158 (ramo `fix-finder-open`, commit 4081a2a)

- **Il ramo `ep995` non è sul fork:** `git ls-remote origin` mostra solo i rami delle PR, `italian`, `transparency`, `model-layout-plot-type` e `fix-hatch-arcs`.
  Il modulo `open_documents.rs` di Enrico non c'era, quindi ho preso la strada delle tre PR a monte.
- **Le tre PR a monte (lette dai diff):**

  | PR | Autore e data | Contenuto |
  |---|---|---|
  | #683 | Isaac Levine, 12/10 | crate `fmv-macos-events` più Info.plist |
  | #1503 | Alex Ortiz, 17/10 | stesso codice di #683 più `gaps.md` |
  | #1347 | filipemirilli, 15/10 | come le altre, più il «quit» con il controllo delle modifiche non salvate e test (solo macOS) |

- **Il crate `fmv-macos-events 0.1.0`:**
  - scaricato da crates.io; sha256 `90642cf66dde7f205dc8b42dfe57b47d0f3a29f571b832742883ef512e23fe55`, uguale al checksum in Cargo.lock;
  - autore Quetzalcohuatl, repository fastmarkdownviewer, una sola versione, circa 44.800 download;
  - un file, letto per intero: gestore `NSAppleEventManager` per 'odoc' e 'quit', registrato subito e di nuovo su `NSApplicationWillFinishLaunching`;
  - contiene `expect` se non è sul thread principale; noi lo chiamiamo dal thread principale, in `main`.
- **Modifiche:**
  - `apps/cadcraft/src/open_documents.rs`, compilato solo con `#[cfg(any(target_os = "macos", test))]`:
    - logica comune `DocEvent`, `openable`, `dispatch`, la stessa su ogni piattaforma;
    - parte solo macOS `mac::AppleEvents` e `mac::poll`;
    - 6 test, che girano anche su Linux.
  - `apps/cadcraft/src/main.rs`: modulo, campo `apple_events`, `install()` prima di `run_native`, `poll` nel creatore dell'app e in `logic`.
  - `apps/cadcraft/Cargo.toml`: `fmv-macos-events = "=0.1.0"`, solo per macOS, con commento.
  - `Cargo.lock`: +1 pacchetto, nessuna altra dipendenza nuova.
  - `packaging/macos/Info.plist.in`: aggiunti `com.autodesk.autocad.dwg` e `org.libreoffice.dxf-document`.
  - Nessun `unsafe` nel nostro repository.
- **Verifiche:**
  - 6 test verdi su Linux;
  - `cargo check -p cadcraft --target aarch64-apple-darwin` e `cargo clippy ... --all-targets -D warnings`: nessun errore;
  - CI completa verde;
  - **mai eseguito su un Mac.**
- **Correzione della frase richiesta per la PR.** Il comando chiedeva «verified on our 0.5.0 build on macOS 27». Quella build verificava il nostro vecchio gestore, non questo codice. Nel testo ho scritto: «our own Apple-event handler was verified on our 0.5.0 build on macOS 27 … this port … uses the crate the three upstream PRs use, and it is not yet run on a Mac.»

---

## 11. Come ricontrollare (istruzioni per un'altra chat)

```
# 0. Remoti
git fetch origin && git fetch https://github.com/storytold/cadcraft.git main
git rev-parse --short origin/main            # atteso 0b32c2e

# 1. Teste e niente force-push (la vecchia testa deve essere antenata della nuova)
for p in pick-what-is-drawn:8fd1185:f64fd2c fix-join:d5fc41b:2db1c91 fix-table-handles:15bab23:57fcadf \
         fix-trim-assoc:0f590ab:6c1db60 fix-table-dxf2007:fec7985:92ff353 fix-dwg-spline:e020801:f9ed4c7 \
         fix-unknown-block:dae88da:a351ccf fix-arc-dimension:4193ccd:7256a5c; do
  IFS=: read b old new <<<"$p"
  echo "$b $(git rev-parse --short origin/$b) atteso $new; antenata: $(git merge-base --is-ancestor $old origin/$b && echo si || echo NO); contiene main: $(git merge-base --is-ancestor 0b32c2e origin/$b && echo si || echo NO)"
done
git rev-parse --short origin/block-double-click origin/fix-finder-open   # 055c783 4081a2a

# 2. Cosa cambia ogni ramo rispetto al main
git diff 0b32c2e origin/<ramo> --stat

# 3. CI (in Linux senza GPU: apt-get install mesa-vulkan-drivers)
git checkout --detach origin/<ramo> && cargo xtask ci     # leggere la riga "CI summary"

# 4. "Il main l'aveva già risolto?": copiare sul main i test nuovi della PR (§4) ed eseguirli: devono fallire

# 5. Su GitHub (serve accesso a storytold): le PR #1200 #1202 #1203 #1218 #1220 #1221 #1222 #1462
#    puntano alle teste nuove e non sono più in conflitto; le due PR nuove vanno aperte in bozza
#    con i testi degli Allegati A e B.
```

Cose che una chat non può controllare da sola:
- il comportamento sul Mac (prove di Enrico, Allegato A, «Not tested»);
- lo stato GitHub a monte, se anche lei non ha accesso a storytold.

---

## 12. Cose viste e non toccate

- `docs/geometry-parity.md` sul main ha righe BREAK e JOIN duplicate.
- fix-trim-assoc, già prima del merge: gli attacchi `Start`/`End` su una LwPolyline (dati vecchi) non vengono tolti dal TRIM.
- Alcune righe nostre già esistenti nei documenti affermano comportamenti di AutoCAD; esempio in #1462: «group 70 … as AutoCAD 2027 writes it». Sono di prima e non le ho cambiate; vanno ricontrollate secondo la regola «not yet checked in AutoCAD».
- PR a monte sovrapposte al punto 4: #683, #1503, #1347, tutte in conflitto. Al punto 3: #1609, che chiede la modifica dei blocchi sul posto; non commentata.
- Nessun commento, risposta o segnalazione scritta su GitHub. Nessun fork, repo o gist creato. `it.tsv` e le traduzioni non toccati.

---

## Allegato A — Testo PR «macOS: open drawings from Finder…» (fixes #158), da aprire in bozza

```markdown
## macOS: open drawings from Finder, Open With, the Dock and `open -a` (fixes #158)

**Draft:** our own Apple-event handler was verified on our 0.5.0 build on macOS 27 (it opened DWGs from Finder); that module is not on the fork any more, so this port to main uses the crate the three upstream PRs use, and it is not yet run on a Mac.

### What changes
1. **Open With.** `packaging/macos/Info.plist.in` declared only our own `ai.storyteller.cadcraft.dwg/dxf` types. With AutoCAD or LibreOffice installed, `.dwg` and `.dxf` files carry `com.autodesk.autocad.dwg` and `org.libreoffice.dxf-document`, so Finder never offered CADCraft. Those are now claimed too (rank Alternate, as before).
2. **The open event.** A Finder open arrives as a `kAEOpenDocuments` ('odoc') Apple event, which winit 0.30 doesn't handle, so AppKit answered "CADCraft cannot open files in the "Drawing" format". `apps/cadcraft/src/open_documents.rs` registers a handler through the pinned `fmv-macos-events = "=0.1.0"` crate (one source file, read before pinning; checksum in Cargo.lock): an `NSAppleEventManager` handler for 'odoc' and 'quit', registered before the launch event arrives, that leaves winit's application delegate alone. No `unsafe` in this repository; `unsafe_code = "forbid"` stays.
   - Each path opens once, in order, through the same path as File ▸ Open (failures reported on the command line).
   - A system quit (Dock ▸ Quit, log out, `osascript`) asks about unsaved drawings like the close button; Cmd+Q is unchanged.

### Compared with #683, #1503 and #1347 (same approach, in conflict with main)
Same crate and the same Info.plist types; the dispatch with the unsaved-changes check comes from #1347. What differs: the handling is platform-independent (`DocEvent`) and tested on every platform, not only on macOS; only the Apple-event bridge is macOS-only. Rebased on today's main.

### Tests
`apps/cadcraft/src/open_documents.rs`: order and duplicates, non-UTF-8 paths, a real DWG and DXF (names with spaces and decomposed accents) opened beside an unsaved drawing, a failed open reported with its path, quit with unsaved work asks and can be cancelled, quit with nothing unsaved closes. `cargo check` and `cargo clippy --all-targets -D warnings` for `aarch64-apple-darwin` pass.

`cargo xtask ci` on 4081a2a (Linux): `CI summary: all 6 steps passed (fmt, clippy, test, assets, layers, wasm)`

### Not tested
- Not run on a Mac (the Apple-event bridge and main.rs wiring can't be unit-tested here).
- To check on the Mac: Finder double-click with the app closed and open; Open With on a `.dwg` that AutoCAD owns; drop on the Dock icon; `open -a CADCraft plan.dxf`; that `CADCraft.app/Contents/MacOS/CADCraft plan.dxf` opens one tab, not two (AppKit's `NSTreatUnknownArgumentsAsOpen`).
- Known, unchanged: opening a drawing that is already open adds a second tab (as File ▸ Open and drag-and-drop do); an AppleScript `open` of a single file (not a list) is ignored by the crate's handler. `docs/gaps.md` rows U13/P3 are left for after the Mac check.
```

## Allegato B — Testo PR «Double-clicking a block reference…», da aprire in bozza

```markdown
## Double-clicking a block reference opens Edit Block Definition on its block

**Draft:** what AutoCAD does when a block reference is double-clicked is not yet checked in AutoCAD; it will be tried on a Mac before this leaves draft.

### What changes
- With no command running and the Block Editor closed, a double-click on a block reference on the canvas opens the **Edit Block Definition** dialog (`bedit.rs`, from #824/#833) with that reference's block chosen; OK opens the Block Editor on it. Before, the double-click did nothing (canvas double-click handled only viewports, TEXTEDIT and Hatch Edit).
- A dynamic block's reference shown through an anonymous `*U` block chooses the dynamic block (the same lookup BEDIT uses for a selected reference, now `cadcraft_engine::blockedit::reference_block`).
- Blocks the Block Editor refuses are left alone: external references and anonymous blocks such as associative arrays (`blockedit::refusal`).
- Like any idle double-click it starts no grip edit (`idle_dbl`).
- Blocks with attributes open the same dialog: the Enhanced Attribute Editor (#1213) is not on main, so attribute editing by double-click is left out.
- Works in model space and in an active viewport; on the paper itself the double-click still goes to the viewports (as for hatches).

Related: #1609 asks for in-place editing of blocks; this PR does not do that.

### Tests
`crates/ui-egui/src/canvas.rs`, `block_double_click_tests`: `double_clicking_a_block_reference_opens_edit_block_definition_on_its_block` (fails on main), `an_associative_array_is_not_a_block_reference_to_open`, `double_clicking_anything_else_does_not_open_it`, `double_clicking_a_reference_inside_the_block_editor_does_not_open_it`.

`cargo xtask ci` on 055c783: `CI summary: all 6 steps passed (fmt, clippy, test, assets, layers, wasm)`

### Not tested
- Not run in the app (only the egui test harness); not yet checked in AutoCAD.
- Paper-space block references (title blocks) are not covered.
```

## Allegato C — I test su Esc (ramo locale `fix-esc-selection`, commit edd5f13, non spinto)

Vanno in fondo a `crates/ui-egui/src/canvas.rs`, sopra `0b32c2e`. Si eseguono con
`cargo test -p cadcraft-ui-egui --lib escape_tests`: sul main passano tutti e 6.

```rust
#[cfg(test)]
mod escape_tests {
    use cadcraft_engine::Session;
    use cadcraft_geom::Vec2;
    use serde_json::json;

    use crate::{CadApp, Services};

    /// One frame of the whole window at `time`.
    fn frame(app: &mut CadApp, ctx: &egui::Context, events: Vec<egui::Event>, time: f64) {
        app.logic(ctx);
        let screen = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1600.0, 1000.0)));
        let input = egui::RawInput { screen_rect: screen, events, time: Some(time), ..Default::default() };
        let mut out = ctx.run_ui(input, |ui| app.ui(ui));
        out.textures_delta.clear();
    }

    fn key(key: egui::Key) -> Vec<egui::Event> {
        let m = egui::Modifiers::NONE;
        vec![
            egui::Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers: m },
            egui::Event::Key { key, physical_key: None, pressed: false, repeat: false, modifiers: m },
        ]
    }

    fn click(at: egui::Pos2) -> Vec<egui::Event> {
        let button = |pressed| egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed, modifiers: egui::Modifiers::default() };
        vec![button(true), button(false)]
    }

    /// A line picked with a click on the canvas, no command running: `(app, ctx, time)`.
    fn picked_line(setup: impl FnOnce(&mut CadApp)) -> (CadApp, egui::Context, f64) {
        let mut app = CadApp::new(Session::new(), Services::default());
        app.session.execute("line", &json!({ "points": [[0, 0], [10, 0]] })).unwrap();
        let _ = app.session.zoom_extents();
        setup(&mut app);
        let ctx = egui::Context::default();
        frame(&mut app, &ctx, Vec::new(), 0.0);
        let at = app.canvas.xf.unwrap().to_screen(Vec2::new(3.0, 0.0));
        frame(&mut app, &ctx, vec![egui::Event::PointerMoved(at)], 0.1);
        frame(&mut app, &ctx, click(at), 0.2);
        // Past the double-click time, so the next click is a click of its own.
        frame(&mut app, &ctx, Vec::new(), 1.0);
        assert_eq!(app.session.selection().len(), 1, "the click picks the line");
        assert!(app.session.running.is_none(), "{:?}", app.session.running.as_ref().map(|r| r.machine.name()));
        (app, ctx, 1.1)
    }

    /// The window as the macOS app shows it: the system menu bar, the integrated title bar.
    fn mac(app: &mut CadApp) {
        app.ui.in_window_menu = false;
        app.integrated_titlebar = true;
    }

    /// Press and release in frames of their own, as a real mouse sends them, a few pixels apart.
    #[test]
    fn escape_clears_a_selection_clicked_over_several_frames_on_a_mac() {
        let mut app = CadApp::new(Session::new(), Services::default());
        mac(&mut app);
        app.session.execute("line", &json!({ "points": [[0, 0], [10, 0]] })).unwrap();
        let _ = app.session.zoom_extents();
        let ctx = egui::Context::default();
        ctx.set_os(egui::os::OperatingSystem::Mac);
        frame(&mut app, &ctx, Vec::new(), 0.0);
        let at = app.canvas.xf.unwrap().to_screen(Vec2::new(3.0, 0.0));
        let m = egui::Modifiers::default();
        frame(&mut app, &ctx, vec![egui::Event::PointerMoved(at)], 0.1);
        frame(&mut app, &ctx, vec![egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed: true, modifiers: m }], 0.2);
        let up = at + egui::vec2(1.0, 0.0);
        frame(&mut app, &ctx, vec![egui::Event::PointerMoved(up), egui::Event::PointerButton { pos: up, button: egui::PointerButton::Primary, pressed: false, modifiers: m }], 0.3);
        frame(&mut app, &ctx, Vec::new(), 1.0);
        assert_eq!(app.session.selection().len(), 1);
        // The pointer leaves the canvas for a palette before Esc.
        frame(&mut app, &ctx, vec![egui::Event::PointerMoved(egui::pos2(1590.0, 500.0))], 1.1);
        frame(&mut app, &ctx, key(egui::Key::Escape), 1.2);
        frame(&mut app, &ctx, Vec::new(), 1.3);
        assert!(app.session.selection().is_empty(), "{:?}", app.session.selection());
    }

    /// Ctrl+A (Select All), then Esc.
    #[test]
    fn escape_clears_select_all() {
        let mut app = CadApp::new(Session::new(), Services::default());
        app.session.execute("line", &json!({ "points": [[0, 0], [10, 0]] })).unwrap();
        let ctx = egui::Context::default();
        frame(&mut app, &ctx, Vec::new(), 0.0);
        let _ = app.run("selectall", json!({}));
        frame(&mut app, &ctx, Vec::new(), 0.1);
        assert_eq!(app.session.selection().len(), 1);
        assert!(app.session.running.is_none(), "{:?}", app.session.running.as_ref().map(|r| r.machine.name()));
        frame(&mut app, &ctx, key(egui::Key::Escape), 0.2);
        frame(&mut app, &ctx, Vec::new(), 0.3);
        assert!(app.session.selection().is_empty(), "{:?}", app.session.selection());
    }

    /// Esc with objects picked on the canvas and no command running clears the selection.
    #[test]
    fn escape_clears_a_selection_picked_on_the_canvas() {
        let (mut app, ctx, t) = picked_line(|_| {});
        frame(&mut app, &ctx, key(egui::Key::Escape), t);
        frame(&mut app, &ctx, Vec::new(), t + 0.1);
        assert!(app.session.selection().is_empty(), "{:?}", app.session.selection());
    }

    /// With Quick Properties on (its palette shows beside the picked object).
    #[test]
    fn escape_clears_the_selection_with_quick_properties_on() {
        let (mut app, ctx, t) = picked_line(|app| {
            app.session.execute("qpmode", &json!({ "on": true })).unwrap();
        });
        frame(&mut app, &ctx, Vec::new(), t);
        frame(&mut app, &ctx, key(egui::Key::Escape), t + 0.1);
        frame(&mut app, &ctx, Vec::new(), t + 0.2);
        assert!(app.session.selection().is_empty(), "{:?}", app.session.selection());
    }

    /// The same with the command line hidden (Cmd+9).
    #[test]
    fn escape_clears_the_selection_with_the_command_line_hidden() {
        let (mut app, ctx, t) = picked_line(|app| app.ui.show_command_line = false);
        frame(&mut app, &ctx, key(egui::Key::Escape), t);
        frame(&mut app, &ctx, Vec::new(), t + 0.1);
        assert!(app.session.selection().is_empty(), "{:?}", app.session.selection());
    }

    /// The pointer still over the picked object (its grips drawn hot under it) when Esc comes.
    #[test]
    fn escape_clears_the_selection_with_the_pointer_on_a_grip() {
        let (mut app, ctx, t) = picked_line(|_| {});
        let grip = app.canvas.xf.unwrap().to_screen(Vec2::new(10.0, 0.0));
        frame(&mut app, &ctx, vec![egui::Event::PointerMoved(grip)], t);
        frame(&mut app, &ctx, key(egui::Key::Escape), t + 0.1);
        frame(&mut app, &ctx, Vec::new(), t + 0.2);
        assert!(app.session.selection().is_empty(), "{:?}", app.session.selection());
    }
}
```
