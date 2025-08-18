# Xeno — A Thin, Portable Web Framework for Rust

**Inspired by Hono as web framework**

> **Tagline:** *Tiny core. Anywhere adapters.*  
> **Goal:** 同一の `Request → Response` コアを、**Cloudflare Workers** や **hyper アダプタ（任意のサーバー環境）**など**複数のホスティング先**に薄いアダプタで載せ替えられる、最小限で実用的な Web フレームワーク。

## 0. コンセプト

- **Minimal Dependencies**：`http`/`bytes` を土台に、`Request<Bytes> → Response<Bytes>` の**Single Path**。  
- **Universal Deployment**：配備先ごとに **Adapter**（hyper / workers-rs / そのほか）を差し替え。  
- **Intentionally Minimal**：ルーティング／抽出／レスポンス／ミドルウェアを**Essential Features Only**から段階拡張。  
- **Standard Contracts**：OpenAPI から **Type-Safe Client Generation**（tRPC 的 BFF は挟まない）。  
- **Common Core Logic**：ビジネスロジックは **Platform Agnostic**のコアで完結。周辺 I/O は `Ctx` 抽象で注入。

## 1. スコープ（MVP）

### MVPでやること
- **Routing**（GET/POST、パラメータ、ワイルドカード）
- **Extractor**（`Path<T>` / `Query<T>` / `Json<T>`）
- **Response**（`IntoResponse`：`Bytes`/`String`/`Json<T>`/`StatusCode`）
- **Middleware**（before/after 連鎖の軽量仕組み）
- **Error Handling**（`thiserror` + ステータスマッピング）
- **Adapter**: hyper（任意のサーバー環境向け）／ Cloudflare Workers（`workers-rs`）
- **OpenAPI 出力（最小）**＋フロント向け TS 型生成

### MVPでやらないこと（以降のステップで）
- SSE/ストリーミング、WebSocket
- tower 互換、圧縮/CORS など
- WASI-HTTP/Compute@Edge の公式アダプタ
- 監視（tracing/OTel）・認証

## 2. アーキテクチャ

```
xeno/
  core/             # 配備先非依存の中核
  adapters/
    hyper/          # tokio + hyper サーバ（任意のサーバー環境向け）
    workers/        # Cloudflare Workers（workers-rs, WASM）
  examples/
    hello-hyper/
    hello-workers/
  tools/
    openapi-gen/    # OpenAPI出力(任意: build.rs/CLI)
```

## 3. リクエスト・ライフサイクル（MVP）

1. アダプタが受信（hyper / workers-rs）
2. 変換して CoreRequest へ
3. ルータがハンドラ解決（Path params を req.extensions() に格納）
4. ミドルウェア before → ハンドラ → after
5. CoreResponse を配備先のレスポンスへ再変換・返却

## 4. 主要 API 断片（あくまで最小の形）

```rust
// core
pub type CoreRequest  = http::Request<bytes::Bytes>;
pub type CoreResponse = http::Response<bytes::Bytes>;

#[async_trait::async_trait]
pub trait Handler<C: Send + Sync + Clone + 'static> {
  async fn call(&self, ctx: C, req: CoreRequest) -> Result<CoreResponse, Error>;
}

pub struct App<C> { /* router, middleware, ctx */ }
impl<C: Clone + Send + Sync + 'static> App<C> {
  pub fn get(self, path: &str, h: impl Handler<C>) -> Self { /* ... */ }
  pub async fn handle(&self, req: CoreRequest) -> CoreResponse { /* ... */ }
}
```


### 抽出（例）
```rust
pub struct Path<T>(pub T);
pub struct Query<T>(pub T);
pub struct Json<T>(pub T);
```


### レスポンス（例）
```rust
pub trait IntoResponse { fn into_response(self) -> CoreResponse; }
// impl for String, &str, Bytes, (StatusCode, Json<T>) など
```

## 5. Context 抽象（環境依存のI/Oを分離）

```
#[async_trait::async_trait]
pub trait Kv { async fn get(&self, key: &str) -> Option<bytes::Bytes>;
               async fn put(&self, key: &str, val: bytes::Bytes); }

#[derive(Clone)]
pub struct Ctx {
  pub kv: Option<std::sync::Arc<dyn Kv>>,
  // clock(), secrets(), cache_api(), r2(), d1() など将来追加
}
```

- **Workers アダプタ**：env.kv("NAME") を包んだ Kv 実装を注入
- **hyper アダプタ**：メモリ実装 or Redis など外部サービスを注入

## 6. ミドルウェア

**目的**： ログ、エラー整形、CORS、ETag、レート制限など
**方式**： Middleware<C> で before/after を直列合成
**方針**： MVP は自作の最小実装のみ。後続ステップで tower 互換をオプショナルに追加

## 7. OpenAPI & 型安全クライアント

- **生成タイミング**： ルート登録と同時に収集、または tools/openapi-gen で静的に生成
- **ベース**： utoipa 互換のメタを最低限サポート（MVP はパス/メソッド/型）
- フロント側：
  - 型だけ：openapi-typescript
  - 型 + クライアント + React Query：orval
- 運用： CI で openapi.json → TS 生成 → 型ドリフト検知をビルドで落とす

## 8. セキュリティ / 性能（MVP 基準）

- **セキュリティ**：エラーメッセージの機密情報遮断、Content-Type 明示、JSONは UTF-8 固定
- **性能**：
  - ボディは最初 Bytes に固定（コピー最小）
  - ルータは matchit、ミドルウェアは最小
  - hyper 環境は hyper の既定最適化、Workers は Cache API/Keep-Alive を活用（後続ステップ）

## 9. 非目標（当面やらない）

- ORM・テンプレート・バンドラ・認証プロバイダ
- 大規模なコードジェネ・マクロ魔法
- フロント側の状態管理や UI コンポーネント

## 10. 実装ロードマップ（Step-by-Step）

> 目的：“最小で動く” → “どこでも動く” → “快適に使える” の順で育てる

### Phase 0: リポジトリ & 基盤
- Workspace 構成（core/, adapters/hyper, adapters/workers, examples/）
- CI（lint/format/test）／MSRV 固定／license/README

Exit 条件：cargo test が空でも通る。構成が確定。

### Phase 1: コア最小
- CoreRequest/Response、App、Handler、ルータ（matchit）
- IntoResponse（&str/String/Bytes/Json<T>）、Error（status() 付き）
- 抽出：Path<T> / Query<T> / Json<T>

Exit 条件：コアのユニットテストで 200/404/400 が返せる。

### Phase 2: hyper アダプタ（任意のサーバー環境向け）
- hyper サーバ → Core I/O 変換
- example: hello-hyper（GET /hello）

Exit 条件：curl localhost:8080/hello が “hello”。

### Phase 3: Workers アダプタ
- workers-rs（worker crate）で Fetch → Core I/O 変換
- KV の最小 Ctx 実装（get/put）
- example: hello-workers（wrangler dev で動作確認）

Exit 条件：ローカル dev と本番（1つの KV バインディング）で動く。

### Phase 4: ミドルウェア最小
- before/after 連鎖、ログ/エラー整形のサンプル
- ルートグループに適用する API

Exit 条件：ログ出力と 500 → JSON 変換が可能。

### Phase 5: OpenAPI（最小）
- ルート登録情報から Spec 構築（メソッド/パス/入出力型の名前）
- tools/openapi-gen or feature = "openapi" で /openapi.json を提供
- example: orval で TS クライアント生成

Exit 条件：openapi.json から TS 型/クライアントが生成され、example で動く。

### Phase 6: DX/周辺
- エラーレスポンスの標準形（problem+json 互換の簡易形）
- ルート毎の Cache-Control/ETag ヘルパ
- ドキュメント（Getting Started）

Exit 条件：サンプルアプリの増設（users/notes など）＋簡易ドキュメント公開。

### Phase 7+: 進化的拡張ポイント（順不同で漸進）

- SSE/ストリーミング 抽象（Stream<Item=Bytes> ↔ Workers ReadableStream / hyper Body）
- WebSocket（Workers WebSocketPair / hyper 環境 tokio_tungstenite）
- tower 互換レイヤ（オプション）
- WASI-HTTP / Fastly アダプタ
- tracing/OTel 連携、ベンチマーク
- CLI（xeno new, xeno dev, xeno openapi）

## 11. 例：ハンドラの書き味（イメージ）

```rust
// core/example
struct Hello;
#[async_trait::async_trait]
impl Handler<Ctx> for Hello {
  async fn call(&self, _ctx: Ctx, _req: CoreRequest) -> Result<CoreResponse, Error> {
    Ok("hello".into_response())
  }
}

pub fn app(ctx: Ctx) -> App<Ctx> {
  App::new(ctx)
    .get("/hello", Hello)
    .get("/users/:id", get_user)
}
```

## 12. テスト戦略

- コア：CoreRequest/Response だけでブラックボックステスト（ルータ・抽出・エラー）
- アダプタ：最小アプリに対し、hyper は reqwest、Workers は workerd ローカルで fetch テスト
- 契約：openapi.json を Snapshot テストし 型ドリフトを検知

## 13. バージョニング / 方針

- **SemVer 準拠**（0.x 期間は破壊変更あり）
- **最小依存の死守**：http/bytes/matchit/thiserror から不要に増やさない
- **移植性優先**：配備先固有の機能は Ctx を介して提供（コアに混ぜない）

## 14. 命名ガイド（暫定）

- crates：xeno-core, xeno-adapter-hyper, xeno-adapter-workers
- 型：CoreRequest / CoreResponse / Ctx / App / Handler
- feature：openapi, sse, ws, tower-compat など

## 採用しない判断（原則）

- “巨大なマクロDSL” は避け、学びやすい素直な API から始める
- “なんでも入り”にはしない（薄さが価値）
- プラットフォーム依存の最適化は アダプタ側で吸収

**Xeno は “薄いコア＋どこでも動く” を最短で形にするためのHonoにインスパイアされたフレームワークです**
