# CLIコマンド一覧

普段はスキルを通して作業し、CLIはスキルが実行します。このページでは、状態を
自分で確認したいときや、スキルが報告したコマンドの意味を知りたいときのために、
`specbind`のコマンドを用途別にまとめます。正確な引数とオプションは
`specbind <command> --help`で確認してください。

## 状態を確認する（読み取り専用）

いつ実行しても安全で、プロジェクトのファイルは変更しません。

| コマンド | 表示する内容 |
| --- | --- |
| `specbind milestone status` | 進行中のMilestoneの段階、進捗、次の操作、リリースを止めているもの |
| `specbind milestone replan capture` | 再計画前のクリーンなコミットと、最新の完了承認があるSpecをJSONで表示 |
| `specbind milestone replan check --baseline <commit> --recovery-revision <commit>` | 再計画後に完了承認の取り下げが必要なSpecと、確認済みの取り下げコミットをJSONで表示 |
| `specbind spec list` | プロジェクトのすべてのSpec |
| `specbind spec status <spec>` | 1つのSpecの状態、鮮度、網羅状況、Taskの進捗 |
| `specbind spec completion batch preflight --spec <id> --spec <id>` | 明示した完了承認の対象と、変更のない共通の実装コミット。JSONで出力 |
| `specbind spec completion batch verify --evidence <file\|->` | 指定した全件の完了承認が、証拠の鮮度を保った1つのコミットにまとまっていること |
| `specbind tasks list <spec>` / `tasks show <spec> <task>` | Taskの一覧と進捗、または1つのTaskの詳細 |
| `specbind tasks revision preflight <spec>` | 既存計画の改訂に使うフィンガープリントと、現在の計画・進捗をJSONで表示 |
| `specbind artifact list <spec>` / `artifact read <spec> <selector>` | Specの成果物の一覧、または1つの成果物の内容 |
| `specbind steering list` / `steering read <selector>` | Steering文書 |
| `specbind configuration show` | プロジェクト設定全体の要約 |
| `specbind release preflight` | リリースを止めているもの |
| `specbind check traceability <spec>` | DesignとTasksによるRequirementsの網羅状況 |
| `specbind check contracts` | プロジェクト全体のContractのエラー |
| `specbind check contracts --for-design <spec>` | 対象SpecのContractと直接の依存先の検証結果。全体の整合性は検証しない |
| `specbind contract graph` / `dependencies` / `consumers` / `owners` / `shared` | Contractの関係とファイルの所有 |

`milestone status`と`spec status`は、ツールやスクリプト向けに`--json`も受け付けます。
表示される状態名は[状態の一覧](./lifecycle-states.md)を参照してください。

Taskに着手するには、グループをまたいだ先行Taskもすべて完了している必要があります。
引き継いだ完了記録が途中にあっても、それより前の未着手・ブロック中の作業は飛ばせません。
`tasks complete`も同じ条件を確認し、証明済みの進捗は維持します。

個別のDesignは、関係のないSpecが設計中でも検証を通せます。ただし、直接使う
相手のContractが存在し、参照する項目を確認できることが必要です。Contract Reviewでは
引き続き全体を検証します。別のSpecの設計や全体レビューで矛盾が見つかれば、
一度承認したDesignでも戻って修正することがあります。

## プロジェクトの設定と埋め込みの内容を読む（読み取り専用）

| コマンド | 表示する内容 |
| --- | --- |
| `specbind template list` / `read` / `resolve` | 成果物のテンプレートと、新しい成果物の配置先 |
| `specbind rule list` / `read` | プロジェクトの共有ルール |
| `specbind adapter list` / `read` | プロジェクトの運用アダプター |
| `specbind protocol list` / `read` | バイナリに埋め込まれた製品プロトコル |
| `specbind schema list` / `read` | バイナリに埋め込まれた構造化成果物のスキーマ |

`specbind schema read drive/v1`は、`.specbind.json`全体ではなく、`drive`項目のスキーマを
返します。`configuration show`ではDriveの既定値と、設定値か従来の既定値かを確認できます。

使い方は[カスタマイズ](../guide/customization.md)を参照してください。

## インストール・更新・削除

自分で実行するコマンドです。どれもファイルを変更する前に計画を表示します。

| コマンド | 用途 | ガイド |
| --- | --- | --- |
| `specbind install` | プロジェクトへの導入または更新（`--dry-run`で事前確認） | [インストール](../guide/install.md)、[アップデート](../guide/update.md) |
| `specbind migration plan --from <version>` | バイナリ更新後の移行作業を表示 | [アップデート](../guide/update.md) |
| `specbind remove-agent <agent>` | 1つのエージェント連携を外す（`--apply`で適用） | [削除とアンインストール](../guide/uninstall.md) |
| `specbind uninstall --knowledge retain\|remove` | プロジェクトの連携全体を外す（`--apply`で適用） | [削除とアンインストール](../guide/uninstall.md) |
| `specbind migrate cc-sdd` | cc-sddからの移行の計画・適用 | [cc-sddから移行](../guide/migrate-from-cc-sdd.md) |
| `specbind feedback` | バグ報告と改善提案の窓口を表示 | [バグ報告と改善提案](../guide/feedback.md) |

## ライフサイクルの状態を変える（スキルが使う）

承認、進捗、リリースを記録するコマンドです。スキルが必要なレビューと確認を
済ませてから実行するので、通常は手で実行しません。

| コマンド | 記録する内容 |
| --- | --- |
| `specbind milestone create` / `update-scope` / `scope` / `rebaseline` | Milestoneの作成とスコープの変更（`sb-discovery`） |
| `specbind spec requirements` / `design` / `tasks` | Gateの承認と無効化（`sb-plan`） |
| `specbind milestone review` | Milestone全体のContractレビュー（`sb-contract-review`） |
| `specbind tasks complete` / `block` / `reopen` | Taskの進捗（`sb-implement`） |
| `specbind tasks revision apply <spec> --input <file\|->` | 既存計画の置き換えと、明示した対応表による進捗の引き継ぎ（`sb-plan`。`--dry-run`で書き込まずに検査） |
| `specbind milestone direct` | Direct項目の完了（`sb-implement`） |
| `specbind spec completion` | Specの完了の証拠（`sb-validate-implementation`） |
| `specbind spec completion batch accept --evidence <file\|->` | 明示した複数Specについて、同じ実装コミットでの完了の証拠 |
| `specbind milestone bind-release <version>` | 対象リリースの紐付け（`sb-release`。前もって自分で紐付けてもよい） |
| `specbind release finalize` | Milestoneのリリース確定処理（`sb-release`） |
| `specbind adoption preflight` / `milestone reverse` | 既存実装からのSpecの確立（`sb-adopt`） |
| `specbind artifact check` / `steering check` | 作成中の新しい成果物の確認 |

!!! warning
    これらのコマンドを直接実行すると、担当スキルが行うレビューを飛ばすことに
    なります。手で実行するのは、ガイドで指示された場合だけにしてください。
    たとえば[既存実装からSpecを確立する](../guide/adopt-existing.md)の
    `specbind milestone reverse abandon`がこれに当たります。

## 進捗があるTask計画を改訂する

Tasksの巻き戻しが許可されて実行された後、`sb-plan`は`tasks revision preflight`と
`schema read task-revision/v1`を使って改訂候補を作ります。どちらの改訂コマンドも、
`tasks`状態で、Requirements・Designの承認とContract Reviewが最新であることを
確認します。旧計画が現在のRequirementsの範囲と合わなくなっていても、修正の材料として読めます。

入力は厳密なJSONです。`schemaVersion: 1`、取得した`sourceFingerprint`、Tasks v1形式の
完全な`plan`、進捗の対応表`progress`を指定します。記録済みの各Taskには、
`from`・`to`・`reason`を持つ`preserve`（維持）または`reset`（未着手へ戻す）、あるいは
`from`・`reason`を持つ`remove`（削除）のいずれかを1つ指定します。
進捗がなければ`progress: []`にします。移動先は実行可能なTaskで、重複できません。
入力に`execution`や新しいブロック理由は含めません。

元の作業義務と証拠が引き続き有効かはスキルが判断し、既存のDrive再計画権限で
認められている場合を除いて、対応表の確認を求めます。CLIは維持する記録をそのままコピーし、
リセットした作業と新しい作業を未着手にし、削除対象の記録を除きます。
対応漏れ・重複、改訂後のRequirementsの網羅不足、改訂元の変更は拒否します。
フィンガープリントが変わったときは、値だけを差し替えず、読み直して対応表を再検討します。

入力は標準入力（`--input -`）か、作業ツリー外の一時ファイルから渡します。
`--dry-run`は書き込まずに対応表と改訂結果を返します。適用時は`tasks.yaml`だけを
一括で置き換えます。Gate承認とGitのチェックポイントは別の操作です。
初回は従来どおり進捗のない計画を作成し、以降の改訂は進捗の有無にかかわらずこのCLIを使います。

## 複数Specの完了承認をまとめる

一括事前確認の後、`sb-validate-implementation`が必要な検査を実行し、各Specの要件を
満たすか個別に評価します。全件が`GO`の場合だけ承認できます。
CLIは検査を実行せず、承認権限も与えません。

`schema read completion-batch/v1`で、一時的な入力JSONの厳密な形式を確認できます。
入力には`schemaVersion: 1`、共通の`implementationRevision`、1件以上の`targets`を
指定します。各対象には重複しない`spec` IDと、1件以上の`mechanicalChecks`が必要です。
各検査には`kind`、実際に実行したとおりの`command`、`exitCode: 0`を指定し、必要なら
プロジェクト相対の`workingDirectory`を加えます。標準入力かプロジェクト外のファイルで渡します。

一括承認は、最初の書き込み前に入力とライフサイクルの条件を全件検査します。
各書き込みの直前にも、固定したHEADと書き込み済みの変更を確認します。
JSONの結果には`accepted`、`pending`、`paths`、`issues`が含まれます。
通常の前提条件違反は`COMPLETION_BATCH_REJECTED`となり、何も書き込みません。
`COMPLETION_BATCH_INTERRUPTED`では、途中まで承認済みの対象が残ることがあります。
ファイルごとの置き換えは不可分ですが、全件をまとめたトランザクションではありません。
失敗時は停止し、自動で巻き戻す、再試行する、一部だけコミットすることは避けます。
入力不正や読み取り専用の事前確認・コミット検証の失敗は、通常のCLIエラーとして報告します。
プロセスが突然終了すると結果が残らない場合もあるため、回復操作の前に現在の状態を調べます。

`COMPLETION_BATCH_ACCEPTED`の後はGitアダプターに従い、返されたメタデータのパスだけを
1つのコミットにまとめます。同じ入力を`batch verify`に渡し、`COMPLETION_BATCH_VERIFIED`を
確認します。これは実装コミットの直後に、親が1つの完了承認コミットがあり、作業ツリーに
変更がなく、前段Gateが変わらず、全件の完了の証拠が最新であることを検証します。
余計なファイルやメタデータの変更、分割したコミットは拒否します。
アダプターがコミットを禁じている場合は、未コミットの対象を報告し、コミット検証を省略します。
HEADや作業ツリーが変わった場合は、その作業を解決して新しい検証を始めます。
入力のコミットIDだけを差し替えても、証拠を取り直したことにはなりません。

## 再計画後の完了承認の回復を確認する

Driveは許可された再計画の前に、`milestone replan capture`が返した完全な
`baselineRevision`を保持します。Plan完了時のコミットを`recoveryRevision`として固定し、
取り下げの前と取り下げコミットの後に`milestone replan check`を実行します。
CLIは一時的なローカルリポジトリで過去の状態を再検証し、対象全件の条件と、
順番どおりに完了承認だけを取り下げたコミットを確認します。
作業ツリーがクリーンでない、履歴が不足している、前段Gateが古い、スコープや
Requirementsの承認が変わった、無関係なコミットが挟まった場合は拒否します。

結果には残りの対象`remaining`、確認済みの取り下げ`withdrawn`、再計画で変更した
パス`recoveryChangedPaths`が含まれます。変更が許可された修正の範囲内かは、
Driveと検証担当が元の権限と実際の差分から判断します。検査を通すために基準の
コミットを進めてはいけません。取り下げとそのコミットは従来どおり検証担当が行い、
新しいコマンド自体はプロジェクトのファイルや承認状態を変更しません。
