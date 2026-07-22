# System Diagrams (DIAGRAM.md) - MyTree

[English](../en/DIAGRAM.md) | **日本語版**

本文書では、`MyTree` の実行フローおよびコンポーネント構造を Mermaid ダイアグラムで可視化します。

---

## 1. 実行フローチャート

```mermaid
flowchart TD
    Start([起動: main]) --> EnableANSI[enable_ansi_support<br/>Windows ANSIエスケープ有効化]
    EnableANSI --> ParseArgs[parse_arguments<br/>コマンドライン引数の解析]
    
    ParseArgs -->|引数エラー| ErrExit[エラーメッセージ出力 & 終了]
    ParseArgs -->|--help| HelpExit[ヘルプ出力 & 終了]
    ParseArgs -->|解析成功| CheckPath{ターゲットパスの存在確認}
    
    CheckPath -->|存在しない| PathErrExit[指定パス不存在エラー & 終了]
    CheckPath -->|存在| CheckFormat{output_format}
    
    CheckFormat -->|Text| PrintRoot[ルートディレクトリ行を出力]
    PrintRoot --> PrintTree[print_tree<br/>再帰ツリー描画を開始]
    
    subgraph TreeRecursion [ツリー再帰処理: print_tree]
        FilterEntries[エントリー読込 & フィルタ処理<br/>- 隠しファイル除外<br/>- 除外パターン合致除外]
        SortEntries[ディレクトリ優先ソート]
        LoopEntries[エントリーを1つずつ反復]
        RenderLine[インデント + 枝記号 + アイコン + 名前 + サイズ描画]
        IsSubDir{ディレクトリか？}
        RecurseSub[再帰呼び出し print_tree]
        
        FilterEntries --> SortEntries
        SortEntries --> LoopEntries
        SortEntries --> RenderLine
        RenderLine --> IsSubDir
        IsSubDir -->|Yes| RecurseSub
        IsSubDir -->|No| NextEntry[次のエントリーへ]
        RecurseSub --> NextEntry
    end
    
    PrintTree --> TreeRecursion
    TreeRecursion --> PrintSummary[最終集計を出力<br/>X directories, Y files]
    
    CheckFormat -->|Json| BuildTreeJson[build_tree_node & serialize_json<br/>JSONオブジェクト出力]
    CheckFormat -->|Xml| BuildTreeXml[build_tree_node & serialize_xml<br/>XMLツリー出力]

    PrintSummary --> End([終了: 0])
    BuildTreeJson --> End
    BuildTreeXml --> End
```

---

## 2. モジュール間データフロー図

```mermaid
sequenceDiagram
    autonumber
    participant CLI as ユーザー (CLI)
    participant Main as main() (main.rs)
    participant Config as config.rs (parse_arguments)
    participant Tree as tree.rs (print_tree)
    participant Serializer as serializer.rs (build_tree_node)
    participant Util as util.rs (match_wildcard / format_byte_size)
    participant Icon as icon.rs (get_icon_emoji)

    CLI->>Main: コマンド実行 (例: MyTree --json -d 2)
    Main->>Main: enable_ansi_support() (Windows API)
    Main->>Config: parse_arguments(args, &mut config)
    Config-->>Main: Ok(()) / TreeConfig オブジェクト
    
    alt テキスト出力 (OutputFormat::Text)
        Main->>Tree: print_tree(target_path, depth, prefixes, config, ...)
        loop 各階層の再帰処理
            Tree->>Util: match_wildcard(pattern, file_name)
            Tree->>Icon: get_icon_emoji(file_name, is_dir)
            Tree->>Util: format_byte_size(size)
            Tree->>CLI: 標準出力へのツリー行描画
        end
    else JSON / XML 出力 (OutputFormat::Json / Xml)
        Main->>Serializer: build_tree_node(target_path, depth, config, ...)
        Serializer->>Util: match_wildcard(pattern, file_name)
        Serializer-->>Main: TreeNode 構造体
        Main->>Serializer: serialize_json() / serialize_xml()
        Serializer->>CLI: 標準出力への構造化データ出力
    end
```

---

## 3. GitHub Actions CI/CD フロー図

```mermaid
flowchart TD
    subgraph Trigger [トリガー検出]
        PushMain[push / PR to main]
        PushTag[push tag: v*]
    end

    subgraph CI_Pipeline [CI ワークフロー: ci.yml]
        PushMain --> CheckoutCI[actions/checkout]
        CheckoutCI --> InstallRustCI[dtolnay/rust-toolchain]
        InstallRustCI --> CacheCI[Swatinem/rust-cache]
        CacheCI --> Test[cargo test]
        Test --> Clippy[cargo clippy]
        Clippy --> Fmt[cargo fmt --check]
        Fmt --> Doc[cargo doc]
        Doc --> BuildCI[cargo build --release]
    end

    subgraph Release_Pipeline [Release ワークフロー: release.yml]
        PushTag --> CheckoutRel[actions/checkout]
        CheckoutRel --> InstallRustRel[dtolnay/rust-toolchain]
        InstallRustRel --> BuildRel[cargo build --release]
        BuildRel --> Zip[Compress-Archive -> MyTree-windows-x64.zip]
        Zip --> GHRelease[softprops/action-gh-release<br/>GitHub Release 作成 & アセット添付]
    end
```
