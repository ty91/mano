# 아키텍처

## 제품 방향

Mano는 마크다운 본문에 노션 같은 속성·관계·데이터베이스 뷰를 결합하는 로컬 우선 노트 앱이다. 본문은 연속된 마크다운 텍스트로 유지하며, 저장 모델을 독립적인 편집 블록으로 나누지 않는다.

다음 구조는 초기 설계안이며 아직 구현되지 않았다. 우선 대상은 macOS와 iOS다.

## 구성

| 영역 | 방향 |
| --- | --- |
| Mac 앱 | GPUI 기반 UI와 마크다운 편집기 |
| iPhone 앱 | SwiftUI/UIKit 기반 UI, 노트 편집과 공통 뷰 지원 |
| 공통 코어 | Rust로 노트·속성·관계·조회·변경 규칙 구현 |
| 로컬 저장 | Markdown 원본, SQLite 검색·관계 인덱스 및 별도 동기화 상태 |
| 기기 동기화 | Swift의 CloudKit/CKSyncEngine 연동, 사용자별 private database |
| 에이전트 실행 | Mac에서 Claude Code와 Codex CLI 실행 |
| 에이전트의 앱 접근 | 앱 CLI와 stdio MCP 서버 |
| 동적 UI | 공통 선언형 뷰 정의와 Mac용 JavaScript 플러그인 |

## 모노레포 구조

실행·배포 단위는 `apps/`, 재사용 Rust 코드는 `crates/`, Apple 전용 연동은 `apple/`에 둔다. 아래는 목표 구조이며, 디렉터리와 패키지는 구현할 때 만든다.

```text
mano/
├── apps/
│   ├── macos/              # GPUI 데스크톱 앱
│   ├── ios/                # Swift 기반 iPhone 앱
│   ├── cli/                # 사람이거나 에이전트인 호출자를 위한 CLI
│   └── mcp/                # stdio MCP 어댑터
├── crates/
│   ├── core/               # 노트·속성·관계·뷰의 모델과 변경 규칙
│   ├── storage/            # Markdown, 인덱스, 변경 기록, 파일 감지
│   ├── agents/             # Claude/Codex 실행, 이벤트, 취소, 세션 관리
│   └── extensions/         # JS 런타임, 호스트 API, 플러그인 수명 관리
├── apple/
│   └── ManoCloudKit/       # Mac·iOS가 공유하는 Swift 동기화 패키지
├── schemas/                # 공통 뷰 정의와 플러그인 manifest의 규격
├── docs/
│   └── architecture.md
└── README.md
```

Rust 패키지는 루트 Cargo workspace로 묶고, Apple 패키지는 Swift Package Manager로 관리한다. JavaScript 플러그인 런타임이 있다는 이유만으로 Node/Bun workspace를 추가하지 않는다. 프런트엔드 빌드나 SDK 패키징이 필요해질 때 별도로 정한다.

`core`는 GPUI·Swift UI·에이전트 제공자에 의존하지 않는다. iOS에서도 공유할 Rust 코어와 Swift 사이의 FFI 경계는 명시적으로 두며, UI 코드를 공유한다는 전제는 두지 않는다.

## 데이터와 동기화

- 노트 본문과 속성의 원본은 `.md`와 frontmatter다. 관계는 파일 경로 대신 안정적인 노트 ID를 사용한다.
- 본문 안의 단순한 표는 Markdown 표로 저장한다. 여러 노트를 모아 보여주는 데이터베이스 뷰는 별도 정의로 저장한다.
- SQLite의 검색·관계 인덱스는 원본에서 재생성한다. 미전송 변경, 충돌 처리에 필요한 버전 정보, CKSyncEngine 상태는 재생성 가능한 인덱스와 구분해 영속 저장한다.
- UI·CLI·MCP·플러그인의 변경은 같은 코어 API를 거친다. Mac에서는 로컬 코어가 변경을 조정하고, CLI/MCP는 IPC로 연결한다. 외부 편집기의 파일 변경도 감지해 같은 처리 흐름에 반영한다.
- Swift 동기화 계층이 로컬 변경과 CloudKit 레코드를 연결한다. 계정 변경, 오프라인 재시도, 삭제와 동시 편집 충돌을 처리하며, 충돌 시 노트 내용을 조용히 덮어쓰지 않는다.
- CloudKit은 별도 서버 운영을 없애지만 동기화 로직을 대신 완성해주지는 않는다. CloudKit 저장 내용이 자동으로 iCloud Drive의 파일로 노출되는 것도 아니다.

## 에이전트와 확장

Mac의 실행 관리자가 `claude -p` 또는 `codex exec`를 시작하고, 구조화된 이벤트를 UI에 전달한다. 제공자별 차이는 어댑터로 감싼다. 로컬 실행은 도구와 프로세스의 위치를 뜻하며, 기본 Claude/Codex 구성의 모델 추론은 클라우드에서 이루어진다.

CLI와 MCP는 노트 검색·수정, 속성 변경, 뷰 생성, 플러그인 검사 같은 공통 기능을 노출한다. stdio MCP는 필요한 동안 실행되는 작은 프로세스로 두고 별도 HTTP 서버를 필수로 하지 않는다. 앱 창을 닫은 뒤에도 작업을 유지할지에 따라 로컬 코어의 프로세스 수명을 정한다.

동적 UI는 두 수준으로 제공한다.

1. **선언형 뷰:** 표·보드·폼·차트와 데이터 연결을 규격화한다. 에이전트가 정의를 만들고, Mac과 iPhone이 각자의 UI로 렌더링한다.
2. **Mac용 JS 플러그인:** Herdr의 mod처럼 스크립트가 패널과 동작을 구성하고 GPUI가 렌더링한다. 앱이 제공한 API 범위에서 확장하며, 웹 DOM이나 임의의 React 라이브러리 호환성을 전제하지 않는다.

플러그인에는 API 타입 선언과 검사 명령을 제공한다. 핫 리로드 실패 시 이전 버전을 유지하고, 실행 예산과 오류 격리, 노트 접근·네트워크·외부 명령 실행 권한을 정의한다. JS 런타임을 넣는 것만으로 보안 경계나 프로세스 격리가 생기지는 않는다.

## 플랫폼 경계와 주요 비용

- **Mac:** GPUI와 스크립트 확장의 조합을 우선한다. Herdr/gpui-shell은 참고 구현이며, 도입할 버전과 안정성은 별도 검증한다. 가장 큰 구현 비용은 한글 입력·선택·Undo·표 편집을 포함한 마크다운 편집기와 플러그인 API다.
- **iPhone:** 공통 데이터와 선언형 뷰를 지원한다. Mac용 CLI 실행이나 GPUI JS 플러그인의 그대로 실행을 전제하지 않는다. 실행 가능한 플러그인을 추가한다면 iOS 환경과 배포 규정을 별도로 검토한다.
- **원격 AI 요청:** iPhone에서 Mac 에이전트에 작업을 맡기는 기능은 문서 동기화와 별개다. Mac과 실행 프로세스가 동작 중이어야 하며, CloudKit은 작업을 실행해주는 서버가 아니다.
- **배포:** Mac은 로컬 도구 연동을 고려해 Developer ID 서명·공증을 통한 직접 배포를 우선 검토한다. 이 방식에서도 적절한 권한과 프로비저닝을 갖추면 CloudKit을 사용할 수 있다.

## 참고

- [Herdr mods](https://github.com/herdrdev/herdr-app/blob/master/docs/mods.md)
- [GPUI Kit](https://github.com/longbridge/gpui-kit)
- [CKSyncEngine](https://developer.apple.com/documentation/cloudkit/cksyncengine-4b4w9)
- [Developer ID와 CloudKit](https://developer.apple.com/developer-id/)
- [Claude Code 비대화형 실행](https://code.claude.com/docs/en/headless)
- [Codex SDK](https://github.com/openai/codex/tree/main/sdk/typescript)
- [MCP 전송 방식](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports)
