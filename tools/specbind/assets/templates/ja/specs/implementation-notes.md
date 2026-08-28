---
type: SpecBind Implementation Notes
artifact_id: main
---

<!-- specbind:instruction create bind=spec
`spec`を現在のauthoring contextにある正規のSpec identityとして解決する。
すべての`{{spec}}`参照をその同じ値で置換し、ディレクトリ外で読んでも対象を
識別できるようタイトルに残す。
-->

<!-- specbind:instruction create bind=artifact_id
`artifact_id`をこのテンプレートのFront Matterにあるリテラルなcollection identityとして
解決する。すべての`{{artifact_id}}`参照をその同じ値で置換し、分けて保持する実装メモを
区別できるようタイトルに残す。
-->

# `{{spec}}` の実装メモ — `{{artifact_id}}`

<!-- specbind:instruction maintain
次にこの変更を実装する担当者のための、永続的な自由記述メモ。
却下した方針、環境固有の癖、Requirements・Design・Contract が保持していない判断など、
自明でないことを記録する。
-->

<!-- specbind:instruction consume
これは実装上の記憶であり、仕様の権威ではない。この文書が承認ゲートの入力になることはない。
-->
