# typst-diff-tool

##### Autorzy:
- Krzysztof Fijałkowski
- Rafał Szczepaniak

##### Prowadzący:
- Łukasz Neumann

#### Zamysł projektu:
Narzędzie CLI do porównywania wersji dokumentów Typst

### Czym jest Typst
Markup-based język do tworzenia eleganckich dokumentów. Podobny do latexa ale bardziej user friendly i interaktywne edytowalny, można go opisać w następujących punktach:

- Wbudowane znaczniki do najczęstszych zadań formatowania
- Elastyczne funkcje do wszystkiego innego
- Ściśle zintegrowany system skryptowy
- Skład matematyczny, zarządzanie bibliografią i więcej
- Szybkie czasy kompilacji dzięki kompilacji przyrostowej
- Przyjazne komunikaty o błędach na wypadek problemów

### Opis problemu
W odróżnieniu od np. Latex nie ma narzędzia do porównywania wytworzonych plików co utrudnia recenzję nowych wersji dokumentacji.

### Pod-problemy i ogólne rozwiązania
- Parsowanie pliku .typ na drzewo AST i zrozumienie w jaki sposób są agregowane elementy
- Parsowanie drzewa AST na np. JSONa z użyciem pandoc 
- Porównanie drzew AST w formacie JSON - znalezienie sposobu na wygodne porównanie plików
- Wyświetlenie różnic (potencjalnie przy użyciu bibliotek do porównywania dużych fragmentów tekstu), przeparsowanie tych różnic na odpowiednie miejsca w drzewie AST
- Stworzenie wynikowego pliku typu .typ.
- Napisanie aplikacji terminalowej w c++  , np rozwinięcie o użycie bibliotek np. NCurses

### Planowane narzędzia i technologie
- Wyciąganie AST z pliku za pomocą typst-syntax
- C4 w celu zilustrowania architektury
- c++23
- CMake
- Github workers
- Lintery i statyczna analiza kodu - clang
- Flagi przy kompilacji które wymagają brak errorów ani warningów i kładą nacisk na bezpieczeństwo

### Przydatne linki:

[Typst](https://typst.app/) -- online edytor

[Typst github](https://github.com/typst/typst) -- repozytorium

[Typst crates](https://crates.io/crates/typst-syntax) -- crates.io types syntax pack

[Typst docs](https://typst.app/docs/reference/visualize/color/) -- dokumentacja (tutaj użycie kolorów)

[github content struct](https://github.com/typst/typst/blob/main/crates/typst/src/foundations/content.rs#L75) - content structure Typst

[ast in json](https://esdiscuss.org/topic/ast-in-json-format) - AST in JSON

[installing pandoc](https://pandoc.org/installing.htm) - In order to run proj install pandoc v. 3.1.13
