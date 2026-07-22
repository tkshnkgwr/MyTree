# System Diagrams (DIAGRAM.md) - MyTree

**English** | [日本語版](../ja/DIAGRAM.md)

This document visualizes the execution flow and component structure of `MyTree` using Mermaid diagrams.

---

## 1. Execution Flowchart

```mermaid
flowchart TD
    Start([Start: main]) --> EnableANSI[enable_ansi_support<br/>Enable Windows ANSI escape]
    EnableANSI --> ParseArgs[parse_arguments<br/>Parse CLI arguments]
    
    ParseArgs -->|Arg error| ErrExit[Print error message & exit]
    ParseArgs -->|--help| HelpExit[Print help & exit]
    ParseArgs -->|Success| CheckPath{Check target path existence}
    
    CheckPath -->|Does not exist| PathErrExit[Path not found error & exit]
    CheckPath -->|Exists| CheckFormat{output_format}
    
    CheckFormat -->|Text| PrintRoot[Print root directory line]
    PrintRoot --> PrintTree[print_tree<br/>Start recursive tree rendering]
    
    subgraph TreeRecursion [Tree Recursion: print_tree]
        FilterEntries[Read entries & filter<br/>- Exclude hidden files<br/>- Exclude match patterns]
        SortEntries[Directory-first sorting]
        LoopEntries[Iterate entries one by one]
        RenderLine[Render indent + branch + icon + name + size]
        IsSubDir{Is directory?}
        RecurseSub[Recursively call print_tree]
        
        FilterEntries --> SortEntries
        SortEntries --> LoopEntries
        SortEntries --> RenderLine
        RenderLine --> IsSubDir
        IsSubDir -->|Yes| RecurseSub
        IsSubDir -->|No| NextEntry[To next entry]
        RecurseSub --> NextEntry
    end
    
    PrintTree --> TreeRecursion
    TreeRecursion --> PrintSummary[Print final summary<br/>X directories, Y files]
    
    CheckFormat -->|Json| BuildTreeJson[build_tree_node & serialize_json<br/>Output JSON object]
    CheckFormat -->|Xml| BuildTreeXml[build_tree_node & serialize_xml<br/>Output XML tree]

    PrintSummary --> End([End: 0])
    BuildTreeJson --> End
    BuildTreeXml --> End
```

---

## 2. Component Inter-Data Flow Diagram

```mermaid
sequenceDiagram
    autonumber
    participant CLI as User (CLI)
    participant Main as main() (main.rs)
    participant Config as config.rs (parse_arguments)
    participant Tree as tree.rs (print_tree)
    participant Serializer as serializer.rs (build_tree_node)
    participant Util as util.rs (match_wildcard / format_byte_size)
    participant Icon as icon.rs (get_icon_emoji)

    CLI->>Main: Execute command (e.g. MyTree --json -d 2)
    Main->>Main: enable_ansi_support() (Windows API)
    Main->>Config: parse_arguments(args, &mut config)
    Config-->>Main: Ok(()) / TreeConfig object
    
    alt Text Output (OutputFormat::Text)
        Main->>Tree: print_tree(target_path, depth, prefixes, config, ...)
        loop Recursive processing per level
            Tree->>Util: match_wildcard(pattern, file_name)
            Tree->>Icon: get_icon_emoji(file_name, is_dir)
            Tree->>Util: format_byte_size(size)
            Tree->>CLI: Render tree line to stdout
        end
    else JSON / XML Output (OutputFormat::Json / Xml)
        Main->>Serializer: build_tree_node(target_path, depth, config, ...)
        Serializer->>Util: match_wildcard(pattern, file_name)
        Serializer-->>Main: TreeNode struct
        Main->>Serializer: serialize_json() / serialize_xml()
        Serializer->>CLI: Render structured data to stdout
    end
```

---

## 3. GitHub Actions CI/CD Workflow Diagram

```mermaid
flowchart TD
    subgraph Trigger [Trigger Detection]
        PushMain[push / PR to main]
        PushTag[push tag: v*]
    end

    subgraph CI_Pipeline [CI Workflow: ci.yml]
        PushMain --> CheckoutCI[actions/checkout]
        CheckoutCI --> InstallRustCI[dtolnay/rust-toolchain]
        InstallRustCI --> CacheCI[Swatinem/rust-cache]
        CacheCI --> Test[cargo test]
        Test --> Clippy[cargo clippy]
        Clippy --> Fmt[cargo fmt --check]
        Fmt --> Doc[cargo doc]
        Doc --> BuildCI[cargo build --release]
    end

    subgraph Release_Pipeline [Release Workflow: release.yml]
        PushTag --> CheckoutRel[actions/checkout]
        CheckoutRel --> InstallRustRel[dtolnay/rust-toolchain]
        InstallRustRel --> BuildRel[cargo build --release]
        BuildRel --> Zip[Compress-Archive -> MyTree-windows-x64.zip]
        Zip --> GHRelease[softprops/action-gh-release<br/>Create GitHub Release & attach asset]
    end
```
