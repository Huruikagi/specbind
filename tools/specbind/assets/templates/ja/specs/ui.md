---
type: SpecBind Design
artifact_id: ui
---

<!-- specbind:instruction create bind=spec
`spec`を現在のauthoring contextにある正規のSpec identityとして解決する。
すべての`{{spec}}`参照をその同じ値で置換し、ディレクトリ外で読んでも対象を
識別できるようタイトルに残す。
-->

<!-- specbind:instruction create bind=artifact_id
`artifact_id`をこのテンプレートのFront Matterにあるリテラルなcollection identityとして
解決し、すべての`{{artifact_id}}`参照をその同じ値で置換する。
-->

# `{{spec}}` の設計 — `{{artifact_id}}`

<!-- specbind:instruction maintain
プロジェクトの選択ルールが、このSpecの現在の責任にこの文書を適用すると判定する間だけ
保持する。この文書が扱うすべてのRequirementをFront Matterの`requirement_ids`配列に
列挙し、それを実現する判断の近くに同じIDを厳密な`_Requirements: 1.1, 1.2_`形式で記載する。

ピクセル単位の完成図ではなく、ユーザーに見える振る舞いと状態を設計する。該当しない節は
削除する。読者と保守担当者が独立して追う永続的なUI責任がある場合にだけ、別のDesign
identityへ分割する。
-->

## ユーザーと利用状況

## 画面一覧

## ナビゲーションと操作フロー

## 画面の振る舞い

### 主要な情報と操作

### 読み込み中、空、エラー、利用不能状態

### 入力と検証フィードバック

## レスポンシブ動作

## アクセシビリティ

## コンポーネント、データ、サービスの境界

## UI検証方針
