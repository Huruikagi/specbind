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

### 目標

### 非目標

## アーキテクチャと境界

## システムフロー

## コンポーネントとインターフェース

## データモデル

## エラー処理

## テスト方針

## 移行と展開

## リスクと代替案
