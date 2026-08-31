---
type: SpecBind Roadmap
---

# ロードマップ

<!-- specbind:instruction create
マイルストーン全体で共有する変更要求と判断理由を記録する。CLIが所有するwork item一覧を
本文へ繰り返したり、本文をライフサイクルstatusとして扱ったりしない。現在のマイルストーンを
明確にできるなら、節は削除、改名、追加してよい。
-->

<!-- specbind:instruction maintain
現在有効なマイルストーン全体の要求と理由を保つ。scopeや順序が変わった場合は説明をその場で
更新する。履歴はGitとリリース済みRoadmapアーカイブが所有する。
-->

<!-- specbind:instruction consume
work itemと依存関係についてはFront Matterが権威を持つ。本文は要求、境界、分解、判断理由を
説明する。
-->

## マイルストーン全体の変更要求

<!-- specbind:instruction maintain
このマイルストーンを1つのdeliveryとして扱う理由と、複数のwork itemに共通する要求を記載する。
各SpecのRequirementsやFront Matterのwork item一覧を複製しない。
-->

## 望む結果

<!-- specbind:instruction maintain
マイルストーン全体が完了したとき、利用者またはプロジェクトに成立している結果を記載する。
個々のwork itemの完了条件を列挙しない。
-->

## アプローチと分解判断

<!-- specbind:instruction maintain
依頼を複数のSpecまたはDirect work itemへ分けた境界と、その分解で一緒に届ける理由を記載する。
分解に追加説明が不要な単一work itemの場合はこの節を削除する。
-->

## スコープの境界

<!-- specbind:instruction maintain
このdeliveryに含めるものと、隣接するが含めないものをマイルストーン全体の粒度で記載する。
各Spec内の詳細な責任境界はRequirementsに置く。
-->

## 依存関係と順序の理由

<!-- specbind:instruction maintain
Front Matterが示す依存関係や順序のうち、自明でない理由、共有する前提、並行化できない境界を
説明する。依存がなく順序にも判断がない場合はこの節を削除する。
-->

## 制約と未解決事項

<!-- specbind:instruction maintain
マイルストーン全体に効く期限、外部条件、未確定の判断と、それが次の作業を止める条件を記載する。
合致する制約や未解決事項がない場合はこの節を削除する。
-->

## Source Collectionと振り分け

<!-- specbind:instruction maintain
Discoveryが明示的なSource Collectionを使った場合、provider、プロジェクト相対の
collection locator、全項目のdisposition、関係するwork item、振り分け理由を保つ。
コレクションが無い場合はこの節を省略する。
-->
