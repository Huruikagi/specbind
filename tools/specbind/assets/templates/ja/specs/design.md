---
type: SpecBind Design
artifact_id: main
---

<!-- specbind:instruction create bind=spec
`spec`を現在のauthoring contextにある正規のSpec identityとして解決する。
すべての`{{spec}}`参照をその同じ値で置換し、ディレクトリ外で読んでも対象を
識別できるようタイトルに残す。
-->

<!-- specbind:instruction create bind=artifact_id
`artifact_id`をこのテンプレートのFront Matterにあるリテラルなcollection identityとして
解決する。すべての`{{artifact_id}}`参照をその同じ値で置換し、分割した設計文書を
区別できるようタイトルに残す。
-->

# `{{spec}}` の設計 — `{{artifact_id}}`

<!-- specbind:instruction maintain
この文書が扱う Requirement ID をすべて列挙した `requirement_ids` 配列を Front Matter に
追加し、同じ ID を `_Requirements: 1.1, 1.2_` という厳密な形式のイタリック本文マーカーとして、
それを満たす節の近くに記載する。Front Matter の集合と本文マーカーの和は完全に一致する必要がある。

大きな変更は、それぞれに `artifact_id` とファイルを与えて複数の設計文書に分割する。
この文書が所有する判断だけを記述する。Research は判断の材料にできるが、権威ある判断と根拠は
この文書だけで理解できるようにする。該当しない節は削除し、図や表は複雑な関係を明確にできる
場合にだけ使う。

内部アーキテクチャと、永続的な Contract をこの設計がどう実現するかを記述するが、Contract の
標準的な接合面一覧は複製しない。異なる実装者でも互換性のある結果に到達できるよう、具体的な
ファイル境界、インターフェース、失敗時の振る舞い、検証方針を十分に残す。

見出し `_Requirements: ...` の形式は機械可読であり、日本語化しない。
-->

## 概要

<!-- specbind:instruction maintain
この文書が所有する設計判断と、Requirementsを実現する中心的な方針を短くまとめる。後続節の
詳細を繰り返さない。
-->

### 目標

<!-- specbind:instruction maintain
この設計によって成立させる技術的な結果を記載する。Requirementsを言い換えず、実現方針として
何を達成するかを示す。
-->

### 非目標

<!-- specbind:instruction maintain
設計対象に見えるが意図的に扱わない技術的責任だけを記載する。意味のある非目標がない場合は
この小節を削除する。
-->

## アーキテクチャと境界

<!-- specbind:instruction maintain
変更後の責任分担、依存方向、所有境界と、その形を選んだ理由を記載する。既存構造を一覧として
複製せず、この変更の判断に必要な差を示す。
-->

## システムフロー

<!-- specbind:instruction maintain
複数の境界をまたぐ処理、状態遷移、非同期のやり取りを順序と失敗経路が分かる形で記載する。
単一コンポーネント内で自明な処理しかない場合はこの節を削除する。
-->

## コンポーネントとインターフェース

<!-- specbind:instruction maintain
新設または変更するコンポーネントごとに責任、入力、出力、保証、呼び出してよい相手を記載する。
言語固有のシグネチャは、その形自体が互換性の判断である場合だけ含める。
-->

## データモデル

<!-- specbind:instruction maintain
新設または変更する概念、永続化形、整合性境界、移行可能性を記載する。データの形や所有に変更が
ない場合はこの節を削除する。
-->

## エラー処理

<!-- specbind:instruction maintain
境界ごとの失敗、利用者または呼び出し元に返す結果、再試行、縮退、観測方法を記載する。既存の
方針をそのまま適用でき、追加の判断がない場合はこの節を削除する。
-->

## 検証方針

<!-- specbind:instruction maintain
各重要な保証と失敗経路を、どの境界で何を観測して検証するか記載する。テスト種別の一般論や
既存コマンドの一覧は書かない。
-->

## 移行と展開

<!-- specbind:instruction maintain
既存データ、利用者、呼び出し元を新しい設計へ安全に移す順序、互換期間、切り戻し条件を記載する。
一段階で安全に置き換えられ、移行判断がない場合はこの節を削除する。
-->

## リスクと代替案

<!-- specbind:instruction maintain
採用案に残る具体的なリスクと、検討した実行可能な代替案を採用しなかった理由を記載する。
判断に影響するリスクや代替案がない場合はこの節を削除する。
-->
