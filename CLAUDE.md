# CLAUDE.md — compiler-for-admrule

`legalize-pipeline`의 `.cache/admrule/*.xml`을 입력받아 bare Git 저장소(`admrule-kr`)를 직접 써내는 Rust 컴파일러입니다. 사용법·의도된 아키텍처는 `README.md`를 참고하세요. 이 문서는 Claude Code 에이전트가 주의해야 할 규약을 기록합니다.

## 현재 상태

**스캐폴드만.** 실제 변환 로직은 구현 전입니다. `src/main.rs`는 `unimplemented` CLI stub이며, `Cargo.toml`은 최소 의존성(`anyhow`, `clap`)만 포함합니다. 이 저장소가 존재하는 이유는 `9bow/admrule-kr` 데이터 저장소와 짝을 이루어 follow-up(F2 파이프라인 + 컴파일러 구현)에서 바로 개발을 시작할 수 있도록 하기 위함입니다.

구현 시작 시에는 자매 저장소 [`compiler-for-precedent`](https://github.com/legalize-kr/compiler-for-precedent)의 아키텍처를 1:1 미러하는 것을 원칙으로 합니다.

## 관련 저장소

| 저장소 | 관계 |
|---|---|
| `legalize-kr/legalize-pipeline` (예정 `admrules/` 패키지) | 참조 구현 (Python). converter가 생성하는 출력과 Rust 컴파일러 출력은 bit-identical이어야 함 |
| `9bow/admrule-kr` | 출력 대상 저장소 (bare repo → clone) |
| `legalize-kr/compiler-for-precedent` | 구조·컨벤션 미러 원천 |

## HARD INVARIANT (CRITICAL — 변경 불가)

1. **`.cache/admrule/*.xml`만 읽는다.** 외부 URL(법제처, 데이터포털 등)을 follow하지 않는다.
2. **바이너리(HWP/PDF/JPG/PNG)를 git tree·`target/`·tempdir 어디에도 write하지 않는다.** 별표·서식 파일은 frontmatter 메타데이터로만 보존한다.
3. **발령일자를 author/committer date로 사용한다.** `1970-01-01` 이전은 Git epoch로 clamp하고 frontmatter에 `epoch_clamped: true` + `발령일자_raw`를 함께 기록한다.
4. **commit author/committer**: `legalize-kr-bot <bot@legalize.kr>` (출력 저장소와 일치).
5. **동일 공포·발령일자 내 정렬**: `int(행정규칙일련번호) ASC` (문자열 lexical 금지 — 자리수 불균일).

위반 시 integration test가 fail-fast합니다.

## Python ↔ Rust 동등성 (구현 시 적용)

`legalize-pipeline/admrules/converter.py`(파이썬, 예정)와 `src/render.rs`(러스트)는 동일한 입력 XML에 대해 **동일한 파일 경로·동일한 Markdown 본문**을 생성해야 합니다. 이 동등성이 깨지면 Python 파이프라인이 만든 기존 `admrule-kr` 히스토리와 Rust 컴파일러가 만드는 새 저장소가 어긋나, 웹사이트·배포 스냅샷·이력 diff가 무너집니다.

변경을 가할 때는 **양쪽을 같이 고칩니다** (compiler-for-precedent CLAUDE.md §"Python ↔ Rust 동등성"과 동일 원칙).

## 개발

```bash
cargo test      # 단위·통합 테스트
cargo fmt
cargo clippy --no-deps -- -D warnings
cargo build --release
```

push 전 필수 로컬 검증 (워크스페이스 루트 `CLAUDE.md` §"Rust 저장소 push 전 검증" 규칙):

```bash
cargo test
cargo clippy --no-deps -- -D warnings
cargo shear        # (선택, shear 설치된 경우)
cargo fmt -- --check
```

## 커밋 규약

- 커밋 author/committer: 일반 개발자 (이 저장소는 bot이 push하지 않음 — 데이터 커밋은 출력 저장소 `admrule-kr`에서만 수행).
- 단일 로직 변경이 Python 재구현과 연동되면 commit message에 sibling 저장소 경로를 명시해 추적성을 남깁니다.
