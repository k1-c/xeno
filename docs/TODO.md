# Xeno Web Framework - TODO List

## 📋 Phase 0: リポジトリ & 基盤
**目標**: ワークスペース雛形を push

- [x] Rust workspace 構成 (core/, adapters/hyper, adapters/workers, examples/)
- [x] 基本的な Cargo.toml 設定 (resolver = "2", workspace dependencies)
- [x] .gitignore ファイル作成
- [x] CI設定 (lint/format/test) - **TODO**
- [x] MSRV 固定 - **TODO**
- [x] LICENSE ファイル - **TODO**
- [x] README.md 更新 - **TODO**

**Exit 条件**: cargo test が空でも通る。構成が確定。

## 🎯 Phase 1: コア最小
**目標**: 基本的なWebフレームワーク機能の実装

- [x] CoreRequest/CoreResponse 型定義
- [x] App 構造体とルート登録
- [x] Handler トレイト定義
- [x] 基本ルータ実装（HashMap ベース）
- [x] IntoResponse トレイト実装
  - [x] &str, String, Bytes, StatusCode 対応
  - [x] Json<T> 対応
  - [x] (StatusCode, T) タプル対応
- [x] Error 型とステータスマッピング
- [x] 抽出機能の基本実装
  - [x] Path<T> - パスパラメータ抽出
  - [x] Query<T> - クエリパラメータ抽出
  - [x] Json<T> - JSONボディ抽出
- [ ] **TODO**: matchit を使った本格的なルーティング（パスパラメータ対応）
- [ ] **TODO**: ユニットテスト作成 (200/404/400 レスポンステスト)

**Exit 条件**: コアのユニットテストで 200/404/400 が返せる。

## 🌐 Phase 2: hyper アダプタ（ECS/VM）
**目標**: Hyper を使用したHTTPサーバー実装

- [x] HyperAdapter 構造体実装
- [x] hyper サーバ → Core I/O 変換
- [x] リクエスト/レスポンス変換機能
- [x] 並行処理対応（tokio::spawn）
- [x] hello-hyper サンプル実装
- [ ] **TODO**: エラーハンドリングの改善
- [ ] **TODO**: リクエストボディサイズ制限
- [ ] **TODO**: 統合テスト作成 (curl テスト)

**Exit 条件**: curl localhost:8080/hello が "Hello, World!" を返す。

## ☁️ Phase 3: Workers アダプタ
**目標**: Cloudflare Workers での動作実装

- [x] WorkersAdapter 基本構造作成（プレースホルダー）
- [x] KV 抽象実装（WorkersKv）
- [ ] **TODO**: worker クレート依存関係追加
- [ ] **TODO**: Fetch イベント → Core I/O 変換
- [ ] **TODO**: Workers 環境での Context 実装
- [ ] **TODO**: hello-workers の完全実装
- [ ] **TODO**: wrangler.toml 設定ファイル
- [ ] **TODO**: wrangler dev でのローカル開発環境構築

**Exit 条件**: ローカル dev と本番（1つの KV バインディング）で動く。

## 🔧 Phase 4: ミドルウェア最小
**目標**: before/after 処理チェーンの実装

- [x] Middleware トレイト定義
- [x] MiddlewareStack 基本実装
- [ ] **TODO**: ログ出力ミドルウェア実装
- [ ] **TODO**: エラー整形ミドルウェア実装
- [ ] **TODO**: ルートグループ適用 API
- [ ] **TODO**: CORS ミドルウェア（オプション）
- [ ] **TODO**: 実際のミドルウェア使用例

**Exit 条件**: ログ出力と 500 → JSON 変換が可能。

## 📘 Phase 5: OpenAPI（最小）
**目標**: API仕様書自動生成

- [x] openapi-gen ツール基本構造
- [x] プレースホルダー OpenAPI JSON 出力
- [ ] **TODO**: ルート情報収集機能
- [ ] **TODO**: 型情報からスキーマ生成
- [ ] **TODO**: derive マクロまたは手動アノテーション
- [ ] **TODO**: /openapi.json エンドポイント提供
- [ ] **TODO**: TypeScript 型定義生成 (openapi-typescript)
- [ ] **TODO**: orval によるクライアント生成サンプル

**Exit 条件**: openapi.json から TS 型/クライアントが生成され、example で動く。

## 🎨 Phase 6: DX/周辺
**目標**: 開発体験の向上

- [ ] **TODO**: エラーレスポンス標準形（problem+json 互換）
- [ ] **TODO**: Cache-Control/ETag ヘルパ関数
- [ ] **TODO**: Getting Started ドキュメント
- [ ] **TODO**: API リファレンス
- [ ] **TODO**: サンプルアプリ追加 (users/notes API など)
- [ ] **TODO**: パフォーマンステスト
- [ ] **TODO**: メモリ使用量測定

**Exit 条件**: サンプルアプリの増設＋簡易ドキュメント公開。

## 🚀 Phase 7+: 進化的拡張ポイント
**目標**: より高度な機能実装

### ストリーミング & WebSocket
- [ ] **TODO**: SSE（Server-Sent Events）抽象
- [ ] **TODO**: WebSocket サポート
- [ ] **TODO**: Stream<Item=Bytes> ↔ Workers ReadableStream 変換
- [ ] **TODO**: hyper Body ストリーミング対応

### 互換性 & エコシステム
- [ ] **TODO**: tower 互換レイヤ（オプション）
- [ ] **TODO**: WASI-HTTP アダプタ
- [ ] **TODO**: Fastly Compute@Edge アダプタ
- [ ] **TODO**: 既存ミドルウェアとの互換性

### 監視 & 運用
- [ ] **TODO**: tracing/OpenTelemetry 連携
- [ ] **TODO**: メトリクス収集
- [ ] **TODO**: ヘルスチェック標準化
- [ ] **TODO**: graceful shutdown

### 開発ツール
- [ ] **TODO**: CLI ツール (xeno new, xeno dev, xeno openapi)
- [ ] **TODO**: ホットリロード開発環境
- [ ] **TODO**: デバッグ用ミドルウェア

### テスト & 品質
- [ ] **TODO**: 統合テストスイート
- [ ] **TODO**: ベンチマークスイート
- [ ] **TODO**: セキュリティ監査
- [ ] **TODO**: メモリリーク検出

## 🐛 現在の既知の課題

- [ ] **FIXME**: Router でのパスパラメータが正しく抽出されない（HashMap 固定値）
- [ ] **FIXME**: Workers adapter の app フィールドが未使用警告
- [ ] **FIXME**: エラーハンドリングでの情報漏洩防止
- [ ] **FIXME**: Hyper adapter でのボディサイズ制限なし

## 📝 メモ & アイデア

- **設計方針**: 薄いコア + どこでも動く を維持
- **依存関係**: 最小限に抑制（http/bytes/matchit/thiserror/async-trait）
- **移植性**: プラットフォーム固有機能は Context で抽象化
- **型安全性**: コンパイル時型チェックを最大活用
- **パフォーマンス**: ゼロコスト抽象を意識した設計

---

**最終更新**: 2025-08-18  
**現在のフェーズ**: Phase 1 完了, Phase 2 ほぼ完了