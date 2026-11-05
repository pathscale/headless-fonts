# Embedded font inventory

SHA-256 values identify the committed font bytes. Upstream release provenance is not established by these hashes.

| File under assets/fonts | SHA-256 | Upstream URL and release |
| --- | --- | --- |
| Gelasio-Italic-VariableWeight.ttf | 52559e845a4d33514e5f93bb9ae7dbeae1894a53f2c565a15f18af40cd337c09 | Unverified; owner evidence required |
| Gelasio-VariableWeight.ttf | 4daecea457258c9ebeb8bc99ed3fd24353618bfad3ea4b93fa0b5d0468fc04e4 | Unverified; owner evidence required |
| NotoMono-Regular.ttf | 41fd7ccc82375e2a1e47f0cc2c941c14d7c99ba2f57cf69c9f738d07fb257686 | Unverified; owner evidence required |
| NotoSans-Bold.ttf | 7c15ac396d2ce6bc33a3b4efacdbd322c9e46376599a725c6f790d8036052cab | Unverified; owner evidence required |
| NotoSans-Italic.ttf | 35600501daadd3e9030ee6ed8098fd3c7f31bfd898ca58da8edf793a3231c48b | Unverified; owner evidence required |
| NotoSans-Regular.ttf | 6b04c8dd65af6b73eb4279472ed1580b29102d6496a377340e80a40cdb3b22c9 | Unverified; owner evidence required |
| NotoSansDevanagari-Regular.ttf | 84cda635b5395e22f428c5802b6c36eaeec66750c8b387279424a48545ac1d74 | Unverified; owner evidence required |

These seven files are embedded by src/lib.rs FACES. Gelasio uses
assets/fonts/Gelasio-LICENSE-OFL.txt; the Noto files use
assets/fonts/Noto-LICENSE-OFL.txt. Preserve those notices in distributions.

Before replacing a font, record its upstream repository or release download URL,
immutable revision or release identifier, license record and independently verified
SHA-256. Reconcile the existing files against that evidence before describing this
inventory as upstream provenance. No parser robustness or authenticity is implied
by a local digest.
