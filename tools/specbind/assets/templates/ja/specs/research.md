---
type: SpecBind Research
---

<!-- specbind:instruction create bind=spec
`spec`を現在のauthoring contextにある正規のSpec identityとして解決する。
すべての`{{spec}}`参照をその同じ値で置換し、ディレクトリ外で読んでも対象を
識別できるようタイトルに残す。
-->

# `{{spec}}` のリサーチ

<!-- specbind:instruction create
Requirements や Design を実際に止めている未解決の問いがある場合にだけ作成する。
通常の変更にリサーチ文書は不要。
-->

<!-- specbind:instruction maintain
追記型の活動ログではなく、現在のマイルストーンで有効な入力として維持する。
調査源、調査結果、選択肢、それぞれの調査が支える判断を記録する。リリース後も必要な結論は
Requirements、Design、Contract のいずれかへ移し、当てはまらない節は削除する。
-->

<!-- specbind:instruction consume
これはマイルストーン中だけの補助的な根拠であり、永続的な権威ではない。
Requirements、Design、Contract は、この文書なしで理解できなければならない。
-->

## 要約

<!-- specbind:instruction maintain
調査によって分かったこと、まだ分からないこと、RequirementsまたはDesignが次に判断できることを
短くまとめる。詳細な根拠は各「調査項目」に置く。
-->

## 調査項目

<!-- specbind:instruction maintain
現在の判断に必要な問いごとに、次の小節一式を繰り返す。時系列の作業記録にはせず、再調査で
結論が変わった場合は現在の結果へ更新する。
-->

<!-- specbind:instruction create
`<問い>`を実際の未解決の問いに置き換え、問いごとに小節一式を繰り返す。空の例示見出しは
live artifactに残さない。
-->

### `<問い>`

#### 背景

<!-- specbind:instruction maintain
この問いがRequirementsまたはDesignを止めている理由と、判断に必要な境界を記載する。
-->

#### 参照した情報源

<!-- specbind:instruction maintain
判断の根拠として実際に参照した資料を、再確認できるURL、文書名、またはプロジェクト相対pathで
記載する。情報源がない推測は調査結果として扱わない。
-->

#### 調査結果

<!-- specbind:instruction maintain
情報源から確認できた事実、制約、不確実性を区別して記載する。結論だけで根拠を省略しない。
-->

#### 変更への影響

<!-- specbind:instruction maintain
この結果がRequirements、Design、Contractのどの判断を可能にするか、または何を未解決のまま
残すかを記載する。影響が説明できない調査項目は削除する。
-->

## 選択肢とトレードオフ

<!-- specbind:instruction maintain
同じ問いに複数の実行可能な選択肢が残る場合だけ、それぞれが得るもの、失うもの、選択条件を
比較する。実行可能な選択肢が1つだけの場合はこの節を削除する。
-->

## リスクとフォローアップ

<!-- specbind:instruction maintain
調査後も残る不確実性、期限付きの仮定、追加確認が必要になる条件を記載する。合致するものが
ない場合はこの節を削除する。情報源は各調査項目に置き、ここへ重複させない。
-->
