<div align="center">

<img src="app-icon.png" width="96" alt="">

# ASCII Local

**Transforme tes photos et vidéos en art ASCII, dither, glitch, verre… et anime-les en GIF.**
**100 % hors-ligne : tes images ne quittent jamais ton appareil.**

Windows · macOS · Android · Web (iPhone)

<img src="docs/screenshot.png" alt="Capture d'écran d'ASCII Local" width="820">

<img src="docs/demo-glitch.gif" width="300" alt="Animation Glitch machine"> <img src="docs/demo-led.gif" width="300" alt="Animation Matrice pop">

</div>

---

## Ce que ça fait

- **43 styles** : ASCII (18 jeux de caractères), blocs, braille, pixel art, mosaïque, hexagones, LEGO, voxel, LED, dither (Game Boy, PICO‑8, C64, Mac 1984…), halftone, gravure, pointillisme, Warhol, duotone, thermique, pixel sort, anaglyphe 3D, kaléidoscope, **16 effets de verre** (WebGL)…
- **Glitch machine** : traînées lumineuses directionnelles + lignes de contour qui « coulent ».
- **Matrice pop** : LED carrées tramées sur palette acide.
- **Animation de photo** : un clic sur 〰 et la photo s'anime en boucle parfaite → export **GIF** ou **vidéo MP4/WebM**.
- **Vidéos et caméra** : les effets s'appliquent en direct, export de la vidéo stylisée.
- Déformations (tourbillon, sphère, ondulation…), flous (tilt‑shift, mouvement…), bloom, scanlines, aberration chromatique, glitch, grain, vignette.
- **41 recettes** en un clic, comparaison avant/après, export PNG jusqu'à ×4, texte ASCII copiable.

## Confidentialité

Tout tient dans un seul fichier, [`src/index.html`](src/index.html), sans aucune bibliothèque externe.

- Une **Content‑Security‑Policy** interdit au navigateur toute connexion vers Internet (`connect-src` n'autorise que le canal local `ipc` utilisé par l'application de bureau).
- Les images exportées ne contiennent **plus de métadonnées** (position GPS, modèle d'appareil…).
- Le code fait ~1 700 lignes commentées en français : tu peux tout vérifier.

## Installer

Télécharge la dernière version dans **[Releases](../../releases/latest)**.

| Système | Fichier | Premier lancement |
|---|---|---|
| **Windows** | `ASCII Local_x.x.x_x64-setup.exe` | Si SmartScreen s'affiche : *Informations complémentaires* › *Exécuter quand même*. |
| **Mac M1/M2/M3/M4** | `ASCII Local_x.x.x_aarch64.dmg` | Glisse l'appli dans *Applications*, puis **clic droit › Ouvrir** la première fois. Si macOS dit que l'appli est endommagée : `xattr -cr "/Applications/ASCII Local.app"` dans le Terminal. |
| **Mac Intel** | `ASCII Local_x.x.x_x64.dmg` | Pareil. |
| **Android** | `ASCII-Local.apk` | Ouvre le fichier sur ton téléphone et autorise l'installation d'applications inconnues. Les exports vont dans *Documents › ASCII Local*. |
| **iPhone / web** | lien GitHub Pages | Ouvre le lien dans Safari › Partager › *Sur l'écran d'accueil*. |

> Les avertissements Windows/macOS sont normaux : l'appli n'est pas signée avec un certificat payant (Apple : 99 $/an, Windows : ~200 €/an).

## Publier une nouvelle version

Les installateurs sont construits automatiquement par **GitHub Actions** :

```bash
git tag v1.0.0
git push origin v1.0.0
```

En ~15 minutes, une Release apparaît avec les fichiers Windows, Mac et Android.
Sans tag, tu peux aussi lancer le workflow à la main (onglet *Actions* › *Build & Release* › *Run workflow*) : les fichiers sont alors dans *Artifacts*.

**Version web** : *Settings* › *Pages* › Source : **GitHub Actions** (une seule fois). Elle sera en ligne sur `https://<ton-pseudo>.github.io/ascii-local/`.

## Développer

L'application elle‑même ne demande aucune installation : ouvre `src/index.html` dans Chrome, Edge ou Firefox.

Pour les applis natives :

```bash
npm install
npm run dev              # appli de bureau (Tauri 2) — nécessite Rust : https://rustup.rs
npm run build:desktop    # installateur pour ton système

npm run android:init     # crée le projet Android (Capacitor) — nécessite Android Studio
npm run android:sync     # copie src/ dans le projet Android
npm run android:open     # ouvre Android Studio pour lancer sur ton téléphone
```

### Structure

```
src/index.html                 l'application complète (HTML + CSS + JS)
src-tauri/                     emballage bureau (Windows, macOS, Linux) avec Tauri 2
capacitor.config.json          emballage Android avec Capacitor
.github/workflows/release.yml  compilation automatique + Release
.github/workflows/pages.yml    version web sur GitHub Pages
```

### Ajouter un style

1. Écris une fonction `monStyle(o, W, H, sc)` qui dessine dans le canvas `o` (tu peux lire l'image source via `sample(cols, rows)` ou `SRC`).
2. Ajoute‑la dans l'objet `STYLES`, puis dans `STYLE_GROUPS` pour qu'elle apparaisse dans la galerie.
3. Pour qu'elle bouge en mode animation, utilise `ANIM.phase` (de 0 à 1 sur une boucle) quand `ANIM.on` est vrai.

## Licence

[MIT](LICENSE) — utilise, modifie et redistribue librement.

Inspiré par l'outil en ligne ascii-magic.com et par le travail de l'artiste decayingsun ; aucun code ni ressource de ces projets n'est utilisé.
